pub mod hexadecimal;
pub mod angles;

use super::PI;

/// Linearly interpolate from `a` to `b`
/// 
/// Clamps `t` between **0.0** and **1.0**
/// 
/// Returns: `f32` with values between `a` and `b`
pub fn lerp( a:f32, b:f32, t:f32 ) -> f32 {
    lerp_unclamped(a, b, t.clamp(0.0, 1.0))
}

/// Linearly interpolate from `a` to `b`
/// 
/// Does **not** clamp `t`
/// 
/// Returns: `f32` with values between `a` and `b`
pub fn lerp_unclamped( a:f32, b:f32, t:f32 ) -> f32 {
    ( 1.0 - t ) * a + b * t
}

/// Get `t` for `value` between `a` and `b`
/// 
/// Returns: `f32`
pub fn inverse_lerp( a:f32, b:f32, v:f32 ) -> f32 {
    ( v - a ) / ( b - a )
}

/// Takes value `v` within given input range( `min1` to `max1` ) into given output range( `min2` to `max2` )
/// 
/// Returns: `f32`
pub fn remap( min1:f32, max1:f32, min2:f32, max2:f32, v:f32 ) -> f32 {
    lerp_unclamped( min2, max2, inverse_lerp( min1, max1, v ) )
}

/// Returns: the **largest** value in the given `array`
pub fn max<N>( values:&[N] ) -> N
where N:PartialOrd + Copy
{
    let mut largest = values[0];
    let mut i = 0;
    while i < values.len() {
        if values[i] > largest {
            largest = values[i];
        }
        i += 1;
    }

    return largest;
}

/// Returns: the **smallest** value in the given `array`
pub fn min<N>( values:&[N] ) -> N
where N:PartialOrd + Copy
{
    let mut smallest = values[0];
    let mut i = 0;
    while i < values.len() {
        if values[i] < smallest {
            smallest = values[i];
        }
        i += 1;
    }

    return smallest;
}