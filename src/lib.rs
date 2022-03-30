//! Math Library for my personal Rust Projects
//! 
//! Provides various math-related types and functions that are relevant to my projects

#![allow(dead_code)]
/// Types such as:
/// 
/// - Vectors: `Vector2`, `Vector3`, `Vector4`
/// - Colors: `RGB`, `RGBA`, `RGB8`, `RGBA8`, `HSV`
/// - `Matrix4x4`
/// - `Angle` enum
pub mod types;

/// Functions such as:
/// 
/// - `min(&[N])` and `max(&[N])` where `N` is a type that impl's `PartialOrd` + `Copy`
/// - `clamp(value:N, min:N, max:N)` where `N` is a type that impl's `PartialOrd`
/// 
/// etc...
pub mod functions;

/// π
pub use core::f32::consts::PI;
/// π / 2.0
pub use core::f32::consts::FRAC_PI_2;