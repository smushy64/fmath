mod vector;
pub use vector::{
    Vector2, Vector3, Vector4
};

mod matrix;
pub use matrix::Matrix4x4;

mod quaternion;
pub use quaternion::Quaternion;

/// `RGB` and `HSV` data structures
pub mod color;
