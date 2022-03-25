use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Neg, Index, IndexMut
};

/// `Vector2` with components set to **0.0**
pub const VECTOR2_ZERO :Vector2 = Vector2{ components:[ 0.0,  0.0] };
/// `Vector2` with components set to **1.0**
pub const VECTOR2_ONE  :Vector2 = Vector2{ components:[ 1.0,  1.0] };
/// `Vector2` with **-1.0** in the `x` component
pub const VECTOR2_LEFT :Vector2 = Vector2{ components:[-1.0,  0.0] };
/// `Vector2` with **1.0** in the `x` component
pub const VECTOR2_RIGHT:Vector2 = Vector2{ components:[ 1.0,  0.0] };
/// `Vector2` with **1.0** in the `y` component
pub const VECTOR2_UP   :Vector2 = Vector2{ components:[ 0.0,  1.0] };
/// `Vector2` with **-1.0** in the `y` component
pub const VECTOR2_DOWN :Vector2 = Vector2{ components:[ 0.0, -1.0] };

/// 2-component Vector
/// 
/// Indexable with **[ ]**
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    components:[f32;2]
}

impl Vector2 {
    /// Create new `Vector2` with given `x` and `y` components
    pub fn new( x:f32, y:f32 ) -> Self {
        Self {
            components:[x,y]
        }
    }

    /// Create new `Vector2` from 2-component `array`
    pub fn from_array( components:[f32;2] ) -> Self {
        Self {
            components
        }
    }

    /// Returns: `reference` to vector's components `array`
    pub fn as_array(&self) -> &[f32;2] {
        &self.components
    }

    /// Returns: `mutable reference` to vector's components `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;2] {
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

    /// Returns: `mutable reference` to `x` component
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.components[0]
    }

    /// Returns: `mutable reference` to `y` component
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.components[1]
    }

    /// Set `x` and `y` component
    pub fn set(&mut self, x:f32, y:f32) {
        self.components = [x,y];
    }

    /// Assign components to given `array`
    pub fn set_array(&mut self, components:[f32;2]) {
        self.components = components;
    }

    // =============================== &self =====================================
    /// Returns: new `Vector2` with the same direction but with `magnitude` of **1.0**
    pub fn normal(&self) -> Self {
        let mag = self.magnitude();
        Self {
            components: [
                self.x() / mag,
                self.y() / mag,
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
    /// Clamp vector's length(`magnitude`) to given length
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
    /// Returns: new `Vector2` with values between `a` and `b`
    pub fn lerp( a:&Self, b:&Self, t:f32 ) -> Self {
        Self::lerp_unclamped(a, b, crate::clamp(t, 0.0, 1.0))
    }

    /// Linearly interpolate from `a` to `b`
    /// 
    /// Does **not** clamp `t`
    /// 
    /// Returns: new `Vector2` with values between `a` and `b`
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
    /// Returns: new `Vector2` with each component being a\[-\] * b\[-\]
    pub fn scale( a:&Self, b:&Self ) -> Self {
        let mut result = a.components.clone();
        super::componentwise_scale_components(a.as_array(), b.as_array(), &mut result);
        Self::from_array(result)
    }

    /// Reflect `direction` off of `normal`
    /// 
    /// `normal` should be a normalized vector
    /// 
    /// Returns: new `Vector2`
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

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, 
            "Vector 2: {}, {}", self.x(), self.y()
        )
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index:usize) -> &f32 {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index:usize) -> &mut f32 {
        &mut self.components[index]
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = self.components.clone();
        super::negate_components(&mut result);
        Self::from_array(result)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        
        let mut result = self.components.clone();
        super::add_components(self.as_array(), rhs.as_array(), &mut result);
        Self::from_array(result)

    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        
        let mut result = self.components.clone();
        super::sub_components(self.as_array(), rhs.as_array(), &mut result);
        Self::from_array(result)

    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs:f32) -> Self {
        
        let mut result = self.components.clone();
        super::scale_components(self.as_array(), rhs, &mut result);
        Self::from_array(result)

    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs:f32) -> Self {
        
        let mut result = self.components.clone();
        super::scale_components(self.as_array(), 1.0/rhs, &mut result);
        Self::from_array(result)

    }
}