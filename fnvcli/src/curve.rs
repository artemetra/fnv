use crate::point::Point;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The only version seen in files saved by FL (u32 at offset 0x04).
pub const FNV_VERSION: u32 = 3;
pub const HEADER_LEN: usize = 12;
pub const POINT_LEN: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurveType {
    Envelope = 0x01,
    Lfo = 0x02,
    Graph = 0x03,
    /// EQ (old name: Map)
    Map = 0x07,
}

impl TryFrom<u32> for CurveType {
    type Error = u32;
    fn try_from(n: u32) -> Result<Self, u32> {
        match n {
            1 => Ok(CurveType::Envelope),
            2 => Ok(CurveType::Lfo),
            3 => Ok(CurveType::Graph),
            7 => Ok(CurveType::Map),
            n => Err(n),
        }
    }
}

impl CurveType {
    /// Length of the whole footer (common part + type-specific part) in bytes.
    pub fn footer_len(self) -> usize {
        CommonFooter::LEN
            + match self {
                CurveType::Envelope => EnvParams::LEN,
                CurveType::Lfo => LfoParams::LEN,
                CurveType::Graph | CurveType::Map => 0,
            }
    }
}

/// First 20 bytes of the footer, present in every curve type.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommonFooter {
    /// Bitfield, see the accessor methods.
    pub flags: u32,
    /// Stored as a Delphi LongBool (-1 = on, 0 = off). Always on for graphs.
    pub enabled: bool,
    /// Point indices of the ADSR markers, `None` is stored as -1.
    pub decay_point: Option<u32>,
    pub loop_start_point: Option<u32>,
    /// "Sustain / Loop end" in FL
    pub sustain_point: Option<u32>,
}

impl CommonFooter {
    pub const LEN: usize = 20;

    pub fn tempo(&self) -> bool {
        self.flags & 0b0001 != 0
    }
    pub fn global(&self) -> bool {
        self.flags & 0b0010 != 0
    }
    /// LFO only. The bit is set when the LFO is *uni*polar.
    pub fn bipolar(&self) -> bool {
        self.flags & 0b0100 == 0
    }
    /// LFO only.
    pub fn frozen(&self) -> bool {
        self.flags & 0b1000 != 0
    }
}

/// Extra 16 footer bytes of envelopes: the ADSR knobs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvParams {
    /// 0..=256, default 128
    pub attack: i32,
    /// 0..=256, default 128
    pub decay: i32,
    /// -128..=128, default 0 (knob centred)
    pub sustain: i32,
    /// 0..=256, default 128
    pub release: i32,
}

impl Default for EnvParams {
    fn default() -> Self {
        EnvParams {
            attack: 128,
            decay: 128,
            sustain: 0,
            release: 128,
        }
    }
}

impl EnvParams {
    pub const LEN: usize = 16;
}

/// Extra 20 footer bytes of LFOs.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LfoParams {
    /// knob 0.0..=1.0 maps linearly to 512..=65536
    pub speed: u32,
    /// -128..=128
    pub tension: i32,
    /// assumed from knob order, never non-zero in the test corpus
    pub skew: i32,
    /// assumed from knob order, never non-zero in the test corpus
    pub pulse_width: i32,
    /// raw value, scale unknown
    pub phase: u32,
}

impl LfoParams {
    pub const LEN: usize = 20;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FooterParams {
    None,
    Envelope(EnvParams),
    Lfo(LfoParams),
}

/// A parsed .fnv file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fnv {
    pub curve_type: CurveType,
    pub points: Vec<Point>,
    pub footer: CommonFooter,
    /// Must match `curve_type`: `Envelope` for envelopes, `Lfo` for LFOs, `None` otherwise.
    pub params: FooterParams,
}

impl Fnv {
    /// Absolute x coordinate of every point.
    pub fn absolute_xs(&self) -> Vec<f64> {
        self.points
            .iter()
            .scan(0.0, |x, p| {
                *x += p.x_offset;
                Some(*x)
            })
            .collect()
    }
}
