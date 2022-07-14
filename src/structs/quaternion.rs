use core::fmt;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut, Neg
};
use crate::{
    FRAC_PI_2,
    structs::{ vector::Vector3, AngleAxis },
};

/// 4D representation of rotations
/// 
/// Compact way to represent rotations without gimbal lock.
/// 
/// `w` `x` `y` `z`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quaternion {
    components:[f32;4]
}
impl Quaternion {
    /// Create new `Quaternion`
    pub fn new( w:f32, x:f32, y:f32, z:f32 ) -> Self { Self { components:[w,x,y,z] } }
    /// Create new identity `Quaternion`
    pub fn new_identity() -> Self { Self::new( 1.0, 0.0, 0.0, 0.0 ) }
    /// Create new `Quaternion` from given `scalar` and `Vector3`
    pub fn from_scalar_vector( scalar:f32, v:Vector3 ) -> Self { Self::new( scalar, v[0], v[1], v[2] ) }
    /// Create new `Quaternion` from float array
    /// 
    /// `0` = scalar
    /// 
    /// `1` `2` `3` = vector
    pub fn from_array( components:[f32;4] ) -> Self { Self { components } }
    /// Returns: new `Quaternion` from given euler angles ( *radians* ), `Vector3`
    pub fn from_euler_angles( euler:Vector3 ) -> Self {

        let ( x_sin, x_cos ) = ( euler[0] / 2.0 ).sin_cos();
        let ( y_sin, y_cos ) = ( euler[1] / 2.0 ).sin_cos();
        let ( z_sin, z_cos ) = ( euler[2] / 2.0 ).sin_cos();

        Self {
            components:[
                ( x_cos * y_cos * z_cos ) + ( x_sin * y_sin * z_sin ), // scalar

                ( x_sin * y_cos * z_cos ) + ( x_cos * y_sin * z_sin ), // vector x
                ( x_cos * y_sin * z_cos ) + ( x_sin * y_cos * z_sin ), // vector y
                ( x_cos * y_cos * z_sin ) + ( x_sin * y_sin * z_cos ), // vector z
            ]
        }
    }
    /// Returns: new `Quaternion` from `AngleAxis`
    pub fn from_angle_axis( angle_axis: AngleAxis ) -> Self {
        let s = ( angle_axis.angle() / 2.0 ).sin();
        Self {
            components:[
                ( angle_axis.angle() / 2.0 ).cos(),
                angle_axis.axis()[0] * s,
                angle_axis.axis()[1] * s,
                angle_axis.axis()[2] * s,
            ]
        }
    }
    /// Returns: new `AngleAxis` from `Quaternion`
    pub fn as_angle_axis(&self) -> AngleAxis {
        AngleAxis::new(
            2.0 * self[0].acos(),
            Vector3::new(
                self[1] / ( 1.0 - ( self[0] * self[0] ) ).sqrt(),
                self[2] / ( 1.0 - ( self[0] * self[0] ) ).sqrt(),
                self[3] / ( 1.0 - ( self[0] * self[0] ) ).sqrt(),
            )
        )
    }
    /// Returns: `Quaternion` rotation as *euler angles* in *radians*
    /// 
    /// **Note: euler angle representation of a quaternion is inconsistent as
    /// there is more than one way to represent a rotation on each axis**
    pub fn as_euler_angles(&self) -> Vector3 {
        Vector3::new(
            ( 2.0 * ( self[0] * self[1] + self[2] * self[3] ) )
                .atan2( 1.0 - 2.0 * ( self[1] * self[1] + self[2] * self[2] ) ),

            Self::no_nan_asin( 2.0 * ( (self[0] * self[2]) - (self[3] * self[1]) ) ),

            ( 2.0 * (self[0] * self[3] + self[1] * self[2]) )
                .atan2( 1.0 - 2.0 * (self[2] * self[2] + self[3] * self[3]) )
        )
    }
    /// Get `Quaternion` as float slice
    /// 
    /// `0` = scalar
    /// 
    /// `1` `2` `3` = vector
    pub fn as_array(&self) -> &[f32] { &self.components }
    /// Returns: `Quaternion` length without applying square root
    /// 
    /// alias: `sqr_norm`
    pub fn sqr_magnitude(&self) -> f32 { self.sqr_norm() }
    /// Returns: `Quaternion` norm without applying square root
    /// 
    /// alias: `sqr_magnitude`
    pub fn sqr_norm(&self) -> f32 {
        ( self[0] * self[0] ) +
        ( self[1] * self[1] ) +
        ( self[2] * self[2] ) +
        ( self[3] * self[3] )
    }
    /// Returns: length of `Quaternion`
    /// 
    /// alias: `norm`
    pub fn magnitude(&self) -> f32 { self.sqr_magnitude().sqrt() }
    /// Returns: norm of `Quaternion`
    /// 
    /// alias: `magnitude`
    pub fn norm(&self) -> f32 { self.sqr_norm().sqrt() }
    /// Returns: new normalized `Quaternion`
    /// 
    /// alias: `versor`
    pub fn normalized(&self) -> Self { *self / self.magnitude() }
    /// Returns: versor `Quaternion`
    /// 
    /// alias: `normal`
    pub fn versor(&self) -> Self { self.normalized() }
    /// Normalize `Quaternion`
    pub fn normalize( &mut self ) { *self = self.normalized(); }
    /// Returns: new `Quaternion` with the same scalar but the sign of the imaginary components flipped
    pub fn conjugate(&self) -> Self { Self::new( self[0], -self[1], -self[2], -self[3] ) }
    /// Returns: inverse of `Quaternion`
    pub fn inverse(&self) -> Self { self.conjugate() / self.magnitude().powi(2) }
    /// Dot product of `a` and `b`
    pub fn dot_product( a:Self, b:Self ) -> f32 {
        a.as_array().iter()
            .zip( b.as_array().iter() )
            .map( |(a, b)| a * b )
            .sum()
    }
    /// Dot product of `self` and `b`
    pub fn dot( &self, b:Self ) -> f32 { Self::dot_product( *self, b ) }
    /// Spherically interpolate from `self` to `b`
    /// 
    /// Clamps `t` between **0.0** and **1.0**
    pub fn slerp_to( &mut self, b:Self, t:f32 ) { *self = Self::slerp( *self, b, t ); }
    /// Spherically interpolate from `self` to `b`
    /// 
    /// Does **not** clamp `t`
    pub fn slerp_to_unclamped( &mut self, b:Self, t:f32 ) { *self = Self::slerp_unclamped( *self, b, t ); }
    /// Spherically interpolate from `a` to `b`
    /// 
    /// Clamps `t` between **0.0** and **1.0**
    pub fn slerp( a:Self, b:Self, t:f32 ) -> Self { Self::slerp_unclamped( a, b, t.clamp( 0.0, 1.0 ) ) }
    /// Spherically interpolate from `a` to `b`
    /// 
    /// Does **not** clamp `t`
    pub fn slerp_unclamped( a:Self, b:Self, t:f32 ) -> Self {
        let dot_prod = Self::dot_product( a, b );
        let lambda = t / 2.0;
        let theta = {
            let acos = dot_prod.acos();
            if acos < 0.0 { -acos }
            else { acos }
        };

        let theta_sin = theta.sin();
        let coeff1 = ( ( 1.0 - lambda ) * theta ).sin() /theta_sin;
        let coeff2 = ( lambda * theta ).sin() /theta_sin;

        let result = Self::new(
            coeff1 * a[0] + coeff2 * b[0],
            coeff1 * a[1] + coeff2 * b[1],
            coeff1 * a[2] + coeff2 * b[2],
            coeff1 * a[3] + coeff2 * b[3],
        );

        result.normalized()
    }

    fn no_nan_asin(n:f32) -> f32 {
        if n.abs() >= 1.0 {
            if n.is_sign_positive() { FRAC_PI_2 }
            else { -FRAC_PI_2 }
        } else { n.asin() }
    }

    /// Add a and b
    pub fn sum( a:Self, b:Self ) -> Self {
        Self::new(
            a[0] + b[0],
            a[1] + b[1],
            a[2] + b[2],
            a[3] + b[3],
        )
    }
    /// Subtract a and b
    pub fn subtract( a:Self, b:Self ) -> Self {
        Self::new(
            a[0] - b[0],
            a[1] - b[1],
            a[2] - b[2],
            a[3] - b[3],
        )
    }
    /// Scale a by scalar
    pub fn scale( a:Self, scalar:f32 ) -> Self {
        Self::new(
            a[0] * scalar,
            a[1] * scalar,
            a[2] * scalar,
            a[3] * scalar,
        )
    }
    /// Negate given quaternion
    pub fn negate( quaternion:Self ) -> Self { quaternion * -1f32 }
    /// Rotate quaternion with quaternion
    pub fn rotate_quaternion( quaternion:Self, rotator:Self ) -> Self {
        Self::new(
                ( quaternion[0] * rotator[0] ) -
                    ( quaternion[1] * rotator[1] + quaternion[2] * rotator[2] + quaternion[3] * rotator[3] ),
                ( quaternion[0] * rotator[1] ) +
                    ( rotator[0] * quaternion[1] ) +
                    ( ( quaternion[2] * rotator[3] ) -
                    ( quaternion[3] * rotator[2] ) ),
                ( quaternion[0] * rotator[2] ) +
                    ( rotator[0] * quaternion[2] ) +
                    ( ( quaternion[3] * rotator[1] ) -
                    ( quaternion[1] * rotator[3] ) ),
                ( quaternion[0] * rotator[3] ) +
                    ( rotator[0] * quaternion[3] ) +
                    ( ( quaternion[1] * rotator[2] ) -
                    ( quaternion[2] * rotator[1] ) ),
        )
    }
    /// Rotate vector with given quaternion
    pub fn rotate_vector( quaternion:Self, vector:Vector3 ) -> Vector3 {
        let p = Self::new( 0.0, vector[0], vector[1], vector[2] );
        let result = quaternion * p * quaternion.conjugate();
        Vector3::new( result[1], result[2], result[3] )
    }
}
impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "( {:7.3} {:7.3}i {:7.3}j {:7.3}k )", self[0], self[1], self[2], self[3] )
    }
}
impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self::Output { Self::negate(self) }
}
impl Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self::sum( self, rhs ) }
}
impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self::subtract(self, rhs) }
}
impl Mul<f32> for Quaternion {
    type Output = Self;
    fn mul(self, rhs:f32) -> Self { Self::scale( self, rhs ) }
}
impl Div<f32> for Quaternion {
    type Output = Self;
    fn div(self, rhs:f32) -> Self { Self::scale( self, 1f32 / rhs ) }
}
impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs:Self) -> Self { Self::rotate_quaternion(self, rhs) }
}
impl Mul<Vector3> for Quaternion {
    type Output = Vector3;
    fn mul(self, rhs:Vector3) -> Vector3 { Self::rotate_vector(self, rhs) }
}
impl Index<usize> for Quaternion {
    type Output = f32;
    fn index(&self, index:usize) -> &Self::Output { &self.components[index] }
}
impl IndexMut<usize> for Quaternion {
    fn index_mut(&mut self, index:usize) -> &mut Self::Output { &mut self.components[index] }
}
impl Default for Quaternion { fn default() -> Self { Self::new_identity() } }
