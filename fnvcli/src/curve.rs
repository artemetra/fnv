#![allow(dead_code)]

use crate::point::{EnvPoint, Point};

#[derive(Debug, PartialEq, Eq)]
pub enum CurveType {
    Envelope = 0x01,
    Lfo = 0x02,
    Graph = 0x03,
    Map = 0x07,
}

#[derive(Default)]
pub struct EnvFlags {
    tempo: bool,
    global: bool,
}

struct InvalidEnvFlagByte(u8);
impl EnvFlags {
    fn new(tempo: bool, global: bool) -> EnvFlags {
        EnvFlags { tempo, global }
    }
    //                               should i just use Option<>?
    fn from_byte(byte: u8) -> Result<EnvFlags, InvalidEnvFlagByte> {
        match byte {
            0 => Ok(EnvFlags {
                tempo: false,
                global: false,
            }),
            1 => Ok(EnvFlags {
                tempo: true,
                global: false,
            }),
            2 => Ok(EnvFlags {
                tempo: false,
                global: true,
            }),
            3 => Ok(EnvFlags {
                tempo: true,
                global: true,
            }),
            n => Err(InvalidEnvFlagByte(n)),
        }
    }
    fn as_byte(&self) -> u8 {
        ((self.global as u8) << 1) | (self.tempo as u8)
    }
}
pub struct LfoFlags {
    frozen: bool,
    bipolar: bool,
}

struct InvalidLfoFlagByte(u8);

impl LfoFlags {
    fn new(frozen: bool, bipolar: bool) -> LfoFlags {
        LfoFlags { frozen, bipolar }
    }
    fn from_byte(byte: u8) -> Result<LfoFlags, InvalidLfoFlagByte> {
        match byte {
            0x07 => Ok(LfoFlags {
                frozen: false,
                bipolar: false,
            }),
            0x0F => Ok(LfoFlags {
                frozen: true,
                bipolar: false,
            }),
            0x03 => Ok(LfoFlags {
                frozen: false,
                bipolar: true,
            }),
            0x0B => Ok(LfoFlags {
                frozen: true,
                bipolar: true,
            }),
            n => Err(InvalidLfoFlagByte(n)),
        }
    }
    fn as_byte(&self) -> u8 {
        match self {
            LfoFlags {
                frozen: false,
                bipolar: false,
            } => 0x07,
            LfoFlags {
                frozen: true,
                bipolar: false,
            } => 0x0F,
            LfoFlags {
                frozen: false,
                bipolar: true,
            } => 0x03,
            LfoFlags {
                frozen: true,
                bipolar: true,
            } => 0x0B,
        }
    }
}

pub struct EnvADSRIndices {
    decay: Option<u32>,
    loop_start: Option<u32>,
    // in FL, it's written as "Sustain / Loop End" so i'm unsure which one to use
    sustain: Option<u32>,
}

// i really should use traits
pub struct LfoADSRIndices {
    loop_start: Option<u32>,
    sustain: Option<u32>,
}

pub trait ADSRIndices {
    fn decay(&self) -> Option<u32>;
    fn loop_start(&self) -> Option<u32>;
    fn sustain(&self) -> Option<u32>;
}

pub trait CurveTrait {
    fn points(&self) -> &Vec<Point>;
    fn point_count(&self) -> u32;
    fn minimum_point_count() -> u8
    where
        Self: Sized,
    {
        1
    }
    fn curve_type(&self) -> CurveType;
    fn adsr_indices(&self) -> Option<Box<dyn ADSRIndices>>;
    fn flags(&self) -> Option<EnvFlags>;
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphCurve {
    points_: Vec<Point>,
}

impl CurveTrait for GraphCurve {
    fn points(&self) -> &Vec<Point> {
        &self.points_
    }
    fn point_count(&self) -> u32 {
        self.points_.len() as u32
    }
    fn curve_type(&self) -> CurveType {
        CurveType::Graph
    }
    fn adsr_indices(&self) -> Option<Box<dyn ADSRIndices>> {
        None
    }
    fn flags(&self) -> Option<EnvFlags> {
        None
    }
    fn minimum_point_count() -> u8
    where
        Self: Sized,
    {
        2
    }
}

pub struct EQCurve {
    points_: Vec<Point>,
}

pub struct EnvADSR {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

pub struct EnvCurve {
    points_: Vec<EnvPoint>,
    on: bool,
    flags_: EnvFlags,
    adsr: EnvADSR,
    adsr_indices: EnvADSRIndices,
}

// TODO: fix naming
pub struct LfoShape {
    speed: f32,
    tension: f32,
    skew: f32,
    pulsewidth: f32,
}

pub struct LfoCurve {
    points_: Vec<EnvPoint>,
    on: bool,
    flags_: EnvFlags,
    adsr_indices: LfoADSRIndices,
    shape: LfoShape,
    phase: f32,
}
