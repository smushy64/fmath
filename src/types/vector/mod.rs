mod vector2;
pub use vector2::{
    Vector2,
};
mod vector3;
pub use vector3::{
    Vector3,
};
mod vector4;
pub use vector4::{
    Vector4,
};

fn negate_components( v:&mut [f32] ) {
    let mut i = 0;
    while i < v.len() {
        v[i] = -v[i];
        i += 1;
    }
}

/// Component-wise addition for two `arrays` with equal lenghts.
pub(crate) fn add_components( v1:&[f32], v2:&[f32], result:&mut [f32] ) {
    let mut i:usize = 0;
    while i < result.len() {
        result[i] = v1[i] + v2[i];
        i += 1;
    }
}

/// Component-wise subtraction for two `arrays` with equal lenghts.
pub(crate) fn sub_components( v1:&[f32], v2:&[f32], result:&mut [f32] ) {
    let mut i:usize = 0;
    while i < result.len() {
        result[i] = v1[i] - v2[i];
        i += 1;
    }
}

/// Component-wise scale `array` by `scalar`
pub(crate) fn scale_components( v:&[f32], scalar:f32, result:&mut [f32] ) {
    let mut i:usize = 0;
    while i < result.len() {
        result[i] = v[i] * scalar;
        i += 1;
    }
}

fn dot_components( v1:&[f32], v2:&[f32] ) -> f32 {
    let mut result:f32 = 0.0;
    let mut i = 0;
    while i < v1.len() {
        result = result + ( v1[i] * v2[i] );
        i += 1;
    }
    return result;
}

fn angle_components( v1:&[f32], v2:&[f32] ) -> f32 {
    dot_components(v1, v2).acos().abs()
}

fn sqr_magnitude_components( components:&[f32] ) -> f32 {
    let mut result = components[0] * components[0];
    for component in components.iter().skip(1) {
        result = result + ( component * component );
    }
    result
}

fn magnitude_components( components:&[f32] ) -> f32 {
    sqr_magnitude_components(components).sqrt()
}

fn clamp_magnitude_components( components:&mut[f32], max:f32 ) {
    let mag = magnitude_components(components);
    if mag > max {
        let mut i:usize = 0;
        while i < components.len() {
            components[i] = (components[i] / mag) * max;
            i += 1;
        }
    }
}

/// Component-wise scale `array` by `array`
pub(crate) fn componentwise_scale_components( v1:&[f32], v2:&[f32], result:&mut [f32] ) {
    let mut i = 0;
    while i < result.len() {
        result[i] = v1[i] * v2[i];
        i += 1;
    }
}

/// Component-wise divide `array` by `array`
pub(crate) fn componentwise_div_components( v1:&[f32], v2:&[f32], result:&mut [f32] ) {
    let mut i = 0;
    while i < result.len() {
        result[i] = v1[i] / v2[i];
        i += 1;
    }
}

fn reflect_components( v1:&[f32], v2:&[f32], result:&mut [f32] ) {
    // reflection = 2 * ( dot( v1, v2 ) ) * v2 - v1

    let dot = dot_components(v1, v2); // dot( v1, v2 )

    let mut sub_result:[f32;2] = [0.0, 0.0];
    sub_components(v2, v1, &mut sub_result); // v2 - v1

    let mut i = 0;
    while i < result.len() {
        result[i] = ( 2.0 * dot ) * sub_result[i]; // 2 * dot * ( v2 - v1 )
        i += 1;
    }

}