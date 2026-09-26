use crate::point::Point;
use serde::{Deserialize, Serialize};

pub const HEADER_LEN: usize = 12;

/// Format version (u32 at offset 0x04). Current FL writes `V3`, but most
/// files shipped with FL are older. FL refuses anything newer than 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Version {
    /// 12-byte points: f32 x_offset, y, tension. No mode or arp mode.
    V0 = 0,
    /// 16-byte points: f32 x_offset, y, tension + mode, arp mode, 2 bytes.
    V1 = 1,
    /// 24-byte points like `V3`, but LFOs have no phase.
    V2 = 2,
    /// 24-byte points: f64 x_offset, y, f32 tension + mode, arp mode, 2 bytes.
    V3 = 3,
}

impl Default for Version {
    fn default() -> Self {
        Version::V3
    }
}

impl TryFrom<u32> for Version {
    type Error = u32;
    fn try_from(n: u32) -> Result<Self, u32> {
        match n {
            0 => Ok(Version::V0),
            1 => Ok(Version::V1),
            2 => Ok(Version::V2),
            3 => Ok(Version::V3),
            n => Err(n),
        }
    }
}

impl Version {
    /// Size of one point record in bytes.
    pub fn point_len(self) -> usize {
        match self {
            Version::V0 => 12,
            Version::V1 => 16,
            Version::V2 | Version::V3 => 24,
        }
    }
    /// LFO footers only have a phase field from version 3 on.
    pub fn has_lfo_phase(self) -> bool {
        self >= Version::V3
    }
}

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
    pub fn footer_len(self, version: Version) -> usize {
        CommonFooter::LEN
            + match self {
                CurveType::Envelope => EnvParams::LEN,
                CurveType::Lfo if version.has_lfo_phase() => LfoParams::LEN,
                CurveType::Lfo => LfoParams::LEN - 4,
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
    /// LFO only. The bit is inverted in the file: 
    /// it is set when bipolar is off, i.e. when it is unipolar
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

/// Extra 20 footer bytes of LFOs (16 before version 3, which has no phase).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LfoParams {
    /// knob 0.0..=1.0 maps linearly to 512..=65536
    pub speed: u32,
    /// -128..=128
    pub tension: i32,
    /// -128..=128
    pub skew: i32,
    /// -128..=128
    pub pulse_width: i32,
    /// raw value, probably the full u32 range = one cycle (not measured yet).
    /// Only stored from version 3 on; 0 when read from older files.
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
    /// Format version the file was read from, and the one `to_bytes` writes.
    pub version: Version,
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
