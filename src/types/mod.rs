/// `Vector2`, `Vector3`, `Vector4`
pub mod vector;
/// `Matrix4x4`
pub mod matrix;
/// `RGB`, `RGBA`, `RGB8`, `RGBA8`, `HSV`
pub mod color;

/// `Degrees` or `Radians`
/// 
/// Used as an input for various rotation functions
pub enum Angle {
    Degrees,
    Radians,
}
use core::fmt::Display;
impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Angle::Degrees => "Angle: Degrees",
            Angle::Radians => "Angle: Radians",
        };
        write!( f, "{}", output )
    }
}