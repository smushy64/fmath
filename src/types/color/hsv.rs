use core::fmt::Display;
use crate::functions::{
    angles::degrees_overflow,
    max, min,
};

use super::RGB;

/// Color representation using `Hue`, `Saturation` and `Value`
/// 
/// `Hue`: **0.0**-**360.0** degrees
/// 
/// `Saturation`: **0.0**-**1.0**
/// 
/// `Value`: **0.0**-**1.0**
pub struct HSV {
    hue:f32,
    saturation:f32,
    value:f32
}

impl HSV {

    /// Create `HSV` from `RGB`
    pub fn from_rgb( color:RGB ) -> Self {
        let rgb = color.as_float_rgb_array();
        Self::from_rgb_array(&rgb)
    }

    /// Create new `HSV` from `hue`, `saturation` and `value`
    /// 
    /// `hue` is overflowed between **0.0** and **360.0**
    /// 
    /// `saturation` is clamped between **0.0** and **1.0**
    /// 
    /// `value` is clamped between **0.0** and **1.0**
    pub fn new( hue:f32, saturation:f32, value:f32 ) -> Self {
        Self {
            hue:degrees_overflow(hue),
            saturation:saturation.clamp( 0.0, 1.0),
            value:value.clamp(0.0, 1.0)
        }
    }

    /// Creates new `HSV` from **0.0**-**1.0** RGB array
    pub(crate) fn from_rgb_array( rgb:&[f32;3] ) -> Self {

        let x_max = max( rgb ); // value
        let x_min = min( rgb ); // value - chroma

        let chroma = x_max - x_min;

        let hue:f32 = {
            if chroma == 0.0 {
                0.0
            } else if x_max == rgb[0] {
                60.0 * ( 0.0 + ( ( rgb[1] - rgb[2] ) / chroma ) )
            } else if x_max == rgb[1] {
                60.0 * ( 2.0 + ( ( rgb[2] - rgb[0] ) / chroma ) )
            } else {
                60.0 * ( 4.0 + ( ( rgb[0] - rgb[1] ) / chroma ) )
            }
        };

        let saturation:f32 = {
            if x_max == 0.0 {
                0.0
            } else {
                chroma / x_max
            }
        };

        Self::new( hue, saturation, x_max )
    }

    /// Converts `HSV` to 3 component `f32` `array`
    pub(crate) fn as_rgb_array(&self) -> [f32;3] {

        let value = self.value().clone();

        let chroma = value * self.saturation();

        let hue = self.hue() / 60.0;
        let hue_index = f32::floor(hue) as i32;

        // second largest component of color
        let x = chroma * ( 1.0 - ( ( ( hue ) % 2.0 ) - 1.0 ).abs() );

        let ( r, g, b ) = {
            if hue_index < 1 {
                ( chroma, x, 0.0 )
            } else if hue_index < 2 {
                ( x, chroma, 0.0 )
            } else if hue_index < 3 {
                ( 0.0, chroma, x )
            } else if hue_index < 4 {
                ( 0.0, x, chroma )
            } else if hue_index < 5 {
                ( x, 0.0, chroma )
            } else {
                ( chroma, 0.0, x )
            }
        };

        let m = value - chroma;

        [r + m, g + m, b + m]
    }

    /// Returns: new `RGB` from `HSV`
    pub fn as_rgb(&self) -> RGB {
        RGB::from_float_array_rgb( self.as_rgb_array() )
    }

    /// Returns: `reference` to `hue` component
    pub fn hue(&self) -> &f32 {
        &self.hue
    }

    /// Returns: `reference` to `saturation` component
    pub fn saturation(&self) -> &f32 {
        &self.saturation
    }

    /// Returns: `reference` to `value` component
    pub fn value(&self) -> &f32 {
        &self.value
    }

    /// Set `hue` component
    /// 
    /// `hue` is overflowed between **0.0** and **360.0**
    pub fn set_hue(&mut self, hue:f32) {
        self.hue = degrees_overflow(hue);
    }

    /// Set `saturation` component
    /// 
    /// `saturation` is clamped between **0.0** and **1.0**
    pub fn set_saturation(&mut self, saturation:f32) {
        self.saturation = saturation.clamp(0.0, 1.0);
    }

    /// Set `value` component
    /// 
    /// `value` is clamped between **0.0** and **1.0**
    pub fn set_value(&mut self, value:f32) {
        self.value = value.clamp(0.0, 1.0);
    }

}

impl Display for HSV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HSV: Hue: {}°, Saturation: {}%, Value: {}%",
            self.hue(), self.saturation() * 100.0, self.value() * 100.0
        )
    }
}