enum PointMode {
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

enum ArpMode {
    None = 0x00,
    Prev = 0x01,
    Same = 0x02,
    Next = 0x03,
}

struct Point {
    x: f32, // offset value
    y: f32,
    tension: f32,
    mode: PointMode,
}

// I'm really unsure about this: probably needs to be
// rewritten using traits
struct EnvPoint {
    x: f32, // offset value
    y: f32,
    tension: f32,
    mode: PointMode,
    arp_mode: ArpMode,
}
