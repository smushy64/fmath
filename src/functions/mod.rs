pub mod hexadecimal;
mod angles;
pub use angles::degrees_overflow;

/// Linearly interpolate from `a` to `b`
/// 
/// Clamps `t` between **0.0** and **1.0**
pub fn lerp( a:f32, b:f32, t:f32 ) -> f32 {
    lerp_unclamped(a, b, t.clamp(0.0, 1.0))
}
/// Linearly interpolate from `a` to `b`
pub fn lerp_unclamped( a:f32, b:f32, t:f32 ) -> f32 {
    ( 1.0 - t ) * a + b * t
}
/// Get `t` for `value` between `a` and `b`
pub fn inverse_lerp( a:f32, b:f32, v:f32 ) -> f32 {
    ( v - a ) / ( b - a )
}
/// Takes value `v` within given input range( `min1` to `max1` ) into given output range( `min2` to `max2` )
pub fn remap( min1:f32, max1:f32, min2:f32, max2:f32, v:f32 ) -> f32 {
    lerp_unclamped( min2, max2, inverse_lerp( min1, max1, v ) )
}

/// Linearly interpolate from `a` to `b`
/// 
/// Clamps `t` between **0.0** and **1.0**
pub fn lerp_f64( a:f64, b:f64, t:f64 ) -> f64 {
    lerp_unclamped_f64(a, b, t.clamp(0.0, 1.0))
}
/// Linearly interpolate from `a` to `b`
pub fn lerp_unclamped_f64( a:f64, b:f64, t:f64 ) -> f64 {
    ( 1.0 - t ) * a + b * t
}
/// Get `t` for `value` between `a` and `b`
pub fn inverse_lerp_f64( a:f64, b:f64, v:f64 ) -> f64 {
    ( v - a ) / ( b - a )
}
/// Takes value `v` within given input range( `min1` to `max1` ) into given output range( `min2` to `max2` )
pub fn remap_f64( min1:f64, max1:f64, min2:f64, max2:f64, v:f64 ) -> f64 {
    lerp_unclamped_f64( min2, max2, inverse_lerp_f64( min1, max1, v ) )
}

/// Returns: `Vec<u8>` **little-endian** byte representation of `&[f32]`
pub fn array_f32_to_le_bytes( array:&[f32] ) -> Vec<u8> {
    let mut buffer = Vec::with_capacity( array.len() * F32SIZE );
    for f in array.iter() { buffer.extend_from_slice( &f.to_le_bytes() ); }
    buffer
}

/// Returns: `Vec<u8>` **big-endian** byte representation of `&[f32]`
pub fn array_f32_to_be_bytes( array:&[f32] ) -> Vec<u8> {
    let mut buffer = Vec::with_capacity( array.len() * F32SIZE );
    for f in array.iter() { buffer.extend_from_slice( &f.to_be_bytes() ); }
    buffer
}

const F32SIZE:usize = 4;
