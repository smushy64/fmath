use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};
use super::{
    color_float_to_color_byte,
    ColorRGB,
    ColorRGBA
};
use crate::{
    u8_add_overflow_max_clamp,
    u8_sub_overflow_min_clamp,
    u8_mul_overflow_max_clamp,
    hexadecimal::{
        decode_hex_str,
        encode_hex,
    },
    vector::Vector4,
};

/// Color represented as a 3 byte `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorRGB8 {
    data:[u8;3]
}

impl ColorRGB8 {
    /// Create `ColorRGB8` from `r`, `g` and `b`
    pub fn new( r:u8, g:u8, b:u8 ) -> Self {
        Self { data:[r,g,b] }
    }

    /// Create `ColorRGB8` from 3 byte `array`
    pub fn from_array( rgb:[u8;3] ) -> Self {
        Self { data:rgb }
    }

    /// Create `ColorRGB8` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `ColorRGB8` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGB8 ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array([bytes[0],bytes[1],bytes[2]]) )
    }

    /// Returns: `reference` to `ColorRGB8`'s data `array`
    pub fn as_array(&self) -> &[u8;3] {
        &self.data
    }

    /// Returns: `mutable reference` to `ColorRGB8`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [u8;3] {
        &mut self.data
    }

    /// Returns: `ColorRGB8` as *hexadecimal* `String`
    pub fn as_hex_string(&self) -> String {
        encode_hex(self.as_array())
    }

    /// Returns: `reference` to `r` component
    pub fn r(&self) -> &u8 {
        &self.data[0]
    }

    /// Returns: `reference` to `g` component
    pub fn g(&self) -> &u8 {
        &self.data[1]
    }

    /// Returns: `reference` to `b` component
    pub fn b(&self) -> &u8 {
        &self.data[2]
    }

    /// Returns: `mutable reference` to `r` component
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.data[0]
    }

    /// Returns: `mutable reference` to `g` component
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.data[1]
    }

    /// Returns: `mutable reference` to `b` component
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.data[2]
    }

    /// Set `r`, `g` and `b` components
    pub fn set(&mut self, r:u8, g:u8, b:u8) {
        self.data = [r,g,b];
    }

    /// Set components to given `array`
    pub fn set_array(&mut self, rgb:[u8;3]) {
        self.data = rgb;
    }
}

impl Display for ColorRGB8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorRGB8: {}, {}, {}",
            self.r(), self.g(), self.b()
        )
    }
}

impl From<Vector4> for ColorRGB8 {
    fn from(v: Vector4) -> Self {
        Self {
            data: [
                color_float_to_color_byte( v[0] ),
                color_float_to_color_byte( v[1] ),
                color_float_to_color_byte( v[2] ),
            ]
        }
    }
}

impl From<ColorRGB> for ColorRGB8 {
    fn from(c: ColorRGB) -> Self {
        Self::from_array(c.as_array_8())
    }
}

impl From<ColorRGBA> for ColorRGB8 {
    fn from(c: ColorRGBA) -> Self {
        let rgba = c.as_array_8();
        Self::from_array([rgba[0], rgba[1], rgba[2]])
    }
}

impl From<ColorRGBA8> for ColorRGB8 {
    fn from(c: ColorRGBA8) -> Self {
        let rgba = c.as_array();
        Self::from_array([rgba[0], rgba[1], rgba[2]])
    }
}

impl Index<usize> for ColorRGB8 {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.data[index]
    }
}

impl IndexMut<usize> for ColorRGB8 {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.data[index]
    }
}

impl Add for ColorRGB8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data:[
                u8_add_overflow_max_clamp( self[0], rhs[0] ),
                u8_add_overflow_max_clamp( self[1], rhs[1] ),
                u8_add_overflow_max_clamp( self[2], rhs[2] ),
            ]
        }
    }
}

impl Sub for ColorRGB8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data:[
                u8_sub_overflow_min_clamp( self[0], rhs[0] ),
                u8_sub_overflow_min_clamp( self[1], rhs[1] ),
                u8_sub_overflow_min_clamp( self[2], rhs[2] ),
            ]
        }
    }
}

impl Mul for ColorRGB8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data:[
                u8_mul_overflow_max_clamp( self[0], rhs[0] ),
                u8_mul_overflow_max_clamp( self[1], rhs[1] ),
                u8_mul_overflow_max_clamp( self[2], rhs[2] ),
            ]
        }
    }
}

impl Div for ColorRGB8 {
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

/// Color represented as a 4 byte `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorRGBA8 {
    data:[u8;4]
}

impl ColorRGBA8 {
    /// Create `ColorRGBA8` from `r`, `g`, `b` and `a`
    pub fn new( r:u8, g:u8, b:u8, a:u8 ) -> Self {
        Self { data:[r,g,b,a] }
    }

    /// Create `ColorRGBA8` from 4 byte `array`
    pub fn from_array( rgba:[u8;4] ) -> Self {
        Self { data:rgba }
    }

    /// Create `ColorRGBA8` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `ColorRGBA8` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGBA8 ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array([bytes[0],bytes[1],bytes[2],255]) )
    }

    /// Returns: `reference` to `ColorRGBA8`'s data `array`
    pub fn as_array(&self) -> &[u8;4] {
        &self.data
    }

    /// Returns: `mutable reference` to `ColorRGBA8`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [u8;4] {
        &mut self.data
    }

    /// Returns: `ColorRGBA8` as *hexadecimal* `String`
    pub fn as_hex_string(&self) -> String {
        let rgba = self.as_array();
        encode_hex(&[ rgba[0], rgba[1], rgba[2] ])
    }

    /// Returns: `reference` to `r` component
    pub fn r(&self) -> &u8 {
        &self.data[0]
    }

    /// Returns: `reference` to `g` component
    pub fn g(&self) -> &u8 {
        &self.data[1]
    }

    /// Returns: `reference` to `b` component
    pub fn b(&self) -> &u8 {
        &self.data[2]
    }

    /// Returns: `reference` to `a` component
    pub fn a(&self) -> &u8 {
        &self.data[3]
    }

    /// Returns: `mutable reference` to `r` component
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.data[0]
    }

    /// Returns: `mutable reference` to `g` component
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.data[1]
    }

    /// Returns: `mutable reference` to `b` component
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.data[2]
    }

    /// Returns: `mutable reference` to `a` component
    pub fn a_mut(&mut self) -> &mut u8 {
        &mut self.data[3]
    }

    /// Set `r`, `g`, `b` and `a` components
    pub fn set(&mut self, r:u8, g:u8, b:u8, a:u8) {
        self.data = [r,g,b,a];
    }

    /// Set components to given `array`
    pub fn set_array(&mut self, rgba:[u8;4]) {
        self.data = rgba;
    }
}

impl Display for ColorRGBA8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorRGBA8: {}, {}, {}, {}",
            self.r(), self.g(), self.b(), self.a()
        )
    }
}

impl From<Vector4> for ColorRGBA8 {
    fn from(v: Vector4) -> Self {
        Self {
            data: [
                color_float_to_color_byte( v[0] ),
                color_float_to_color_byte( v[1] ),
                color_float_to_color_byte( v[2] ),
                color_float_to_color_byte( v[3] ),
            ]
        }
    }
}

impl From<ColorRGB8> for ColorRGBA8 {
    fn from(c:ColorRGB8) -> Self {
        Self::from_array([ c[0], c[1], c[2], 255 ])
    }
}

impl From<ColorRGB> for ColorRGBA8 {
    fn from(c: ColorRGB) -> Self {
        let rgb = c.as_array_8();
        Self::from_array([ rgb[0], rgb[1], rgb[2], 255 ])
    }
}

impl From<ColorRGBA> for ColorRGBA8 {
    fn from(c: ColorRGBA) -> Self {
        Self::from_array(c.as_array_8())
    }
}

impl Index<usize> for ColorRGBA8 {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.data[index]
    }
}

impl IndexMut<usize> for ColorRGBA8 {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.data[index]
    }
}

impl Add for ColorRGBA8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data:[
                u8_add_overflow_max_clamp( self[0], rhs[0] ),
                u8_add_overflow_max_clamp( self[1], rhs[1] ),
                u8_add_overflow_max_clamp( self[2], rhs[2] ),
                u8_add_overflow_max_clamp( self[3], rhs[3] ),
            ]
        }
    }
}

impl Sub for ColorRGBA8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data:[
                u8_sub_overflow_min_clamp( self[0], rhs[0] ),
                u8_sub_overflow_min_clamp( self[1], rhs[1] ),
                u8_sub_overflow_min_clamp( self[2], rhs[2] ),
                u8_sub_overflow_min_clamp( self[3], rhs[3] ),
            ]
        }
    }
}

impl Mul for ColorRGBA8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data:[
                u8_mul_overflow_max_clamp( self[0], rhs[0] ),
                u8_mul_overflow_max_clamp( self[1], rhs[1] ),
                u8_mul_overflow_max_clamp( self[2], rhs[2] ),
                u8_mul_overflow_max_clamp( self[3], rhs[3] ),
            ]
        }
    }
}

impl Div for ColorRGBA8 {
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