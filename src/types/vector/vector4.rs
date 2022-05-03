use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Neg, Index, IndexMut
};

use super::{
    Vector2,
    Vector3
};

/// 4-component Vector
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector4 {
    components:[f32;4]
}

impl Vector4 {

    /// Create new `Vector4` with `x`, `y`, `z` and `w` set to **1.0**
    pub fn new_one() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    /// Create new `Vector4` with `x`, `y`, `z` and `w` set to **0.0**
    pub fn new_zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Create new `Vector4` with given `x`, `y`, `z` and `w` components
    pub fn new( x:f32, y:f32, z:f32, w:f32 ) -> Self {
        Self {
            components:[x,y,z,w]
        }
    }

    /// Create new `Vector4` from 4-component `array`
    pub fn from_array( components:[f32;4] ) -> Self {
        Self {
            components
        }
    }

    /// Returns: `reference` to vector's components `array`
    pub fn as_array(&self) -> &[f32;4] {
        &self.components
    }

    /// Returns: `mutable reference` to vector's components `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;4] {
        &mut self.components
    }

    /// Returns: `reference` to `x` component
    pub fn x(&self) -> &f32 {
        &self.components[0]
    }

    /// Returns: `reference` to `y` component
    pub fn y(&self) -> &f32 {
        &self.components[1]
    }

    /// Returns: `reference` to `z` component
    pub fn z(&self) -> &f32 {
        &self.components[2]
    }

    /// Returns: `reference` to `w` component
    pub fn w(&self) -> &f32 {
        &self.components[3]
    }

    /// Returns: `mutable reference` to `x` component
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.components[0]
    }

    /// Returns: `mutable reference` to `y` component
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.components[1]
    }

    /// Returns: `mutable reference` to `z` component
    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.components[2]
    }

    /// Returns: `mutable reference` to `w` component
    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.components[3]
    }

    /// Set `x`, `y`, `z` and `w` component
    pub fn set(&mut self, x:f32, y:f32, z:f32, w:f32) {
        self.components = [x,y,z,w];
    }

    /// Assign components to given `array`
    pub fn set_array(&mut self, components:[f32;4]) {
        self.components = components;
    }

    // =============================== &self =====================================
    /// Returns: new `Vector4` with the same direction but with `magnitude` of **1.0**
    pub fn normal(&self) -> Self {
        let mag = self.magnitude();
        Self {
            components: [
                self.x() / mag,
                self.y() / mag,
                self.z() / mag,
                self.w() / mag,
            ]
        }
    }

    /// Returns: vector's length
    pub fn magnitude(&self) -> f32 {
        super::magnitude_components(&self.components)
    }

    /// Returns: vector's length without applying square root
    /// 
    /// Useful when you only need to compare vectors' lengths.
    pub fn sqr_magnitude(&self) -> f32 {
        super::sqr_magnitude_components(&self.components)
    }

    // =============================== &mut self =====================================
    /// Clamp vector's `magnitude` to given `max`
    pub fn clamp_magnitude(&mut self, max:f32) {
        super::clamp_magnitude_components(&mut self.components, max);
    }

    // =============================== STATIC =====================================
    /// Normalize given vector
    pub fn normalize( v:&mut Self ) {
        let mag = v.magnitude();
        let mut i = 0;
        while i < v.components.len() {
            v[i] = v[i] / mag;
            i += 1;
        }
    }

    /// Linearly interpolate from `a` to `b`
    /// 
    /// Clamps `t` between **0.0** and **1.0**
    /// 
    /// Returns: new `Vector4` with values between `a` and `b`
    pub fn lerp( a:&Self, b:&Self, t:f32 ) -> Self {
        Self::lerp_unclamped(a, b, t.clamp(0.0, 1.0))
    }

    /// Linearly interpolate from `a` to `b`
    /// 
    /// Does **not** clamp `t`
    /// 
    /// Returns: new `Vector4` with values between `a` and `b`
    pub fn lerp_unclamped( a:&Self, b:&Self, t:f32 ) -> Self {
        ( *a * ( 1.0 - t ) ) + ( *b * t )
    }

    /// Returns: **dot** product of `a` and `b`
    pub fn dot( a:&Self, b:&Self ) -> f32 {
        super::dot_components( a.as_array(), b.as_array() )
    }

    /// Returns: angle between `a` and `b`
    pub fn angle( a:&Self, b:&Self ) -> f32 {
        super::angle_components( a.as_array(), b.as_array() )
    }

    /// Component-wise scale `a` and `b`
    /// 
    /// Returns: new `Vector4` with each component being a\[-\] * b\[-\]
    pub fn scale( a:&Self, b:&Self ) -> Self {
        let mut result = a.components.clone();
        super::componentwise_scale_components(a.as_array(), b.as_array(), &mut result);
        Self::from_array(result)
    }

    /// Reflect `direction` off of `normal`
    /// 
    /// `normal` should be a normalized vector
    /// 
    /// Returns: new `Vector4`
    pub fn reflect( direction:&Self, normal:&Self ) -> Self {
        let mut result = direction.components.clone();
        super::reflect_components(direction.as_array(), normal.as_array(), &mut result);
        Self::from_array(result)
    }

    /// Calculate distance between `from` and `to`
    pub fn distance( from:&Self, to:&Self ) -> f32 {
        ( *from - *to ).magnitude()
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, 
            "( {:7.3}, {:7.3}, {:7.3}, {:7.3} )", self.x(), self.y(), self.z(), self.w()
        )
    }
}

/// Create new `Vector4` from `Vector2`
/// 
/// `z` and `w` components are set to **0.0**
impl From<Vector2> for Vector4 {
    fn from(v: Vector2) -> Self {
        Self::new(v[0], v[1], 0.0, 0.0)
    }
}

/// Create new `Vector4` from `Vector3`
/// 
/// `w` component is set to **0.0**
impl From<Vector3> for Vector4 {
    fn from(v: Vector3) -> Self {
        Self::new(v[0], v[1], v[2], 0.0)
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index:usize) -> &f32 {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index:usize) -> &mut f32 {
        &mut self.components[index]
    }
}

impl Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = self.components.clone();
        super::negate_components(&mut result);
        Self::from_array(result)
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        
        let mut result = self.components.clone();
        super::add_components(self.as_array(), rhs.as_array(), &mut result);
        Self::from_array(result)

    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        
        let mut result = self.components.clone();
        super::sub_components(self.as_array(), rhs.as_array(), &mut result);
        Self::from_array(result)

    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs:f32) -> Self {
        
        let mut result = self.components.clone();
        super::scale_components(self.as_array(), rhs, &mut result);
        Self::from_array(result)

    }
}

impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs:f32) -> Self {
        
        let mut result = self.components.clone();
        super::scale_components(self.as_array(), 1.0/rhs, &mut result);
        Self::from_array(result)

    }
}