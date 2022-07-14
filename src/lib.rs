//! Math Library for my personal Rust Projects
//! 
//! Provides various math-related structs and functions that are relevant to my projects

#![allow(dead_code)]
/// Vectors, Matrices, colors
/// 
/// - `Vector2` `Vector3` `Vector4`
/// - `Matrix3x3` `Matrix4x4` `Transform`
/// - Colors
///     - `RGB`
///     - `HSV`
/// - `Quaternion`
pub mod structs;

/// Various helper functions
/// 
/// - `lerp` `inverse lerp` `remap`
/// - `max` and `min` for number arrays
/// - `decode` and `encode` hexadecimal
/// - `degree overflow ( wrap value between 0.0-360.0 )`
pub mod functions;

/// π
pub use core::f32::consts::PI;
/// π / 2.0
pub use core::f32::consts::FRAC_PI_2;

pub(crate) unsafe fn any_as_byte_slice<T: Sized>( t:&T ) -> &[u8] {
    core::slice::from_raw_parts(
        (t as *const T) as *const u8,
        core::mem::size_of::<T>()
    )
}
