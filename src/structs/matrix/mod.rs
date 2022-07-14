#[allow(dead_code)]

mod matrix4x4;
pub use matrix4x4::Matrix4x4;

mod matrix3x3;
pub use matrix3x3::Matrix3x3;

pub(crate) fn determinant3x3( m:&[f32;9] ) -> f32 {
     ( m[0] * ( ( m[4] * m[8] ) - ( m[7] * m[5] ) ) ) +
    -( m[3] * ( ( m[1] * m[8] ) - ( m[7] * m[2] ) ) ) +
     ( m[6] * ( ( m[1] * m[5] ) - ( m[4] * m[2] ) ) )
}

pub(crate) fn determinant2x2( m:&[f32;4] ) -> f32 {
    ( m[0] * m[3] ) - ( m[2] * m[1] )
}