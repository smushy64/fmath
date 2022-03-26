use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Neg, Index, IndexMut
};

pub mod consts {
    use super::Vector3;
    /// `Vector3` with components set to **0.0**
    pub const VECTOR3_ZERO   :Vector3 = Vector3{ components:[ 0.0,  0.0,  0.0] };
    /// `Vector3` with components set to **1.0**
    pub const VECTOR3_ONE    :Vector3 = Vector3{ components:[ 1.0,  1.0,  1.0] };
    /// `Vector3` with **-1.0** in the `x` component
    pub const VECTOR3_LEFT   :Vector3 = Vector3{ components:[-1.0,  0.0,  0.0] };
    /// `Vector3` with **1.0** in the `x` component
    pub const VECTOR3_RIGHT  :Vector3 = Vector3{ components:[ 1.0,  0.0,  0.0] };
    /// `Vector3` with **1.0** in the `y` component
    pub const VECTOR3_UP     :Vector3 = Vector3{ components:[ 0.0,  1.0,  0.0] };
    /// `Vector3` with **-1.0** in the `y` component
    pub const VECTOR3_DOWN   :Vector3 = Vector3{ components:[ 0.0, -1.0,  0.0] };
    /// `Vector3` with **1.0** in the `z` component
    pub const VECTOR3_FORWARD:Vector3 = Vector3{ components:[ 0.0,  0.0,  1.0] };
    /// `Vector3` with **-1.0** in the `z` component
    pub const VECTOR3_BACK   :Vector3 = Vector3{ components:[ 0.0,  0.0, -1.0] };
}

/// 3-component Vector
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector3 {
    components:[f32;3]
}

impl Vector3 {
    /// Create new `Vector3` with given `x`, `y` and `z` components
    pub fn new( x:f32, y:f32, z:f32 ) -> Self {
        Self {
            components:[x,y,z]
        }
    }

    /// Create new `Vector3` from 3-component `array`
    pub fn from_array( components:[f32;3] ) -> Self {
        Self {
            components
        }
    }

    /// Returns: `reference` to vector's components `array`
    pub fn as_array(&self) -> &[f32;3] {
        &self.components
    }

    /// Returns: `mutable reference` to vector's components `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;3] {
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

    /// Set `x`, `y` and `z` component
    pub fn set(&mut self, x:f32, y:f32, z:f32) {
        self.components = [x,y,z];
    }

    /// Assign components to given `array`
    pub fn set_array(&mut self, components:[f32;3]) {
        self.components = components;
    }

    // =============================== &self =====================================
    /// Returns: new `Vector3` with the same direction but with `magnitude` of **1.0**
    pub fn normal(&self) -> Self {
        let mag = self.magnitude();
        Self {
            components: [
                self.x() / mag,
                self.y() / mag,
                self.z() / mag,
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
    /// Returns: new `Vector3` with values between `a` and `b`
    pub fn lerp( a:&Self, b:&Self, t:f32 ) -> Self {
        Self::lerp_unclamped(a, b, crate::clamp(t, 0.0, 1.0))
    }

    /// Linearly interpolate from `a` to `b`
    /// 
    /// Does **not** clamp `t`
    /// 
    /// Returns: new `Vector3` with values between `a` and `b`
    pub fn lerp_unclamped( a:&Self, b:&Self, t:f32 ) -> Self {
        ( *a * ( 1.0 - t ) ) + ( *b * t )
    }

    /// Returns: **dot** product of `a` and `b`
    pub fn dot( a:&Self, b:&Self ) -> f32 {
        super::dot_components( a.as_array(), b.as_array() )
    }

    /// Returns: **cross** product of `a` and `b`
    pub fn cross( a:&Self, b:&Self ) -> Self {
        Self::from_array([
            ( a.y() * b.z() ) - ( a.z() * b.y() ),
            ( a.z() * b.x() ) - ( a.x() * b.z() ),
            ( a.x() * b.y() ) - ( a.y() * b.x() ),
        ])
    }

    /// Returns: angle between `a` and `b`
    pub fn angle( a:&Self, b:&Self ) -> f32 {
        super::angle_components( a.as_array(), b.as_array() )
    }

    /// Component-wise scale `a` and `b`
    /// 
    /// Returns: new `Vector3` with each component being a\[-\] * b\[-\]
    pub fn scale( a:&Self, b:&Self ) -> Self {
        let mut result = a.components.clone();
        super::componentwise_scale_components(a.as_array(), b.as_array(), &mut result);
        Self::from_array(result)
    }

    /// Reflect `direction` off of `normal`
    /// 
    /// `normal` should be a normalized vector
    /// 
    /// Returns: new `Vector3`
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

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, 
            "Vector 3: {}, {}, {}", self.x(), self.y(), self.z()
        )
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index:usize) -> &f32 {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index:usize) -> &mut f32 {
        &mut self.components[index]
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = self.components.clone();
        super::negate_components(&mut result);
        Self::from_array(result)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        
        let mut result = self.components.clone();
        super::add_components(self.as_array(), rhs.as_array(), &mut result);
        Self::from_array(result)

    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        
        let mut result = self.components.clone();
        super::sub_components(self.as_array(), rhs.as_array(), &mut result);
        Self::from_array(result)

    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs:f32) -> Self {
        
        let mut result = self.components.clone();
        super::scale_components(self.as_array(), rhs, &mut result);
        Self::from_array(result)

    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs:f32) -> Self {
        
        let mut result = self.components.clone();
        super::scale_components(self.as_array(), 1.0/rhs, &mut result);
        Self::from_array(result)

    }
}