use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};
use super::{
    color_byte_to_color_float,
    color_float_to_color_byte,
    RGB,
    RGB8,
    RGBA8,
    HSV,
};
use crate::{
    functions::hexadecimal::{
        decode_hex_str,
        encode_hex,
    },
    types::vector::Vector4
};

/// Color represented as a 4 `f32` `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGBA {
    data:[f32;4]
}

impl RGBA {

    /// Create `RGBA` from `r`, `g`, `b` and `a`
    pub fn new( r:f32, g:f32, b:f32, a:f32 ) -> Self {
        Self { data:[r,g,b,a] }
    }

    /// Create `RGBA` from `Vector4`
    pub fn from_vector4( v:Vector4 ) -> Self {
        Self::from_array(v.as_array().clone())
    }

    /// Create `RGBA` from 4 `f32` `array`
    pub fn from_array( rgba:[f32;4] ) -> Self {
        Self { data:rgba }
    }

    /// Create `RGBA` from 3 byte `array`
    fn from_array_8( rgba:[u8;4] ) -> Self {
        Self {
            data: [
                color_byte_to_color_float(rgba[0]),
                color_byte_to_color_float(rgba[1]),
                color_byte_to_color_float(rgba[2]),
                color_byte_to_color_float(rgba[3]),
            ]
        }
    }

    /// Create `RGBA` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `RGBA` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGBA ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array_8([bytes[0],bytes[1],bytes[2], 255]) )
    }

    /// Returns: `reference` to `RGBA`'s data `array`
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

    /// Returns: `mutable reference` to `RGBA`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;4] {
        &mut self.data
    }

    /// Returns: `RGBA` as *hexadecimal* `String`
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

    /// Create new `RGBA` that represents `red`
    pub fn new_red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Create new `RGBA` that represents `green`
    pub fn new_green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    /// Create new `RGBA` that represents `blue`
    pub fn new_blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// Create new `RGBA` that represents `cyan`
    pub fn new_cyan() -> Self {
        Self::new(0.0, 1.0, 1.0, 1.0)
    }

    /// Create new `RGBA` that represents `magenta`
    pub fn new_magenta() -> Self {
        Self::new(1.0, 0.0, 1.0, 1.0)
    }

    /// Create new `RGBA` that represents `yellow`
    pub fn new_yellow() -> Self {
        Self::new(1.0, 1.0, 0.0, 1.0)
    }

    /// Create new `RGBA` that represents `black`
    pub fn new_black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Create new `RGBA` that represents `gray`
    pub fn new_gray() -> Self {
        Self::new(0.5, 0.5, 0.5, 1.0)
    }

    /// Create new `RGBA` that represents `white`
    pub fn new_white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    /// Create new `RGBA` that represents `clear`
    pub fn new_clear() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Display for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGBA: {}, {}, {}, {}",
            self.r(), self.g(), self.b(), self.a()
        )
    }
}

/// Create `RGBA` from `RGB`
/// 
/// `a` component set to **1.0**
impl From<RGB> for RGBA {
    fn from(c:RGB) -> Self {
        Self::new(c[0], c[1], c[2], 1.0)
    }
}

/// Create `RGBA` from `RGB8`
/// 
/// `a` component set to **1.0**
impl From<RGB8> for RGBA {
    fn from(c:RGB8) -> Self {
        Self::new(
            color_byte_to_color_float(c[0]),
            color_byte_to_color_float(c[1]),
            color_byte_to_color_float(c[2]),
            1.0
        )
    }
}

/// Create `RGBA` from `HSV`
/// 
/// `a` component set to **1.0**
impl From<HSV> for RGBA {
    fn from(c_hsv:HSV) -> Self {
        let c = c_hsv.as_rgb_array();
        Self::new(
            c[0], c[1], c[2], 1.0
        )
    }
}

impl From<RGBA8> for RGBA {
    fn from(c:RGBA8) -> Self {
        Self::from_array_8(c.as_array().clone())
    }
}

impl Index<usize> for RGBA {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for RGBA {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Add for RGBA {
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

impl Sub for RGBA {
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

impl Mul for RGBA {
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

impl Div for RGBA {
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