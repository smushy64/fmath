use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};
use super::{
    color_byte_to_color_float,
    color_float_to_color_byte,
    RGB8,
    RGBA8,
    RGBA,
    HSV,
};
use crate::functions::{
    hexadecimal::{
        decode_hex_str,
        encode_hex,
    },
};

/// Color represented as a 3 `f32` `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGB {
    data:[f32;3]
}

impl RGB {

    /// Create `RGB` from `RGBA`
    /// 
    /// `a` component is **lost** in conversion!
    pub fn from_rgba( c:RGBA ) -> Self {
        Self::new( c[0], c[1], c[2] )
    }

    /// Create `RGB` from `RGBA8`
    /// 
    /// `a` component is **lost** in conversion!
    pub fn from_rgba8( c:RGBA8 ) -> Self {
        Self::new(
            color_byte_to_color_float(c[0]),
            color_byte_to_color_float(c[1]),
            color_byte_to_color_float(c[2]),
        )
    }

    /// Create `RGB` from `r`, `g` and `b`
    pub fn new( r:f32, g:f32, b:f32 ) -> Self {
        Self { data:[r,g,b] }
    }

    /// Create `RGB` from 3 `f32` `array`
    pub fn from_array( rgb:[f32;3] ) -> Self {
        Self { data:rgb }
    }

    /// Create `RGB` from 3 byte `array`
    pub(crate) fn from_array_8( rgb:[u8;3] ) -> Self {
        Self {
            data: [
                color_byte_to_color_float(rgb[0]),
                color_byte_to_color_float(rgb[1]),
                color_byte_to_color_float(rgb[2]),
            ]
        }
    }

    /// Create `RGB` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `RGB` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGB ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array_8([bytes[0],bytes[1],bytes[2]]) )
    }

    /// Returns: `reference` to `RGB`'s data `array`
    pub fn as_array(&self) -> &[f32;3] {
        &self.data
    }

    /// Returns: new `array` with data as bytes instead of `f32`
    pub(crate) fn as_array_8(&self) -> [u8;3] {
        [
            color_float_to_color_byte( self.data[0] ),
            color_float_to_color_byte( self.data[1] ),
            color_float_to_color_byte( self.data[2] )
        ]
    }

    /// Returns: `mutable reference` to `RGB`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [f32;3] {
        &mut self.data
    }

    /// Returns: `RGB` as *hexadecimal* `String`
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

    /// Create new `RGB` that represents `red`
    pub fn new_red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Create new `RGB` that represents `green`
    pub fn new_green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Create new `RGB` that represents `blue`
    pub fn new_blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Create new `RGB` that represents `cyan`
    pub fn new_cyan() -> Self {
        Self::new(0.0, 1.0, 1.0)
    }

    /// Create new `RGB` that represents `magenta`
    pub fn new_magenta() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }

    /// Create new `RGB` that represents `yellow`
    pub fn new_yellow() -> Self {
        Self::new(1.0, 1.0, 0.0)
    }

    /// Create new `RGB` that represents `black`
    pub fn new_black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Create new `RGB` that represents `gray`
    pub fn new_gray() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }

    /// Create new `RGB` that represents `white`
    pub fn new_white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB: {}, {}, {}",
            self.r(), self.g(), self.b()
        )
    }
}

impl From<RGB8> for RGB {
    fn from(c:RGB8) -> Self {
        Self::from_array_8(c.as_array().clone())
    }
}

impl From<HSV> for RGB {
    fn from(c:HSV) -> Self {
        Self::from_array( c.as_rgb_array() )
    }
}

impl Index<usize> for RGB {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for RGB {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Add for RGB {
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

impl Sub for RGB {
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

impl Mul for RGB {
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

impl Div for RGB {
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