//! nom parser for .fnv files.
//!
//! Layout (all little endian):
//! ```text
//! header   u32 curve type | u32 version (0-3) | u32 point count
//! points   point count * 12 / 16 / 24 bytes (version 0 / 1 / 2-3), see `Point`
//! footer   20 bytes common to all types (`CommonFooter`)
//!          + 16 bytes for envelopes (`EnvParams`)
//!          + 20 bytes for LFOs (`LfoParams`), 16 before version 3
//! ```
use crate::curve::{
    CommonFooter, CurveType, EnvParams, Fnv, FooterParams, LfoParams, Version, HEADER_LEN,
};
use crate::point::{ArpMode, Point, PointMode};
use nom::{
    error::{ErrorKind, ParseError},
    multi::count,
    number::complete::{le_f32, le_f64, le_i32, le_i8, le_u32, le_u8},
    IResult, Parser,
};
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum FnvReadErrorKind {
    InvalidCurveType(u32),
    /// Newer than version 3 (FL refuses these too)
    UnsupportedVersion(u32),
    /// File size doesn't match `12 + point length * point_count + footer length`
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
    let (input, version) = version(input)?;
    let (input, point_count) = le_u32(input)?;

    // checking the size up front also stops a bogus point count from being used
    let expected = HEADER_LEN as u64
        + version.point_len() as u64 * point_count as u64
        + curve_type.footer_len(version) as u64;
    if expected != whole.len() as u64 {
        return fail(
            &whole[8..],
            FnvReadErrorKind::SizeMismatch {
                expected,
                found: whole.len(),
            },
        );
    }

    let point = match version {
        Version::V0 => point_v0,
        Version::V1 => point_v1,
        Version::V2 | Version::V3 => point_v3,
    };
    let (input, points) = count(point, point_count as usize).parse(input)?;
    let (input, footer) = common_footer(input)?;
    let (input, params) = match curve_type {
        CurveType::Envelope => {
            let (i, p) = env_params(input)?;
            (i, FooterParams::Envelope(p))
        }
        CurveType::Lfo => {
            let (i, p) = lfo_params(input, version)?;
            (i, FooterParams::Lfo(p))
        }
        CurveType::Graph | CurveType::Map => (input, FooterParams::None),
    };
    Ok((
        input,
        Fnv {
            curve_type,
            version,
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

fn version(input: &[u8]) -> PResult<'_, Version> {
    let (rest, n) = le_u32(input)?;
    match Version::try_from(n) {
        Ok(v) => Ok((rest, v)),
        Err(n) => fail(input, FnvReadErrorKind::UnsupportedVersion(n)),
    }
}

/// Version 0: 12 bytes, three f32 and nothing else.
fn point_v0(input: &[u8]) -> PResult<'_, Point> {
    let (input, x_offset) = le_f32(input)?;
    let (input, y) = le_f32(input)?;
    let (input, tension) = le_f32(input)?;
    Ok((
        input,
        Point {
            x_offset: x_offset as f64,
            y: y as f64,
            tension,
            ..Point::default()
        },
    ))
}

/// Version 1: 16 bytes, f32 coordinates.
fn point_v1(input: &[u8]) -> PResult<'_, Point> {
    let (input, x_offset) = le_f32(input)?;
    let (input, y) = le_f32(input)?;
    point_tail(input, x_offset as f64, y as f64)
}

/// Versions 2 and 3: 24 bytes, f64 coordinates.
fn point_v3(input: &[u8]) -> PResult<'_, Point> {
    let (input, x_offset) = le_f64(input)?;
    let (input, y) = le_f64(input)?;
    point_tail(input, x_offset, y)
}

/// The 8 bytes after the coordinates, shared by versions 1-3.
fn point_tail(input: &[u8], x_offset: f64, y: f64) -> PResult<'_, Point> {
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

fn lfo_params(input: &[u8], version: Version) -> PResult<'_, LfoParams> {
    let (input, speed) = le_u32(input)?;
    let (input, tension) = le_i32(input)?;
    let (input, skew) = le_i32(input)?;
    let (input, pulse_width) = le_i32(input)?;
    let (input, phase) = if version.has_lfo_phase() {
        le_u32(input)?
    } else {
        (input, 0)
    };
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

    /// version 0, shipped with FL: "Data/Patches/Envelopes/Maps/Default.fnv" (a graph)
    const GRAPH_V0: &str = "03 00 00 00 00 00 00 00 02 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 80
        00 00 80 3f 00 00 80 3f 00 00 00 00
        00 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff";

    /// version 0, shipped with FL: Harmor "Data/LFO/Default.fnv"
    const LFO_V0: &str = "02 00 00 00 00 00 00 00 04 00 00 00
        00 00 00 00 00 00 00 3f 00 00 00 00
        00 00 80 3f 00 00 00 3f 00 00 00 00
        00 00 80 3e 00 00 80 3f 00 00 00 00
        00 00 80 3f 00 00 00 3f 00 00 00 00
        00 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff 02 00 00 00
        40 9c 00 00 00 00 00 00 00 00 00 00 00 00 00 00";

    /// version 2, shipped with FL: Harmor "Data/LFO/Pitch vibrato.fnv" (no phase)
    const LFO_V2: &str = "02 00 00 00 02 00 00 00 01 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 a0 aa aa e0 3f 00 00 00 00 00 00 00 00
        02 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff
        95 d1 00 00 00 00 00 00 00 00 00 00 00 00 00 00";

    /// version 1 (2019): "second dword testing/Arp - 1 1 2 3.fnv"
    const ENV_V1: &str = "01 00 00 00 01 00 00 00 09 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 00 02 00 00
        00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00
        00 00 00 3f 00 00 00 00 00 00 00 00 00 02 00 00
        00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00
        00 00 00 3f 00 00 00 00 00 00 00 00 00 03 00 00
        00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00
        00 00 00 3f 00 00 00 00 00 00 00 00 00 03 00 00
        00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00
        00 00 00 3f 00 00 00 00 00 00 00 00 00 03 00 00
        03 00 00 00 ff ff ff ff ff ff ff ff 00 00 00 00 08 00 00 00
        80 00 00 00 40 00 00 00 00 00 00 00 80 00 00 00";

    /// FL's version 3 resave of `ENV_V1`: "Arp - 1 1 2 3 - resave.fnv"
    const ENV_V1_RESAVED: &str = "01 00 00 00 03 00 00 00 09 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 02 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 00 00 00 00 00 00 02 00 02
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 00 00 00 00 00 00 03 00 02
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 00 00 00 00 00 00 03 00 02
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 f0 3f 00 00 00 00 00 00 00 02
        00 00 00 00 00 00 e0 3f 00 00 00 00 00 00 00 00 00 00 00 00 00 03 00 02
        03 00 00 00 ff ff ff ff ff ff ff ff 00 00 00 00 08 00 00 00
        80 00 00 00 40 00 00 00 00 00 00 00 80 00 00 00";

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

        let mut b = hex(GRAPH_3);
        b[4] = 4;
        assert_eq!(
            read_fnv(&b).unwrap_err(),
            FnvReadError {
                offset: 4,
                kind: FnvReadErrorKind::UnsupportedVersion(4)
            }
        );

        assert!(read_fnv(&[]).is_err());
    }

    #[test]
    fn roundtrip() {
        for h in [
            GRAPH_3,
            ENV_DEFAULT,
            LFO_DEFAULT,
            GRAPH_V0,
            LFO_V0,
            LFO_V2,
            ENV_V1,
            ENV_V1_RESAVED,
        ] {
            let b = hex(h);
            assert_eq!(read_fnv(&b).unwrap().to_bytes(), b);
        }
    }

    #[test]
    fn graph_v0() {
        let f = read_fnv(&hex(GRAPH_V0)).unwrap();
        assert_eq!(f.version, Version::V0);
        assert_eq!(f.curve_type, CurveType::Graph);
        assert_eq!(f.absolute_xs(), vec![0.0, 1.0]);
        assert_eq!(f.points[1].y, 1.0);
        // stored as -0.0; must survive the round trip
        assert!(f.points[0].tension == 0.0 && f.points[0].tension.is_sign_negative());
        assert_eq!(f.points[1].mode, PointMode::SingleCurve);
    }

    /// The version 0 LFO shipped with FL is the same curve as today's default LFO.
    #[test]
    fn lfo_v0_matches_default_lfo() {
        let old = read_fnv(&hex(LFO_V0)).unwrap();
        let new = read_fnv(&hex(LFO_DEFAULT)).unwrap();
        assert_eq!(old.version, Version::V0);
        let coords = |f: &Fnv| -> Vec<(f64, f64, f32)> {
            f.points.iter().map(|p| (p.x_offset, p.y, p.tension)).collect()
        };
        assert_eq!(coords(&old), coords(&new));
        assert_eq!(old.footer, new.footer);
        assert_eq!(old.params, new.params);
    }

    #[test]
    fn lfo_v2_has_no_phase() {
        let f = read_fnv(&hex(LFO_V2)).unwrap();
        assert_eq!(f.version, Version::V2);
        assert_eq!(f.points.len(), 1);
        assert_eq!(f.points[0].y, 0.5208333134651184);
        assert!(f.footer.global());
        assert_eq!(
            f.params,
            FooterParams::Lfo(LfoParams {
                speed: 53653,
                tension: 0,
                skew: 0,
                pulse_width: 0,
                phase: 0,
            })
        );
    }

    /// FL converted the version 1 file to version 3 without changing the curve;
    /// only the (then missing) tension sign byte was filled in.
    #[test]
    fn env_v1_matches_its_v3_resave() {
        let old = read_fnv(&hex(ENV_V1)).unwrap();
        let new = read_fnv(&hex(ENV_V1_RESAVED)).unwrap();
        assert_eq!(old.version, Version::V1);
        assert_eq!(new.version, Version::V3);
        assert_eq!(old.points.len(), 9);
        for (a, b) in old.points.iter().zip(&new.points) {
            assert_eq!(
                (a.x_offset, a.y, a.tension, a.mode, a.arp_mode),
                (b.x_offset, b.y, b.tension, b.mode, b.arp_mode)
            );
        }
        let arps: Vec<ArpMode> = old.points.iter().map(|p| p.arp_mode).collect();
        assert_eq!(arps[..4], [ArpMode::Same, ArpMode::None, ArpMode::Same, ArpMode::None]);
        assert_eq!(old.footer, new.footer);
        assert_eq!(old.params, new.params);
    }

    /// Converting between versions keeps the curve (for values f32 can hold).
    #[test]
    fn convert_versions() {
        let orig = read_fnv(&hex(ENV_V1)).unwrap();
        let mut f = orig.clone();
        f.version = Version::V3;
        let mut v3 = read_fnv(&f.to_bytes()).unwrap();
        assert_eq!(v3.points, orig.points);
        v3.version = Version::V1;
        assert_eq!(v3.to_bytes(), hex(ENV_V1));
    }
}
