use serde::{Deserialize, Serialize};

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
    #[serde(with = "json_float::f64")]
    pub x_offset: f64,
    #[serde(with = "json_float::f64")]
    pub y: f64,
    /// -1.0..=1.0 (FL shows it as -100%..100%)
    #[serde(with = "json_float::f32")]
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

/// JSON has no NaN or infinity, but real files contain NaN coordinates (with
/// various payloads). Finite values are plain numbers; the rest are strings:
/// `"inf"`, `"-inf"`, `"NaN"` (the default quiet NaN) or `"NaN:0x<bits>"`,
/// so that every file survives a JSON round trip byte for byte.
mod json_float {
    macro_rules! float_module {
        ($name:ident, $float:ident, $bits:ident, $serialize:ident, $default_nan:expr, $width:expr) => {
            pub mod $name {
                use serde::{Deserializer, Serializer, de};
                use std::fmt;

                pub fn serialize<S: Serializer>(v: &$float, s: S) -> Result<S::Ok, S::Error> {
                    if v.is_finite() {
                        s.$serialize(*v)
                    } else if *v == $float::INFINITY {
                        s.serialize_str("inf")
                    } else if *v == $float::NEG_INFINITY {
                        s.serialize_str("-inf")
                    } else if v.to_bits() == $default_nan {
                        s.serialize_str("NaN")
                    } else {
                        s.serialize_str(&format!("NaN:{:#0w$x}", v.to_bits(), w = $width))
                    }
                }

                pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<$float, D::Error> {
                    d.deserialize_any(Visitor)
                }

                struct Visitor;

                impl de::Visitor<'_> for Visitor {
                    type Value = $float;
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str("a number, \"inf\", \"-inf\", \"NaN\" or \"NaN:0x<bits>\"")
                    }
                    fn visit_f64<E: de::Error>(self, v: f64) -> Result<$float, E> {
                        Ok(v as $float)
                    }
                    fn visit_u64<E: de::Error>(self, v: u64) -> Result<$float, E> {
                        Ok(v as $float)
                    }
                    fn visit_i64<E: de::Error>(self, v: i64) -> Result<$float, E> {
                        Ok(v as $float)
                    }
                    fn visit_str<E: de::Error>(self, v: &str) -> Result<$float, E> {
                        let bad = || E::invalid_value(de::Unexpected::Str(v), &self);
                        match v {
                            "inf" => Ok($float::INFINITY),
                            "-inf" => Ok($float::NEG_INFINITY),
                            "NaN" => Ok($float::from_bits($default_nan)),
                            _ => {
                                let hex = v.strip_prefix("NaN:0x").ok_or_else(bad)?;
                                let f = $float::from_bits($bits::from_str_radix(hex, 16).map_err(|_| bad())?);
                                if f.is_nan() { Ok(f) } else { Err(bad()) }
                            }
                        }
                    }
                }
            }
        };
    }

    float_module!(f64, f64, u64, serialize_f64, 0x7ff8_0000_0000_0000, 18);
    float_module!(f32, f32, u32, serialize_f32, 0x7fc0_0000, 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_non_finite_floats() {
        let payload_nan = f64::from_bits(0x7ff8_03ac_8000_0000);
        for (x_offset, y, tension, json) in [
            (0.5, -0.0, 0.25, r#""x_offset":0.5,"y":-0.0,"tension":0.25"#),
            (f64::NAN, f64::INFINITY, f32::NEG_INFINITY, r#""x_offset":"NaN","y":"inf","tension":"-inf""#),
            (payload_nan, 1.0, f32::from_bits(0xffc0_0001), r#""x_offset":"NaN:0x7ff803ac80000000","y":1.0,"tension":"NaN:0xffc00001""#),
        ] {
            let p = Point { x_offset, y, tension, ..Point::default() };
            let s = serde_json::to_string(&p).unwrap();
            assert!(s.contains(json), "{}", s);
            let back: Point = serde_json::from_str(&s).unwrap();
            assert_eq!(back.x_offset.to_bits(), x_offset.to_bits());
            assert_eq!(back.y.to_bits(), y.to_bits());
            assert_eq!(back.tension.to_bits(), tension.to_bits());
        }
        // integers are accepted too, and strings that aren't a NaN are rejected
        let p: Point = serde_json::from_str(
            r#"{"x_offset":1,"y":0,"tension":0,"mode":"Hold","arp_mode":"None","reserved":0,"tension_sign":2}"#,
        )
        .unwrap();
        assert_eq!((p.x_offset, p.mode), (1.0, PointMode::Hold));
        assert!(serde_json::from_str::<Point>(r#"{"x_offset":"NaN:0x3ff0000000000000","y":0,"tension":0,"mode":"Hold","arp_mode":"None","reserved":0,"tension_sign":2}"#).is_err());
    }
}
