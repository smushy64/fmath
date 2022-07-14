use core::fmt;
use core::ops::{
    Add, AddAssign,
    Sub, SubAssign,
};

use crate::structs::Vector3;

/// Unit `Vector3` and angle of revolution about that vector
/// 
/// Angle in **radians**
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AngleAxis {
    angle: f32,
    axis:  Vector3,
}

impl AngleAxis {

    /// Create new `AngleAxis` from angle in **radians** and normalized `Vector3`
    pub fn new( angle_rad:f32, axis_normalized:Vector3 ) -> Self {
        Self {
            angle: angle_rad,
            axis:  axis_normalized,
        }
    }

    /// Returns: `AngleAxis` angle component
    pub fn angle(&self) -> f32      { self.angle }
    /// Returns: `AngleAxis` axis component
    pub fn axis(&self)  -> &Vector3 { &self.axis }

    /// Returns: `AngleAxis` mut angle component
    pub fn angle_mut(&mut self) -> &mut f32     { &mut self.angle }
    /// Returns: `AngleAxis` mut axis component
    pub fn axis_mut (&mut self) -> &mut Vector3 { &mut self.axis  }

    /// Set angle component
    pub fn set_angle( &mut self, angle_rad:f32 ) { self.angle = angle_rad; }
    /// Set axis component
    pub fn set_axis(  &mut self, axis_normalized:Vector3 ) { self.axis = axis_normalized; }

    /// Clamp angle between *min* and *max*
    pub fn clamp_to( &mut self, min:f32, max:f32 ) {
        self.angle = self.angle.clamp( min, max );
    }

    /// Clamp angle between *min* and *max*
    pub fn clamp( aa:Self, min:f32, max:f32 ) -> Self {
        Self { angle: aa.angle.clamp(min, max), axis: aa.axis }
    }

}

impl fmt::Display for AngleAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "θ: {} axis: {}", self.angle(), self.axis() )
    }
}

impl Add<f32> for AngleAxis {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new( self.angle() + rhs, *self.axis() )
    }
}

impl AddAssign<f32> for AngleAxis {
    fn add_assign( &mut self, rhs:f32 ) {
        *self = *self + rhs
    }
}

impl Sub<f32> for AngleAxis {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new( self.angle() - rhs, *self.axis() )
    }
}

impl SubAssign<f32> for AngleAxis {
    fn sub_assign( &mut self, rhs:f32 ) {
        *self = *self - rhs
    }
}
