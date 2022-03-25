#[allow(dead_code)]

pub mod vector;
pub mod matrix;

pub use core::f32::consts::PI;

/// Clamps the given `value` between given `minimum` and `maximum`.
/// 
/// * Returns: `mininimum` if `value` is *less* than `minimum`.
/// 
/// * Returns: `maximum` if `value` is *greater* than `maximum`.
/// 
/// * Returns: `value` if `value` is between the `minimum` and `maximum` range.
/// 
pub fn clamp<N>( value:N, min:N, max:N ) -> N
where N:PartialOrd
{
    assert!( min <= max );
    if value < min {
        return min;
    } else if value > max {
        return max;
    }

    return value;
}

/// `Degrees` or `Radians`
/// 
/// Used as an input for various rotation functions
pub enum Angle {
    Degrees,
    Radians,
}

/// Returns: radians as`f32`
pub fn degrees_to_radians(d:f32) -> f32 {
    d * PI/180.0
}

/// Returns: degrees as `f32`
pub fn radians_to_degrees(r:f32) -> f32 {
    r * 180.0/PI
}