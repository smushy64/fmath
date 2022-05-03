//! Math Library for my personal Rust Projects
//! 
//! Provides various math-related types and functions that are relevant to my projects

#![allow(dead_code)]
/// Vectors, Matrix4x4, colors
/// 
/// - `Vector2` `Vector3` `Vector4`
/// - `Matrix4x4`
/// - Colors
///     - `RGB`
///     - `HSV`
/// - `Quaternion` ***work in progress***
pub mod types;

/// Various helper functions
/// 
/// - `lerp` `inverse lerp` `remap`
/// - `max` and `min` for number arrays
/// - hexadecimal
///     - `decode` and `encode`
/// - angles
///     - degrees -> radians
///     - radians -> degrees
///     - degree overflow ( wrap value between 0.0-360.0 )
pub mod functions;

/// π
pub use core::f32::consts::PI;
/// π / 2.0
pub use core::f32::consts::FRAC_PI_2;