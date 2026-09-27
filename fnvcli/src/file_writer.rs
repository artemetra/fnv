//! Serializes an `Fnv` back into the on-disk format (inverse of `file_reader`).
use crate::curve::{CommonFooter, CurveType, Fnv, FooterParams, Version, HEADER_LEN};
use crate::point::Point;
use std::{error::Error, fmt};

/// An `Fnv` that can't be written as a valid file.
#[derive(Debug, Clone, PartialEq)]
pub enum FnvWriteError {
    /// Envelopes need `FooterParams::Envelope`, LFOs `FooterParams::Lfo`,
    /// graphs and EQs `FooterParams::None`.
    ParamsMismatch { curve_type: CurveType },
    /// Point indices are stored as i32
    PointIndexTooLarge(u32),
}

impl fmt::Display for FnvWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FnvWriteError::ParamsMismatch { curve_type } => {
                let expected = match curve_type {
                    CurveType::Envelope => "Envelope",
                    CurveType::Lfo => "Lfo",
                    CurveType::Graph | CurveType::Map => "None",
                };
                write!(f, "{:?} curves need params {}", curve_type, expected)
            }
            FnvWriteError::PointIndexTooLarge(i) => {
                write!(f, "point index {} doesn't fit in an i32", i)
            }
        }
    }
}

impl Error for FnvWriteError {}

impl Fnv {
    /// Checks that `to_bytes` will produce a file that can be read back.
    pub fn check(&self) -> Result<(), FnvWriteError> {
        let params_ok = matches!(
            (self.curve_type, &self.params),
            (CurveType::Envelope, FooterParams::Envelope(_))
                | (CurveType::Lfo, FooterParams::Lfo(_))
                | (CurveType::Graph | CurveType::Map, FooterParams::None)
        );
        if !params_ok {
            return Err(FnvWriteError::ParamsMismatch {
                curve_type: self.curve_type,
            });
        }
        let f = &self.footer;
        for i in [f.decay_point, f.loop_start_point, f.sustain_point]
            .into_iter()
            .flatten()
        {
            if i > i32::MAX as u32 {
                return Err(FnvWriteError::PointIndexTooLarge(i));
            }
        }
        Ok(())
    }

    /// Writes the file in `self.version`. Older versions store less, so
    /// writing them drops data: version 0 has no mode / arp mode / flags per
    /// point, versions 0 and 1 round coordinates to f32, and versions 0-2
    /// have no LFO phase. Files read from disk always write back identically.
    /// Call `check` first for an `Fnv` that didn't come from `read_fnv`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let v = self.version;
        let mut out = Vec::with_capacity(
            HEADER_LEN + v.point_len() * self.points.len() + self.curve_type.footer_len(v),
        );
        out.extend((self.curve_type as u32).to_le_bytes());
        out.extend((v as u32).to_le_bytes());
        out.extend((self.points.len() as u32).to_le_bytes());
        for p in &self.points {
            write_point(&mut out, p, v);
        }
        write_common_footer(&mut out, &self.footer);
        match &self.params {
            FooterParams::None => {}
            FooterParams::Envelope(e) => {
                out.extend(e.attack.to_le_bytes());
                out.extend(e.decay.to_le_bytes());
                out.extend(e.sustain.to_le_bytes());
                out.extend(e.release.to_le_bytes());
            }
            FooterParams::Lfo(l) => {
                out.extend(l.speed.to_le_bytes());
                out.extend(l.tension.to_le_bytes());
                out.extend(l.skew.to_le_bytes());
                out.extend(l.pulse_width.to_le_bytes());
                if v.has_lfo_phase() {
                    out.extend(l.phase.to_le_bytes());
                }
            }
        }
        out
    }
}

fn write_point(out: &mut Vec<u8>, p: &Point, version: Version) {
    match version {
        Version::V0 | Version::V1 => {
            out.extend((p.x_offset as f32).to_le_bytes());
            out.extend((p.y as f32).to_le_bytes());
        }
        Version::V2 | Version::V3 => {
            out.extend(p.x_offset.to_le_bytes());
            out.extend(p.y.to_le_bytes());
        }
    }
    out.extend(p.tension.to_le_bytes());
    if version == Version::V0 {
        return;
    }
    out.push(p.mode as u8);
    out.push(p.arp_mode as u8);
    out.push(p.reserved);
    out.extend(p.tension_sign.to_le_bytes());
}

fn write_common_footer(out: &mut Vec<u8>, f: &CommonFooter) {
    let index = |i: Option<u32>| i.map_or(-1, |i| i as i32);
    out.extend(f.flags.to_le_bytes());
    out.extend((if f.enabled { -1i32 } else { 0 }).to_le_bytes());
    out.extend(index(f.decay_point).to_le_bytes());
    out.extend(index(f.loop_start_point).to_le_bytes());
    out.extend(index(f.sustain_point).to_le_bytes());
}
