#![allow(unused_imports)]
use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};

use crate::{
    FRAC_PI_2,
    types::{
        Angle,
        vector::{
            Vector3,
            magnitude_components,
            dot_components, cross_components, scale_components
        },
    },
    functions::angles::{
        radians_to_degrees,
        degrees_to_radians
    }
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quaternion {
    components:[f32;4]
}

impl Quaternion {

    pub fn new( scalar:f32, x:f32, y:f32, z:f32 ) -> Self {
        Self { components:[scalar,x,y,z] }
    }

    pub fn from_scalar_vector( scalar:f32, v:Vector3 ) -> Self {
        Self { components:[ scalar, v[0], v[1], v[2] ] }
    }

    pub fn from_euler_angles( euler:Vector3, angle_kind:Angle ) -> Self {

        let mut angles = euler.as_array().clone();

        match angle_kind {
            Angle::Degrees => {
                angles[0] = degrees_to_radians(angles[0]);
                angles[1] = degrees_to_radians(angles[1]);
                angles[2] = degrees_to_radians(angles[2]);
            },
            _ => {},
        }

        let ( x_sin, x_cos ) = ( angles[0] / 2.0 ).sin_cos();
        let ( y_sin, y_cos ) = ( angles[1] / 2.0 ).sin_cos();
        let ( z_sin, z_cos ) = ( angles[2] / 2.0 ).sin_cos();

        Self {
            components:[
                ( x_cos * y_cos * z_cos ) + ( x_sin * y_sin * z_sin ), // scalar

                ( x_sin * y_cos * z_cos ) + ( x_cos * y_sin * z_sin ), // vector x
                ( x_cos * y_sin * z_cos ) + ( x_sin * y_cos * z_sin ), // vector y
                ( x_cos * y_cos * z_sin ) + ( x_sin * y_sin * z_cos ), // vector z
            ]
        }
    }

    pub fn as_euler_angles(&self, angle_kind:Angle) -> Vector3 {
        let mut result = Vector3::from_array([
            ( 2.0 * ( self[0] * self[1] + self[2] * self[3] ) )
                .atan2( 1.0 - 2.0 * ( self[1] * self[1] + self[2] * self[2] ) ),

            Self::no_nan_asin( 2.0 * ( (self[0] * self[2]) - (self[3] * self[1]) ) ),

            ( 2.0 * (self[0] * self[3] + self[1] * self[2]) )
                .atan2( 1.0 - 2.0 * (self[2] * self[2] + self[3] * self[3]) )
        ]);

        match angle_kind {
            Angle::Degrees => {
                result[0] = radians_to_degrees(result[0]);
                result[1] = radians_to_degrees(result[1]);
                result[2] = radians_to_degrees(result[2]);
                return result
            },
            Angle::Radians => return result,
        }
    }

    pub fn as_array(&self) -> &[f32;4] {
        &self.components
    }

    pub fn sqr_magnitude(&self) -> f32 {
        ( self[0] * self[0] ) +
        ( self[1] * self[1] ) +
        ( self[2] * self[2] ) +
        ( self[3] * self[3] )
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.sqr_magnitude();
        if norm == 0.0 {
            panic!("Quaternion cannot be normalized if its square magnitude is 0.0!");
        }
        let n = 1.0 / norm.sqrt();

        self[0] = self[0] * n;
        self[1] = self[1] * n;
        self[2] = self[2] * n;
        self[3] = self[3] * n;
    }

    pub fn conjugate(&self) -> Self {
        Self {
            components:[
                self[0].clone(),
                -self[1],
                -self[2],
                -self[3],
            ]
        }
    }

    fn no_nan_asin(n:f32) -> f32 {
        if n.abs() >= 1.0 {
            if n.is_sign_positive() {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            }
        } else {
            n.asin()
        }
    }

}

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "Quaternion: scalar:{}, vector: {}, {}, {}", self[0], self[1], self[2], self[3] )
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            components:[
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
            ]
        }
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            components:[
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
            ]
        }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs:f32) -> Self {
        Self {
            components:[
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
                self[3] * rhs,
            ]
        }
    }
}

impl Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs:f32) -> Self {
        Self {
            components:[
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
                self[3] / rhs,
            ]
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, rhs:Self) -> Self {
        Self {
            components: [
                ( self[0] * rhs[0] ) - ( self[1] * rhs[1] + self[2] * rhs[2] + self[3] * rhs[3] ),

                ( self[0] * rhs[1] ) + ( rhs[0] * self[1] ) + ( ( self[2] * rhs[3] ) - ( self[3] * rhs[2] ) ),
                ( self[0] * rhs[2] ) + ( rhs[0] * self[2] ) + ( ( self[3] * rhs[1] ) - ( self[1] * rhs[3] ) ),
                ( self[0] * rhs[3] ) + ( rhs[0] * self[3] ) + ( ( self[1] * rhs[2] ) - ( self[2] * rhs[1] ) ),
            ]
        }
    }
}

impl Mul<Vector3> for Quaternion {
    type Output = Vector3;

    fn mul(self, rhs:Vector3) -> Vector3 {
        let p = Self { components:[0.0, rhs[0], rhs[1], rhs[2]] };
        let result = self * p * self.conjugate();
        Vector3::new(result[1], result[2], result[3])
    }
}

impl Index<usize> for Quaternion {
    type Output = f32;

    fn index(&self, index:usize) -> &Self::Output {
        &self.components[index]
    }
}

impl IndexMut<usize> for Quaternion {
    fn index_mut(&mut self, index:usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}