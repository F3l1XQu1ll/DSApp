pub trait MinMax: Clone + Copy + PartialEq + PartialOrd + 'static {
    const MIN: Self;
    const MAX: Self;
}

impl MinMax for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl MinMax for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;
}

impl MinMax for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}
