mod vector;
pub use vector::{
    Vector2, Vector2Int, Vector2Bool, Vector2f64,
    Vector3, Vector3Int, Vector3Bool, Vector3f64,
    Vector4, Vector4Int, Vector4Bool, Vector4f64,
};

mod matrix;
pub use matrix::{ Matrix3x3, Matrix4x4 };

mod quaternion;
pub use quaternion::Quaternion;

mod angle_axis;
pub use angle_axis::AngleAxis;

mod transform;
pub use transform::Transform;

/// `RGB` and `HSV` data structures
pub mod color;
