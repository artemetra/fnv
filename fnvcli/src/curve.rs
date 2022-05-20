enum CurveType {
    None, // internal
    Envelope = 0x01,
    Lfo = 0x02,
	Graph = 0x03,
	Map = 0x07,
}

struct Curve {
    points: Vec<Point>,
}

#[derive(Default)]
struct Flags {
    m_tempo: bool,
	m_global: bool,
	m_on: bool,
}


struct EnvADSRSection {
    decay: u32,
    loop_start: u32,
    
    // in fl, it's written as "Sustain / Loop End" so i'm unsure which one to use
    sustain: u32,
}

// i really should use traits
struct LfoADSRSection {
    loop_start: u32,
    sustain: u32,
}

struct GraphCurve {
    points: Vec<Point>,
}
