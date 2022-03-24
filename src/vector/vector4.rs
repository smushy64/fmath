use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Neg
};

pub const VECTOR4_ZERO:Vector4 = Vector4{ components:[ 0.0,  0.0,  0.0, 0.0 ] };
pub const VECTOR4_ONE :Vector4 = Vector4{ components:[ 1.0,  1.0,  1.0, 1.0 ] };

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector4 {
    components:[f32;4]
}

impl Vector4 {
    pub fn new( x:f32, y:f32, z:f32, w:f32 ) -> Self {
        Self {
            components:[x,y,z,w]
        }
    }

    pub fn from_array( components:[f32;4] ) -> Self {
        Self {
            components
        }
    }

    pub fn as_array(&self) -> &[f32;4] {
        &self.components
    }

    pub fn x(&self) -> &f32 {
        &self.components[0]
    }

    pub fn y(&self) -> &f32 {
        &self.components[1]
    }

    pub fn z(&self) -> &f32 {
        &self.components[2]
    }

    pub fn w(&self) -> &f32 {
        &self.components[3]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.components[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.components[1]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.components[2]
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.components[3]
    }

    pub fn set(&mut self, x:f32, y:f32, z:f32, w:f32) {
        self.components = [x,y,z,w];
    }

    pub fn set_array(&mut self, components:[f32;4]) {
        self.components = components;
    }

    // =============================== &self =====================================
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

    pub fn magnitude(&self) -> f32 {
        super::magnitude_components(&self.components)
    }

    pub fn sqr_magnitude(&self) -> f32 {
        super::sqr_magnitude_components(&self.components)
    }

    // =============================== &mut self =====================================
    pub fn clamp_magnitude(&mut self, max:f32) {
        super::clamp_magnitude_components(&mut self.components, max);
    }

    // =============================== STATIC =====================================
    pub fn lerp( a:&Self, b:&Self, t:f32 ) -> Self {
        Self::lerp_unclamped(a, b, crate::clamp(t, 0.0, 1.0))
    }

    pub fn lerp_unclamped( a:&Self, b:&Self, t:f32 ) -> Self {
        ( *a * ( 1.0 - t ) ) + ( *b * t )
    }

    pub fn dot( a:&Self, b:&Self ) -> f32 {
        super::dot_components( a.as_array(), b.as_array() )
    }

    pub fn angle( a:&Self, b:&Self ) -> f32 {
        super::angle_components( a.as_array(), b.as_array() )
    }

    pub fn scale( a:&Self, b:&Self ) -> Self {
        let mut result = a.components.clone();
        super::componentwise_scale_components(a.as_array(), b.as_array(), &mut result);
        Self::from_array(result)
    }

    pub fn reflect( direction:&Self, normal:&Self ) -> Self {
        let mut result = direction.components.clone();
        super::reflect_components(direction.as_array(), normal.as_array(), &mut result);
        Self::from_array(result)
    }

    pub fn distance( from:&Self, to:&Self ) -> f32 {
        ( *from - *to ).magnitude()
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, 
            "Vector 4: {}, {}, {}, {}", self.x(), self.y(), self.z(), self.w()
        )
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