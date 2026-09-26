use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// How the connection to the *left* of a point (previous point -> this one) is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointMode {
    SingleCurve = 0x00,
    DoubleCurve = 0x01,
    Hold = 0x02,
    Stairs = 0x03,
    SmoothStairs = 0x04,
    Pulse = 0x05,
    Wave = 0x06,
    SingleCurve2 = 0x07,
    DoubleCurve2 = 0x08,
    HalfSine = 0x09,
    Smooth = 0x0A,
    SingleCurve3 = 0x0B,
    DoubleCurve3 = 0x0C,
}

impl Default for PointMode {
    fn default() -> Self {
        PointMode::SingleCurve
    }
}

impl TryFrom<u8> for PointMode {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, u8> {
        use PointMode::*;
        Ok(match byte {
            0x00 => SingleCurve,
            0x01 => DoubleCurve,
            0x02 => Hold,
            0x03 => Stairs,
            0x04 => SmoothStairs,
            0x05 => Pulse,
            0x06 => Wave,
            0x07 => SingleCurve2,
            0x08 => DoubleCurve2,
            0x09 => HalfSine,
            0x0A => Smooth,
            0x0B => SingleCurve3,
            0x0C => DoubleCurve3,
            n => return Err(n),
        })
    }
}

/// Envelope arpeggiator mode. Graph points always have `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArpMode {
    None = 0x00,
    Prev = 0x01,
    Same = 0x02,
    Next = 0x03,
}

impl Default for ArpMode {
    fn default() -> Self {
        ArpMode::None
    }
}

impl TryFrom<u8> for ArpMode {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, u8> {
        Ok(match byte {
            0x00 => ArpMode::None,
            0x01 => ArpMode::Prev,
            0x02 => ArpMode::Same,
            0x03 => ArpMode::Next,
            n => return Err(n),
        })
    }
}

/// A single point record. In versions 2 and 3 it is 24 bytes:
///
/// | offset | type | field      |
/// |--------|------|------------|
/// | 0x00   | f64  | x_offset   |
/// | 0x08   | f64  | y          |
/// | 0x10   | f32  | tension    |
/// | 0x14   | u8   | mode       |
/// | 0x15   | u8   | arp_mode   |
/// | 0x16   | u8   | reserved   |
/// | 0x17   | i8   | tension_sign |
///
/// Version 1 (16 bytes) stores x_offset and y as f32, followed by the same
/// tension and 4 bytes. Version 0 (12 bytes) is only the three f32 values;
/// the other fields are left at their defaults when reading it.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    /// x distance from the previous point (not an absolute coordinate)
    pub x_offset: f64,
    pub y: f64,
    /// -1.0..=1.0 (FL shows it as -100%..100%)
    pub tension: f32,
    pub mode: PointMode,
    pub arp_mode: ArpMode,
    /// always 0 in files saved by FL
    pub reserved: u8,
    /// 1 for positive tension, -1 for negative, 2 (sometimes 0) for zero,
    /// and always 0 on the first point. FL ignores it on load and recomputes
    /// it on save (tested with every point mode in every curve type).
    /// Always 0 in version 1 files, and often 0 in version 2 files.
    pub tension_sign: i8,
}
