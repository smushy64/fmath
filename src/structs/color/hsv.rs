use core::fmt;
use crate::functions::degrees_overflow;
use super::RGB;

/// Color representation using `Hue`, `Saturation` and `Value`
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct HSV { hue:f32, saturation:f32, value:f32 }
impl HSV {
    /// Create new `HSV` from `hue`, `saturation` and `value`
    /// 
    /// `hue` is overflowed between **0.0** and **360.0**
    /// 
    /// `saturation` is clamped between **0.0** and **1.0**
    /// 
    /// `value` is clamped between **0.0** and **1.0**
    pub fn new( hue:f32, saturation:f32, value:f32 ) -> Self {
        Self {
            hue:        degrees_overflow(hue),
            saturation: saturation.clamp( 0.0, 1.0),
            value:      value.clamp(0.0, 1.0)
        }
    }
    /// Create new `HSV` from hsv `array`
    /// 
    /// `hue` is overflowed between **0.0** and **360.0**
    /// 
    /// `saturation` is clamped between **0.0** and **1.0**
    /// 
    /// `value` is clamped between **0.0** and **1.0**
    pub fn from_array_hsv( hsv:&[f32;3] ) -> Self {
        Self::new( hsv[0], hsv[1], hsv[2] )
    }
    /// Create `HSV` from `RGB`
    pub fn from_rgb( color:RGB ) -> Self {
        let rgb = color.as_f32_rgb();
        Self::from_array_rgb_f32(&rgb)
    }
    /// Creates new `HSV` from **0.0**-**1.0** RGB array
    pub fn from_array_rgb_f32( rgb:&[f32;3] ) -> Self {

        let x_max = rgb.clone().into_iter().reduce( f32::max ).unwrap(); // value
        let x_min = rgb.clone().into_iter().reduce( f32::min ).unwrap(); // value - chroma

        let chroma = x_max - x_min;

        let hue:f32 = if chroma == 0.0 { 0.0 }
        else if x_max == rgb[0] {
            60.0 * ( 0.0 + ( ( rgb[1] - rgb[2] ) / chroma ) )
        } else if x_max == rgb[1] {
            60.0 * ( 2.0 + ( ( rgb[2] - rgb[0] ) / chroma ) )
        } else {
            60.0 * ( 4.0 + ( ( rgb[0] - rgb[1] ) / chroma ) )
        };

        let saturation:f32 = if x_max == 0.0 { 0.0 }
            else { chroma / x_max };

        Self::new( hue, saturation, x_max )
    }
    /// Converts `HSV` to 3 component `f32` `array`
    pub fn as_array_rgb_f32(&self) -> [f32;3] {
        let value = self.value();
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
    /// Converts `HSV` to `RGB`
    pub fn to_rgb(self) -> RGB { RGB::from_f32_slice( &self.as_array_rgb_f32() ) }

    /// Returns: `hue` component
    pub fn hue(&self) -> f32 { self.hue }

    /// Returns: `saturation` component
    pub fn saturation(&self) -> f32 { self.saturation }

    /// Returns: `value` component
    pub fn value(&self) -> f32 { self.value }

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
impl fmt::Display for HSV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hue {:5.3}°, Saturation {:5.2}%, Value {:5.2}%",
            self.hue(), self.saturation() * 100.0, self.value() * 100.0
        )
    }
}
impl From<RGB> for HSV { fn from(rgb: RGB) -> Self { Self::from_rgb(rgb) } }
