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

pub fn min_array(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0].min(b[0]), a[1].min(b[1]), a[1].min(b[1])]
}
pub fn max_array(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0].max(b[0]), a[1].max(b[1]), a[1].max(b[1])]
}
pub fn minmax<T: std::cmp::PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
