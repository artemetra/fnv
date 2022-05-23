use crate::point::{EnvPoint, Point};

#[derive(Debug)]
pub enum CurveType {
    None,
    Envelope = 0x01,
    Lfo = 0x02,
    Graph = 0x03,
    Map = 0x07,
}

#[derive(Default)]
pub struct Flags {
    tempo: bool,
    global: bool,
    on: bool,
}

pub struct EnvADSRIndices {
    decay: u32,
    loop_start: u32,
    // in FL, it's written as "Sustain / Loop End" so i'm unsure which one to use
    sustain: u32,
}

// i really should use traits
pub struct LfoADSRIndices {
    loop_start: u32,
    sustain: u32,
}

pub trait CurveTrait {
    fn point_count() -> u32;
}

pub struct GraphCurve {
    points: Vec<Point>,
}

pub struct EnvCurve {
    points: Vec<EnvPoint>,
    adsr: EnvADSRIndices,
}
