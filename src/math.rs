pub fn lerp<T: std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>>(
    a: T,
    b: T,
    t: f32,
) -> T {
    a * (1.0 - t) + b * t
}

pub fn clamp<T: PartialOrd<T>>(low: T, high: T, value: T) -> T {
    debug_assert!(low <= high, "low must be greater than high");
    if value > high {
        high
    } else if value < low {
        low
    } else {
        value
    }
}
