#![allow(dead_code)]
use std::rc::Weak;

use crate::curve::CurveTrait;
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

pub enum ArpMode {
    None = 0x00,
    Prev = 0x01,
    Same = 0x02,
    Next = 0x03,
}

// pub fn get_absolute_x<C>(curve: &C, idx: u32) -> f32
// where
//     C: CurveTrait,
// {
//     // not implemented
//     0 as f32
// }

pub trait PointTrait {
    fn absolute_x(&self) -> f32;
    fn parent_curve(&self) -> Weak<dyn CurveTrait>;
}

pub struct Point {
    x_offset: f32, // offset value
    y: f32,
    tension: f32,
    mode: PointMode,
}

pub struct EnvPoint {
    x_offset: f32, // offset value
    y: f32,
    tension: f32,
    mode: PointMode,
    arp_mode: ArpMode,
}
