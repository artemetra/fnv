//! nom parser for .fnv files.
//!
//! Layout (all little endian):
//! ```text
//! header   u32 curve type | u32 version (3) | u32 point count
//! points   point count * 24 bytes, see `Point`
//! footer   20 bytes common to all types (`CommonFooter`)
//!          + 16 bytes for envelopes (`EnvParams`) / 20 bytes for LFOs (`LfoParams`)
//! ```
use crate::curve::{
    CommonFooter, CurveType, EnvParams, Fnv, FooterParams, LfoParams, FNV_VERSION, HEADER_LEN,
    POINT_LEN,
};
use crate::point::{ArpMode, Point, PointMode};
use nom::{
    error::{ErrorKind, ParseError},
    multi::count,
    number::complete::{le_f32, le_f64, le_i32, le_i8, le_u32, le_u8},
    IResult,
};
use std::convert::TryFrom;
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum FnvReadErrorKind {
    InvalidCurveType(u32),
    /// Only version 3 has been seen in files saved by FL
    UnsupportedVersion(u32),
    /// File size doesn't match `12 + 24 * point_count + footer length`
    SizeMismatch { expected: u64, found: usize },
    InvalidPointMode(u8),
    InvalidArpMode(u8),
    /// A LongBool that is neither -1 nor 0
    InvalidBool(i32),
    /// A point index that is negative but not -1
    InvalidPointIndex(i32),
    Nom(ErrorKind),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnvReadError {
    /// byte offset into the file
    pub offset: usize,
    pub kind: FnvReadErrorKind,
}

impl fmt::Display for FnvReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} at offset {:#x}", self.kind, self.offset)
    }
}

impl Error for FnvReadError {}

/// Internal nom error; keeps the remaining input so the offset can be computed later.
#[derive(Debug)]
struct PError<'a> {
    input: &'a [u8],
    kind: FnvReadErrorKind,
}

impl<'a> ParseError<&'a [u8]> for PError<'a> {
    fn from_error_kind(input: &'a [u8], kind: ErrorKind) -> Self {
        PError {
            input,
            kind: FnvReadErrorKind::Nom(kind),
        }
    }
    fn append(_: &'a [u8], _: ErrorKind, other: Self) -> Self {
        other
    }
}

type PResult<'a, T> = IResult<&'a [u8], T, PError<'a>>;

fn fail<T>(input: &[u8], kind: FnvReadErrorKind) -> PResult<'_, T> {
    Err(nom::Err::Failure(PError { input, kind }))
}

/// Parses a whole .fnv file. Trailing or missing bytes are an error.
pub fn read_fnv(bytes: &[u8]) -> Result<Fnv, FnvReadError> {
    match fnv(bytes) {
        Ok((_, fnv)) => Ok(fnv),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(FnvReadError {
            offset: bytes.len() - e.input.len(),
            kind: e.kind,
        }),
        Err(nom::Err::Incomplete(_)) => unreachable!("only complete parsers are used"),
    }
}

fn fnv(input: &[u8]) -> PResult<'_, Fnv> {
    let whole = input;
    let (input, curve_type) = curve_type(input)?;
    let (input, version) = le_u32(input)?;
    if version != FNV_VERSION {
        return fail(&whole[4..], FnvReadErrorKind::UnsupportedVersion(version));
    }
    let (input, point_count) = le_u32(input)?;

    // checking the size up front also stops a bogus point count from being used
    let expected =
        HEADER_LEN as u64 + POINT_LEN as u64 * point_count as u64 + curve_type.footer_len() as u64;
    if expected != whole.len() as u64 {
        return fail(
            &whole[8..],
            FnvReadErrorKind::SizeMismatch {
                expected,
                found: whole.len(),
            },
        );
    }

    let (input, points) = count(point, point_count as usize)(input)?;
    let (input, footer) = common_footer(input)?;
    let (input, params) = match curve_type {
        CurveType::Envelope => {
            let (i, p) = env_params(input)?;
            (i, FooterParams::Envelope(p))
        }
        CurveType::Lfo => {
            let (i, p) = lfo_params(input)?;
            (i, FooterParams::Lfo(p))
        }
        CurveType::Graph | CurveType::Map => (input, FooterParams::None),
    };
    Ok((
        input,
        Fnv {
            curve_type,
            points,
            footer,
            params,
        },
    ))
}

fn curve_type(input: &[u8]) -> PResult<'_, CurveType> {
    let (rest, n) = le_u32(input)?;
    match CurveType::try_from(n) {
        Ok(t) => Ok((rest, t)),
        Err(n) => fail(input, FnvReadErrorKind::InvalidCurveType(n)),
    }
}

fn point(input: &[u8]) -> PResult<'_, Point> {
    let (input, x_offset) = le_f64(input)?;
    let (input, y) = le_f64(input)?;
    let (input, tension) = le_f32(input)?;
    let (input, mode) = point_mode(input)?;
    let (input, arp_mode) = arp_mode(input)?;
    let (input, reserved) = le_u8(input)?;
    let (input, tension_sign) = le_i8(input)?;
    Ok((
        input,
        Point {
            x_offset,
            y,
            tension,
            mode,
            arp_mode,
            reserved,
            tension_sign,
        },
    ))
}

fn point_mode(input: &[u8]) -> PResult<'_, PointMode> {
    let (rest, n) = le_u8(input)?;
    match PointMode::try_from(n) {
        Ok(m) => Ok((rest, m)),
        Err(n) => fail(input, FnvReadErrorKind::InvalidPointMode(n)),
    }
}

fn arp_mode(input: &[u8]) -> PResult<'_, ArpMode> {
    let (rest, n) = le_u8(input)?;
    match ArpMode::try_from(n) {
        Ok(m) => Ok((rest, m)),
        Err(n) => fail(input, FnvReadErrorKind::InvalidArpMode(n)),
    }
}

fn long_bool(input: &[u8]) -> PResult<'_, bool> {
    let (rest, n) = le_i32(input)?;
    match n {
        -1 => Ok((rest, true)),
        0 => Ok((rest, false)),
        n => fail(input, FnvReadErrorKind::InvalidBool(n)),
    }
}

/// -1 means no point
fn point_index(input: &[u8]) -> PResult<'_, Option<u32>> {
    let (rest, n) = le_i32(input)?;
    match n {
        -1 => Ok((rest, None)),
        n if n >= 0 => Ok((rest, Some(n as u32))),
        n => fail(input, FnvReadErrorKind::InvalidPointIndex(n)),
    }
}

fn common_footer(input: &[u8]) -> PResult<'_, CommonFooter> {
    let (input, flags) = le_u32(input)?;
    let (input, enabled) = long_bool(input)?;
    let (input, decay_point) = point_index(input)?;
    let (input, loop_start_point) = point_index(input)?;
    let (input, sustain_point) = point_index(input)?;
    Ok((
        input,
        CommonFooter {
            flags,
            enabled,
            decay_point,
            loop_start_point,
            sustain_point,
        },
    ))
}

fn env_params(input: &[u8]) -> PResult<'_, EnvParams> {
    let (input, attack) = le_i32(input)?;
    let (input, decay) = le_i32(input)?;
    let (input, sustain) = le_i32(input)?;
    let (input, release) = le_i32(input)?;
    Ok((
        input,
        EnvParams {
            attack,
            decay,
            sustain,
            release,
        },
    ))
}

fn lfo_params(input: &[u8]) -> PResult<'_, LfoParams> {
    let (input, speed) = le_u32(input)?;
    let (input, tension) = le_i32(input)?;
    let (input, skew) = le_i32(input)?;
    let (input, pulse_width) = le_i32(input)?;
    let (input, phase) = le_u32(input)?;
    Ok((
        input,
        LfoParams {
            speed,
            tension,
            skew,
            pulse_width,
            phase,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        s.split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect()
    }

    /// "3 points - default.fnv"
    const GRAPH_3: &str = "03 00 00 00 03 00 00 00 03 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
        00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff";

    /// "default envelope.fnv"
    const ENV_DEFAULT: &str = "01 00 00 00 03 00 00 00 04 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
        00 00 00 00 00 00 e8 3f 00 00 00 00 00 00 f0 3f cd cc 4c 3e 00 00 00 01
        00 00 00 00 00 00 f8 3f 00 00 00 00 00 00 e0 3f cd cc 4c 3e 00 00 00 01
        00 00 00 00 00 00 d0 3f 00 00 00 00 00 00 00 00 cd cc 4c 3e 00 00 00 01
        00 00 00 00 00 00 00 00 01 00 00 00 ff ff ff ff 02 00 00 00
        80 00 00 00 80 00 00 00 00 00 00 00 80 00 00 00";

    /// "default lfo.fnv"
    const LFO_DEFAULT: &str = "02 00 00 00 03 00 00 00 04 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 00
        00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 d0 3f 00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff 02 00 00 00
        40 9c 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";

    #[test]
    fn graph() {
        let f = read_fnv(&hex(GRAPH_3)).unwrap();
        assert_eq!(f.curve_type, CurveType::Graph);
        assert_eq!(f.points.len(), 3);
        assert_eq!(f.absolute_xs(), vec![0.0, 0.5, 1.0]);
        let ys: Vec<f64> = f.points.iter().map(|p| p.y).collect();
        assert_eq!(ys, vec![0.0, 0.5, 1.0]);
        assert_eq!(f.points[1].mode, PointMode::SingleCurve);
        assert_eq!(f.points[1].tension_sign, 2);
        assert!(f.footer.enabled);
        assert_eq!(f.footer.sustain_point, None);
        assert_eq!(f.params, FooterParams::None);
    }

    #[test]
    fn envelope() {
        let f = read_fnv(&hex(ENV_DEFAULT)).unwrap();
        assert_eq!(f.curve_type, CurveType::Envelope);
        assert_eq!(f.absolute_xs(), vec![0.0, 0.75, 2.25, 2.5]);
        assert_eq!(f.points[1].y, 1.0);
        assert_eq!(f.points[1].tension, 0.2);
        assert_eq!(f.points[1].tension_sign, 1);
        assert!(!f.footer.enabled);
        assert_eq!(f.footer.decay_point, Some(1));
        assert_eq!(f.footer.loop_start_point, None);
        assert_eq!(f.footer.sustain_point, Some(2));
        assert_eq!(
            f.params,
            FooterParams::Envelope(EnvParams {
                attack: 128,
                decay: 128,
                sustain: 0,
                release: 128,
            })
        );
    }

    #[test]
    fn lfo() {
        let f = read_fnv(&hex(LFO_DEFAULT)).unwrap();
        assert_eq!(f.curve_type, CurveType::Lfo);
        assert_eq!(f.points[0].y, 0.5);
        assert_eq!(f.absolute_xs(), vec![0.0, 1.0, 1.25, 2.25]);
        assert!(f.footer.enabled);
        assert_eq!(f.footer.sustain_point, Some(2));
        match f.params {
            FooterParams::Lfo(p) => {
                assert_eq!(p.speed, 40000);
                assert_eq!(p.tension, 0);
            }
            p => panic!("wrong params {:?}", p),
        }
    }

    #[test]
    fn errors() {
        let mut b = hex(GRAPH_3);
        b[0] = 5;
        assert_eq!(
            read_fnv(&b).unwrap_err(),
            FnvReadError {
                offset: 0,
                kind: FnvReadErrorKind::InvalidCurveType(5)
            }
        );

        let mut b = hex(GRAPH_3);
        b.push(0);
        assert!(matches!(
            read_fnv(&b).unwrap_err().kind,
            FnvReadErrorKind::SizeMismatch {
                expected: 104,
                found: 105
            }
        ));

        let mut b = hex(GRAPH_3);
        b[12 + 24 + 20] = 0x0D; // mode of the 2nd point
        assert_eq!(
            read_fnv(&b).unwrap_err(),
            FnvReadError {
                offset: 56,
                kind: FnvReadErrorKind::InvalidPointMode(0x0D)
            }
        );

        assert!(read_fnv(&[]).is_err());
    }

    #[test]
    fn roundtrip() {
        for h in [GRAPH_3, ENV_DEFAULT, LFO_DEFAULT] {
            let b = hex(h);
            assert_eq!(read_fnv(&b).unwrap().to_bytes(), b);
        }
    }
}
