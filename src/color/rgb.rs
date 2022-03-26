use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};
use super::{
    color_byte_to_color_float,
    color_float_to_color_byte,
    ColorRGB8,
    ColorRGBA8
};
use crate::{
    hexadecimal::{
        decode_hex_str,
        encode_hex,
    },
    vector::Vector4,
};

/// Color represented as a 3 `f32` `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorRGB {
    data:[f32;3]
}

impl ColorRGB {
    /// Create `ColorRGB` from `r`, `g` and `b`
    pub fn new( r:f32, g:f32, b:f32 ) -> Self {
        Self { data:[r,g,b] }
    }

    /// Create `ColorRGB` from 3 `f32` `array`
    pub fn from_array( rgb:[f32;3] ) -> Self {
        Self { data:rgb }
    }

    /// Create `ColorRGB` from 3 byte `array`
    pub fn from_array_8( rgb:[u8;3] ) -> Self {
        Self {
            data: [
                color_byte_to_color_float(rgb[0]),
                color_byte_to_color_float(rgb[1]),
                color_byte_to_color_float(rgb[2]),
            ]
        }
    }

    /// Create `ColorRGB` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `ColorRGB` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGB ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array_8([bytes[0],bytes[1],bytes[2]]) )
    }

    /// Returns: `reference` to `ColorRGB`'s data `array`
    pub fn as_array(&self) -> &[f32;3] {
        &self.data
    }

    /// Returns: new `array` with data as bytes instead of `f32`
    pub fn as_array_8(&self) -> [u8;3] {
        [
            color_float_to_color_byte( self.data[0] ),
            color_float_to_color_byte( self.data[1] ),
            color_float_to_color_byte( self.data[2] )
        ]
    }

    /// Returns: `mutable reference` to `ColorRGB`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;3] {
        &mut self.data
    }

    /// Returns: `ColorRGB` as *hexadecimal* `String`
    pub fn as_hex_string(&self) -> String {
        encode_hex(&self.as_array_8())
    }

    /// Returns: `reference` to `r` component
    pub fn r(&self) -> &f32 {
        &self.data[0]
    }

    /// Returns: `reference` to `g` component
    pub fn g(&self) -> &f32 {
        &self.data[1]
    }

    /// Returns: `reference` to `b` component
    pub fn b(&self) -> &f32 {
        &self.data[2]
    }

    /// Returns: `mutable reference` to `r` component
    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.data[0]
    }

    /// Returns: `mutable reference` to `g` component
    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.data[1]
    }

    /// Returns: `mutable reference` to `b` component
    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.data[2]
    }

    /// Set `r`, `g` and `b` components
    pub fn set(&mut self, r:f32, g:f32, b:f32) {
        self.data = [r,g,b];
    }

    /// Set components to given `array`
    pub fn set_array(&mut self, rgb:[f32;3]) {
        self.data = rgb;
    }
}

impl Display for ColorRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorRGB: {}, {}, {}",
            self.r(), self.g(), self.b()
        )
    }
}

impl From<Vector4> for ColorRGB {
    fn from(v: Vector4) -> Self {
        Self::from_array([ v[0], v[1], v[2] ])
    }
}

impl From<ColorRGB8> for ColorRGB {
    fn from(c:ColorRGB8) -> Self {
        Self::from_array_8(c.as_array().clone())
    }
}

impl From<ColorRGBA> for ColorRGB {
    fn from(c: ColorRGBA) -> Self {
        Self::from_array([ c[0], c[1], c[2] ])
    }
}

impl From<ColorRGBA8> for ColorRGB {
    fn from(c: ColorRGBA8) -> Self {
        Self::from_array_8([ c[0], c[1], c[2] ])
    }
}

impl Index<usize> for ColorRGB {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for ColorRGB {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Add for ColorRGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
            ]
        }
    }
}

impl Sub for ColorRGB {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
            ]
        }
    }
}

impl Mul for ColorRGB {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2],
            ]
        }
    }
}

impl Div for ColorRGB {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] / rhs[0],
                self[1] / rhs[1],
                self[2] / rhs[2],
            ]
        }
    }
}

/// Color represented as a 4 `f32` `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorRGBA {
    data:[f32;4]
}

impl ColorRGBA {
    /// Create `ColorRGBA` from `r`, `g`, `b` and `a`
    pub fn new( r:f32, g:f32, b:f32, a:f32 ) -> Self {
        Self { data:[r,g,b,a] }
    }

    /// Create `ColorRGBA` from 4 `f32` `array`
    pub fn from_array( rgba:[f32;4] ) -> Self {
        Self { data:rgba }
    }

    /// Create `ColorRGBA` from 3 byte `array`
    pub fn from_array_8( rgba:[u8;4] ) -> Self {
        Self {
            data: [
                color_byte_to_color_float(rgba[0]),
                color_byte_to_color_float(rgba[1]),
                color_byte_to_color_float(rgba[2]),
                color_byte_to_color_float(rgba[3]),
            ]
        }
    }

    /// Create `ColorRGBA` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `ColorRGBA` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGBA ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array_8([bytes[0],bytes[1],bytes[2], 255]) )
    }

    /// Returns: `reference` to `ColorRGBA`'s data `array`
    pub fn as_array(&self) -> &[f32;4] {
        &self.data
    }

    /// Returns: new `array` with data as bytes instead of `f32`
    pub fn as_array_8(&self) -> [u8;4] {
        [
            color_float_to_color_byte( self.data[0] ),
            color_float_to_color_byte( self.data[1] ),
            color_float_to_color_byte( self.data[2] ),
            color_float_to_color_byte( self.data[3] )
        ]
    }

    /// Returns: `mutable reference` to `ColorRGBA`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;4] {
        &mut self.data
    }

    /// Returns: `ColorRGBA` as *hexadecimal* `String`
    pub fn as_hex_string(&self) -> String {
        let rgba8 = self.as_array_8();
        encode_hex(&[ rgba8[0], rgba8[1], rgba8[2] ])
    }

    /// Returns: `reference` to `r` component
    pub fn r(&self) -> &f32 {
        &self.data[0]
    }

    /// Returns: `reference` to `g` component
    pub fn g(&self) -> &f32 {
        &self.data[1]
    }

    /// Returns: `reference` to `b` component
    pub fn b(&self) -> &f32 {
        &self.data[2]
    }

    /// Returns: `reference` to `a` component
    pub fn a(&self) -> &f32 {
        &self.data[3]
    }

    /// Returns: `mutable reference` to `r` component
    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.data[0]
    }

    /// Returns: `mutable reference` to `g` component
    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.data[1]
    }

    /// Returns: `mutable reference` to `b` component
    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.data[2]
    }

    /// Returns: `mutable reference` to `a` component
    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self.data[3]
    }

    /// Set `r`, `g`, `b` and `a` components
    pub fn set(&mut self, r:f32, g:f32, b:f32, a:f32) {
        self.data = [r,g,b,a];
    }

    /// Set components to given `array`
    pub fn set_array(&mut self, rgba:[f32;4]) {
        self.data = rgba;
    }
}

impl Display for ColorRGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorRGBA: {}, {}, {}, {}",
            self.r(), self.g(), self.b(), self.a()
        )
    }
}

impl From<Vector4> for ColorRGBA {
    fn from(v: Vector4) -> Self {
        Self::from_array(v.as_array().clone())
    }
}

impl From<ColorRGB8> for ColorRGBA {
    fn from(c: ColorRGB8) -> Self {
        Self::from_array_8([ c[0], c[1], c[2], 255 ])
    }
}

impl From<ColorRGBA8> for ColorRGBA {
    fn from(c: ColorRGBA8) -> Self {
        Self::from_array_8(c.as_array().clone())
    }
}

impl From<ColorRGB> for ColorRGBA {
    fn from(c: ColorRGB) -> Self {
        Self::from_array([c[0], c[1], c[2], 1.0])
    }
}

impl Index<usize> for ColorRGBA {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for ColorRGBA {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Add for ColorRGBA {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
            ]
        }
    }
}

impl Sub for ColorRGBA {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
            ]
        }
    }
}

impl Mul for ColorRGBA {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2],
                self[3] * rhs[3],
            ]
        }
    }
}

impl Div for ColorRGBA {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            data:[
                self[0] / rhs[0],
                self[1] / rhs[1],
                self[2] / rhs[2],
                self[3] / rhs[3],
            ]
        }
    }
}