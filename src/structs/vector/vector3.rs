use core::fmt;
use core::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Neg, Not,
    Index, IndexMut
};
use super::{
    Vector2, Vector2Bool, Vector2Int, Vector2f64,
    Vector4, Vector4Bool, Vector4f64, Vector4Int,
};
use crate::{ any_as_byte_slice, structs::color::RGB };

macro_rules! impl_vector3_float {
    ($struct:ty, $float_type:ty) => {
        impl $struct {
            /// Returns: dot product between a and b
            pub fn dot_product(a:Self, b:Self) -> $float_type { (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2]) }
            /// Returns: cross product between a and b
            pub fn cross_product( a:Self, b:Self ) -> Self {
                Self::from_array([
                    ( a.y() * b.z() ) - ( a.z() * b.y() ),
                    ( a.z() * b.x() ) - ( a.x() * b.z() ),
                    ( a.x() * b.y() ) - ( a.y() * b.x() ),
                ])
            }
            /// Returns: unsigned angle between a and b in radians
            pub fn angle_between_vectors( a:Self, b:Self ) -> $float_type { Self::signed_angle_between_vectors(a, b).abs() }
            /// Returns: signed angle between a and b in radians
            pub fn signed_angle_between_vectors( a:Self, b:Self ) -> $float_type { Self::dot_product(a, b).acos() }
            /// Returns: vector with magnitude clamped to max
            pub fn clamp_vector( vector:Self, max:$float_type ) -> Self {
                let mag = vector.magnitude();
                if mag > max {
                    (vector / mag) * max
                } else { vector }
            }
            /// Returns: distance between a and b
            pub fn distance( a:Self, b:Self ) -> $float_type { ( a - b ).magnitude() }
            /// Returns: linear interpolation between a and b at point t
            /// 
            /// *t clamped between 0.0 and 1.0*
            pub fn lerp( a:Self, b:Self, t:$float_type ) -> Self { Self::lerp_unclamped( a, b, t.clamp( 0.0, 1.0 ) ) }
            /// Returns: linear interpolation between a and b at point t
            pub fn lerp_unclamped( a:Self, b:Self, t:$float_type ) -> Self { ( a * ( 1.0 - t ) ) + ( b * t ) }
            /// Returns: direction reflected off of given normal
            pub fn reflect_vector( direction:Self, normal:Self ) -> Self {
                ( normal - direction ) * ( 2.0 * ( direction.dot( normal ) ) )
            }

            /// Get the squared length of this vector
            pub fn sqr_magnitude(&self) -> $float_type { self[0].powi(2) + self[1].powi(2) + self[2].powi(2) }
            /// Get the length of this vector
            pub fn magnitude(&self) -> $float_type { self.sqr_magnitude().sqrt() }
            /// Returns: this vector with a magnitude of 1
            pub fn normalized(&self) -> Self {
                let mag = self.magnitude();
                if mag.abs() <= 0.00001 { return Self::new_zero() }
                *self / mag
            }
            /// Returns: dot product between this vector and other vector
            pub fn dot(&self, other:Self) -> $float_type { Self::dot_product( *self, other ) }
            /// Returns: cross product between this vector and other vector
            pub fn cross( &self, other:Self ) -> Self {
                Self::cross_product( *self, other )
            }
            /// Returns: unsigned angle between this vector and other vector in radians
            pub fn angle( &self, other:Self ) -> $float_type { Self::angle_between_vectors(*self, other) }
            /// Returns: signed angle between this vector and other vector in radians
            pub fn signed_angle( &self, other:Self ) -> $float_type { Self::signed_angle_between_vectors(*self, other) }
            /// Returns: distance between this vector and other
            pub fn distance_to( &self, other:Self ) -> $float_type { Self::distance( *self, other ) }
            /// Returns: linear interpolation between this vector and other at point t
            /// 
            /// *t clamped between 0.0 and 1.0*
            pub fn lerp_to( &self, other:Self, t:$float_type ) -> Self { Self::lerp( *self, other, t ) }
            /// Returns: linear interpolation between this vector and other at point t
            pub fn lerp_unclamped_to( &self, other:Self, t:$float_type ) -> Self { Self::lerp_unclamped( *self, other, t ) }
            
            /// Clamp this vector's magnitude to given max
            pub fn clamp_magnitude(&mut self, max:$float_type) { *self = Self::clamp_vector( *self, max ); }
            /// Normalize this vector
            pub fn normalize(&mut self) { *self = self.normalized(); }
            /// Reflect this vector off given normal
            pub fn reflect(&mut self, normal:Self) {
                *self = Self::reflect_vector( *self, normal );
            }
        }
        impl Add for $struct {
            type Output = Self;
            fn add(self, rhs:Self) -> Self {
                Self {
                    components: [
                        self[0] + rhs[0],
                        self[1] + rhs[1],
                        self[2] + rhs[2],
                    ]
                }
            }
        }
        impl Sub for $struct {
            type Output = Self;
            fn sub(self, rhs:Self) -> Self {
                Self {
                    components: [
                        self[0] - rhs[0],
                        self[1] - rhs[1],
                        self[2] - rhs[2],
                    ]
                }
            }
        }
        impl Mul<f32> for $struct {
            type Output = Self;
            fn mul( self, rhs: f32 ) -> Self {
                Self::new( self[0] * (rhs as $float_type), self[1] * (rhs as $float_type), self[2] * (rhs as $float_type) )
            }
        }
        impl Mul<f64> for $struct {
            type Output = Self;
            fn mul( self, rhs: f64 ) -> Self {
                Self::new( self[0] * (rhs as $float_type), self[1] * (rhs as $float_type), self[2] * (rhs as $float_type) )
            }
        }
        impl Div<f32> for $struct {
            type Output = Self;
            fn div( self, rhs: f32 ) -> Self {
                Self::new( self[0] / (rhs as $float_type), self[1] / (rhs as $float_type), self[2] / (rhs as $float_type) )
            }
        }
        impl Div<f64> for $struct {
            type Output = Self;
            fn div( self, rhs: f64 ) -> Self {
                Self::new( self[0] / (rhs as $float_type), self[1] / (rhs as $float_type), self[2] / (rhs as $float_type) )
            }
        }
        impl MulAssign<f32> for $struct { fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; } }
        impl MulAssign<f64> for $struct { fn mul_assign(&mut self, rhs: f64) { *self = *self * rhs; } }
        impl DivAssign<f32> for $struct { fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; } }
        impl DivAssign<f64> for $struct { fn div_assign(&mut self, rhs: f64) { *self = *self / rhs; } }
        impl Mul<$struct> for $struct {
            type Output = Self;
            fn mul( self, rhs: Self ) -> Self {
                Self::new( self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2] )
            }
        }
        impl Div<$struct> for $struct {
            type Output = Self;
            fn div( self, rhs: Self ) -> Self {
                Self::new( self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2] )
            }
        }
        impl MulAssign<$struct> for $struct { fn mul_assign(&mut self, rhs:Self) { *self = *self * rhs; } }
        impl DivAssign<$struct> for $struct { fn div_assign(&mut self, rhs:Self) { *self = *self / rhs; } }
        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!( f, "( {:7.3}, {:7.3}, {:7.3} )", self[0], self[1], self[2] )
            }
        }
    };
}
macro_rules! impl_vector3_number {
    ($struct:ty, $float_type:ty) => {
        impl $struct {
            /// Create vector with x, y and z set to 1
            pub fn new_one() -> Self { Self::new( 1 as $float_type, 1 as $float_type, 1 as $float_type ) }
            /// Create vector with x, y and z set to 0
            pub fn new_zero() -> Self { Self::new( 0 as $float_type, 0 as $float_type, 0 as $float_type ) }
            
            /// Create vector with y set to 1
            pub fn new_up() -> Self { Self::new( 0 as $float_type, 1 as $float_type, 0 as $float_type ) }
            /// Create vector with y set to -1
            pub fn new_down() -> Self { Self::new( 0 as $float_type, -1 as $float_type, 0 as $float_type ) }
            /// Create vector with x set to 1
            pub fn new_right() -> Self { Self::new( 1 as $float_type, 0 as $float_type, 0 as $float_type ) }
            /// Create vector with x set to -1
            pub fn new_left() -> Self { Self::new( -1 as $float_type, 0 as $float_type, 0 as $float_type ) }
            /// Create vector with z set to 1
            pub fn new_forward() -> Self { Self::new( 0 as $float_type, 0 as $float_type, 1 as $float_type ) }
            /// Create vector with z set to -1
            pub fn new_back() -> Self { Self::new( 0 as $float_type, 0 as $float_type, -1 as $float_type ) }

            /// Create vector from largest components of two vectors
            pub fn max( lhs:Self, rhs:Self ) -> Self {
                Self::new(
                    if lhs[0] > rhs[0] { lhs[0] } else { rhs[0] },
                    if lhs[1] > rhs[1] { lhs[1] } else { rhs[1] },
                    if lhs[2] > rhs[2] { lhs[2] } else { rhs[2] },
                )
            }
            /// Create vector from smallest components of two vectors
            pub fn min( lhs:Self, rhs:Self ) -> Self {
                Self::new(
                    if lhs[0] < rhs[0] { lhs[0] } else { rhs[0] },
                    if lhs[1] < rhs[1] { lhs[1] } else { rhs[1] },
                    if lhs[2] < rhs[2] { lhs[2] } else { rhs[2] },
                )
            }
        }
        impl AddAssign for $struct {
            fn add_assign(&mut self, rhs:Self) {
                *self = *self + rhs;
            }
        }
        impl SubAssign for $struct {
            fn sub_assign(&mut self, rhs:Self) {
                *self = *self - rhs;
            }
        }
        impl Neg for $struct {
            type Output = Self;
            fn neg(self) -> Self {
                Self { components: [ -self[0], -self[1], -self[2], ] }
            }
        }
    };
}
macro_rules! impl_vector3_common {
    ($struct:ty, $type:ty) => {
        impl $struct {
            /// Create new vector
            pub fn new( x:$type, y:$type, z:$type ) -> Self { Self{components: [x, y, z]} }
            /// Create new vector from array
            pub fn from_array( array:[$type;3] ) -> Self { Self{ components: array } }
            /// Create new vector from slice
            pub fn from_slice( slice:&[$type] ) -> Self {
                let mut components = [Default::default();3];
                for ( component, element ) in components.iter_mut().zip( slice.iter() ) {
                    *component = *element;
                }
                Self{ components }
            }

            /// Get x component
            pub fn x(&self) -> $type { self.components[0] }
            /// Get y component
            pub fn y(&self) -> $type { self.components[1] }
            /// Get z component
            pub fn z(&self) -> $type { self.components[2] }
            /// Returns true if components are equal
            pub fn equals(&self, other:$struct) -> bool { self[0] == other[0] && self[1] == other[1] && self[2] == other[2] }

            /// Set x, y and z components
            pub fn set( &mut self, x:$type, y:$type, z:$type ) { self[0] = x; self[1] = y; self[2] = z; }

            /// Get component slice
            pub fn as_slice( &self ) -> &[$type] { &self.components }
            /// Get mutable component slice
            pub fn as_mut_slice( &mut self ) -> &mut [$type] { &mut self.components }
            /// Get pointer to components
            pub fn as_ptr( &self ) -> *const $type { self.components.as_ptr() }
            /// Get mutable component slice
            pub fn as_mut_ptr( &mut self ) -> *mut $type { self.components.as_mut_ptr() }
            /// Get vector as byte slice
            pub fn as_bytes(&self) -> &[u8] { unsafe { any_as_byte_slice( self ) } }
            /// Get vector as string
            pub fn as_string(&self) -> String { format!( "{}", self ) }
        }
        impl Index<usize> for $struct {
            type Output = $type;
            fn index(&self, index: usize) -> &Self::Output {
                &self.components[index]
            }
        }
        impl IndexMut<usize> for $struct {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.components[index]
            }
        }
    };
}

/// bool 3-component Vector
/// 
/// Indexable with **[]**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Vector3Bool { components:[bool;3] }
impl Vector3Bool {
    /// Create vector with components set to true
    pub fn new_true() -> Self { Self::new( true, true, true ) }
    /// Create vector with components set to false
    pub fn new_false() -> Self { Self::new( false, false, false ) }
    /// Convert to int bool
    pub fn to_int_bool(self) -> [u32;3] { [self[0] as u32, self[1] as u32, self[2] as u32] }
    /// Convert to char bool
    pub fn to_char_bool(self) -> [u8;3] { [self[0] as u8, self[1] as u8, self[2] as u8] }
}
impl Not for Vector3Bool {
    type Output = Self;
    fn not(self) -> Self { Self::new( !self[0], !self[1], !self[2] ) }
}
impl fmt::Display for Vector3Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "( {}, {}, {} )", self[0], self[1], self[2] )
    }
}
impl_vector3_common!(Vector3Bool, bool);
impl From<Vector3Int> for Vector3Bool {
    fn from(v: Vector3Int) -> Self { Self::new( v[0] != 0, v[1] != 0, v[2] != 0 ) }
}
impl From<Vector2Bool> for Vector3Bool {
    fn from(v: Vector2Bool) -> Self { Self::new( v[0], v[1], false ) }
}
impl From<Vector4Bool> for Vector3Bool {
    fn from(v: Vector4Bool) -> Self { Self::new( v[0], v[1], v[2] ) }
}

/// i32 3-component Vector
/// 
/// Indexable with **[]**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Vector3Int { components:[i32;3] }
impl Vector3Int {
    /// Get the squared length of this vector
    pub fn sqr_magnitude(&self) -> i32 { self[0].pow(2) + self[1].pow(2) + self[2].pow(2) }
    /// Get the length of this vector
    pub fn magnitude(&self) -> f32 { (self.sqr_magnitude() as f32).sqrt() }
    /// Clamp vector to the bounds provided
    pub fn clamp( &mut self, min:Self, max:Self ) {
        self[0] = self[0].clamp( min[0], max[0] );
        self[1] = self[1].clamp( min[1], max[1] );
        self[2] = self[2].clamp( min[2], max[2] );
    }
    /// Convert f32 vector by ceiling components
    pub fn from_vector3_f32_ceil( v:Vector3 ) -> Self {
        Self::new( v[0].ceil() as i32, v[1].ceil() as i32, v[2].ceil() as i32 )
    }
    /// Convert f64 vector by ceiling components
    pub fn from_vector3_f64_ceil( v:Vector3f64 ) -> Self {
        Self::new( v[0].ceil() as i32, v[1].ceil() as i32, v[2].ceil() as i32 )
    }
    /// Convert f32 vector by flooring components
    pub fn from_vector3_f32_floor( v:Vector3 ) -> Self {
        Self::new( v[0].floor() as i32, v[1].floor() as i32, v[2].floor() as i32 )
    }
    /// Convert f64 vector by flooring components
    pub fn from_vector3_f64_floor( v:Vector3f64 ) -> Self {
        Self::new( v[0].floor() as i32, v[1].floor() as i32, v[2].floor() as i32 )
    }
    /// Convert f32 vector by rounding components
    pub fn from_vector3_f32_round( v:Vector3 ) -> Self {
        Self::new( v[0].round() as i32, v[1].round() as i32, v[2].round() as i32 )
    }
    /// Convert f64 vector by rounding components
    pub fn from_vector3_f64_round( v:Vector3f64 ) -> Self {
        Self::new( v[0].round() as i32, v[1].round() as i32, v[2].round() as i32 )
    }
    /// Convert to u8 array
    pub fn to_u8_array(self) -> [u8;3] {
        [self[0] as u8, self[1] as u8, self[2] as u8]
    }
    /// Convert to i8 array
    pub fn to_i8_array(self) -> [i8;3] {
        [self[0] as i8, self[1] as i8, self[2] as i8]
    }
    /// Convert to u16 array
    pub fn to_u16_array(self) -> [u16;3] {
        [self[0] as u16, self[1] as u16, self[2] as u16]
    }
    /// Convert to i16 array
    pub fn to_i16_array(self) -> [i16;3] {
        [self[0] as i16, self[1] as i16, self[2] as i16]
    }
    /// Convert to u32 array
    pub fn to_u32_array(self) -> [u32;3] {
        [self[0] as u32, self[1] as u32, self[2] as u32]
    }
}
impl Add for Vector3Int {
    type Output = Self;
    fn add(self, rhs:Self) -> Self {
        Self::new(
            self[0].saturating_add( rhs[0] ),
            self[1].saturating_add( rhs[1] ),
            self[2].saturating_add( rhs[2] ),
        )
    }
}
impl Sub for Vector3Int {
    type Output = Self;
    fn sub(self, rhs:Self) -> Self {
        Self::new(
            self[0].saturating_sub( rhs[0] ),
            self[1].saturating_sub( rhs[1] ),
            self[2].saturating_sub( rhs[2] ),
        )
    }
}
impl Mul<i32> for Vector3Int {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self::new(
            self[0].saturating_mul(rhs),
            self[1].saturating_mul(rhs),
            self[2].saturating_mul(rhs)
        )
    }
}
impl Div<i32> for Vector3Int {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Self::new(
            self[0].saturating_div(rhs),
            self[1].saturating_div(rhs),
            self[2].saturating_div(rhs)
        )
    }
}
impl MulAssign<i32> for Vector3Int {
    fn mul_assign(&mut self, rhs: i32) { *self = *self * rhs }
}
impl DivAssign<i32> for Vector3Int {
    fn div_assign(&mut self, rhs: i32) { *self = *self / rhs }
}
impl Mul<Vector3Int> for Vector3Int {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self[0].saturating_mul(rhs[0]),
            self[1].saturating_mul(rhs[1]),
            self[2].saturating_mul(rhs[2]),
        )
    }
}
impl Div<Vector3Int> for Vector3Int {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new(
            self[0].saturating_div(rhs[0]),
            self[1].saturating_div(rhs[1]),
            self[2].saturating_div(rhs[2]),
        )
    }
}
impl MulAssign<Vector3Int> for Vector3Int {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign<Vector3Int> for Vector3Int {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl fmt::Display for Vector3Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "( {:12}, {:12}, {:12} )", self[0], self[1], self[2] )
    }
}
impl_vector3_number!(Vector3Int, i32);
impl_vector3_common!(Vector3Int, i32);
impl From<Vector3> for Vector3Int {
    fn from( v:Vector3 ) -> Self { Self::new( v[0] as i32, v[1] as i32, v[2] as i32 ) }
}
impl From<Vector3f64> for Vector3Int {
    fn from( v:Vector3f64 ) -> Self { Self::new( v[0] as i32, v[1] as i32, v[2] as i32 ) }
}
impl From<Vector3Bool> for Vector3Int {
    fn from(v: Vector3Bool) -> Self { Self::new( v[0] as i32, v[1] as i32, v[2] as i32 ) }
}
impl From<Vector2Int> for Vector3Int {
    fn from(v: Vector2Int) -> Self { Self::new( v[0], v[1], 0 ) }
}
impl From<Vector4Int> for Vector3Int {
    fn from(v: Vector4Int) -> Self { Self::new( v[0], v[1], v[2] ) }
}
impl From<RGB> for Vector3Int {
    fn from(v: RGB) -> Self {
        let v_u8 = v.as_bytes_rgb();
        Self::from_slice( &v_u8.into_iter().map( |x| x as i32 ).collect::<Vec<i32>>() )
    }
}

/// f32 3-component Vector
/// 
/// Indexable with **[]**
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector3 { components:[f32;3] }
impl Vector3 {
    /// Create new vector with components set to positive infinity
    pub fn new_positive_infinity() -> Self {
        Self::new( core::f32::INFINITY, core::f32::INFINITY, core::f32::INFINITY )
    }
    /// Create new vector with components set to negative infinity
    pub fn new_negative_infinity() -> Self {
        Self::new( core::f32::NEG_INFINITY, core::f32::NEG_INFINITY, core::f32::NEG_INFINITY )
    }
}
impl_vector3_float!(Vector3, f32);
impl_vector3_number!(Vector3, f32);
impl_vector3_common!(Vector3, f32);
impl From<Vector3Int> for Vector3 {
    fn from(v: Vector3Int) -> Self { Self::new( v[0] as f32, v[1] as f32, v[2] as f32 ) }
}
impl From<Vector3f64> for Vector3 {
    fn from(v: Vector3f64) -> Self { Self::new( v[0] as f32, v[1] as f32, v[2] as f32 ) }
}
impl From<Vector2> for Vector3 {
    fn from(v: Vector2) -> Self { Self::new( v[0], v[1], 0f32 ) }
}
impl From<Vector2f64> for Vector3 {
    fn from(v: Vector2f64) -> Self { Self::new( v[0] as f32, v[1] as f32, 0f32 ) }
}
impl From<Vector4> for Vector3 {
    fn from(v: Vector4) -> Self { Self::new( v[0], v[1], v[2] ) }
}
impl From<Vector4f64> for Vector3 {
    fn from(v: Vector4f64) -> Self { Self::new( v[0] as f32, v[1] as f32, v[2] as f32 ) }
}
impl From<RGB> for Vector3 {
    fn from(v: RGB) -> Self { Self::from_array( v.as_f32_rgb() ) }
}

/// f64 3-component Vector
/// 
/// Indexable with **[]**
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector3f64 { components:[f64;3] }
impl Vector3f64 {
    /// Create new vector with components set to positive infinity
    pub fn new_positive_infinity() -> Self {
        Self::new( core::f64::INFINITY, core::f64::INFINITY, core::f64::INFINITY )
    }
    /// Create new vector with components set to negative infinity
    pub fn new_negative_infinity() -> Self {
        Self::new( core::f64::NEG_INFINITY, core::f64::NEG_INFINITY, core::f64::NEG_INFINITY )
    }
}
impl_vector3_float!(Vector3f64, f64);
impl_vector3_number!(Vector3f64, f64);
impl_vector3_common!(Vector3f64, f64);
impl From<Vector3Int> for Vector3f64 {
    fn from(v: Vector3Int) -> Self { Self::new( v[0] as f64, v[1] as f64, v[2] as f64 ) }
}
impl From<Vector3> for Vector3f64 {
    fn from(v: Vector3) -> Self { Self::new( v[0] as f64, v[1] as f64, v[2] as f64 ) }
}
impl From<Vector2> for Vector3f64 {
    fn from(v: Vector2) -> Self { Self::new( v[0] as f64, v[1] as f64, 0f64 ) }
}
impl From<Vector2f64> for Vector3f64 {
    fn from(v: Vector2f64) -> Self { Self::new( v[0], v[1], 0f64 ) }
}
impl From<Vector4> for Vector3f64 {
    fn from(v: Vector4) -> Self { Self::new( v[0] as f64, v[1] as f64, v[2] as f64 ) }
}
impl From<Vector4f64> for Vector3f64 {
    fn from(v: Vector4f64) -> Self { Self::new( v[0], v[1], v[2] ) }
}
impl From<RGB> for Vector3f64 {
    fn from(v: RGB) -> Self { Vector3::from_array( v.as_f32_rgb() ).into() }
}
