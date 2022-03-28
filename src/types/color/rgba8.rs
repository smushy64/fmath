use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};
use super::{
    color_float_to_color_byte,
    RGB,
    RGB8,
    RGBA,
    HSV
};
use crate::functions::{
    u8_add_overflow_max_clamp,
    u8_sub_overflow_min_clamp,
    u8_mul_overflow_max_clamp,
    hexadecimal::{
        decode_hex_str,
        encode_hex,
    },
};

/// Color represented as a 4 byte `array`
/// 
/// Indexable with **[ ]**
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGBA8 {
    data:[u8;4]
}

impl RGBA8 {

    /// Create `RGBA8` from `r`, `g`, `b` and `a`
    pub fn new( r:u8, g:u8, b:u8, a:u8 ) -> Self {
        Self { data:[r,g,b,a] }
    }

    /// Create `RGBA8` from 4 byte `array`
    pub fn from_array( rgba:[u8;4] ) -> Self {
        Self { data:rgba }
    }

    /// Create `RGBA8` from *hexadecimal* `&str`
    /// 
    /// - Returns: new `RGBA8` if hexadecimal decode is successful
    /// - Returns: error `String` if hexadecimal decode is **not** successful
    /// - Returns: error `String` if `decode_hex_str(hex)` returns a `Vec<u8>` with greater than **or** less than 3 elements
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        let bytes = decode_hex_str(hex)?;
        if bytes.len() != 3 {
            return Err( format!("COLOR RGBA8 ERROR: Input hexadecimal should be formatted as such: rrggbb!") )
        }
        Ok( Self::from_array([bytes[0],bytes[1],bytes[2],255]) )
    }

    /// Returns: `reference` to `RGBA8`'s data `array`
    pub fn as_array(&self) -> &[u8;4] {
        &self.data
    }

    /// Returns: `mutable reference` to `RGBA8`'s data `array`
    pub fn as_mut_array(&mut self) -> &mut [u8;4] {
        &mut self.data
    }

    /// Returns: `RGBA8` as *hexadecimal* `String`
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

    /// Create new `RGBA8` that represents `red`
    pub fn new_red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    /// Create new `RGBA8` that represents `green`
    pub fn new_green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    /// Create new `RGBA8` that represents `blue`
    pub fn new_blue() -> Self {
        Self::new(0, 0, 255, 255)
    }

    /// Create new `RGBA8` that represents `cyan`
    pub fn new_cyan() -> Self {
        Self::new(0, 255, 255, 255)
    }

    /// Create new `RGBA8` that represents `magenta`
    pub fn new_magenta() -> Self {
        Self::new(255, 0, 255, 255)
    }

    /// Create new `RGBA8` that represents `yellow`
    pub fn new_yellow() -> Self {
        Self::new(255, 255, 0, 255)
    }

    /// Create new `RGBA8` that represents `black`
    pub fn new_black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    /// Create new `RGBA8` that represents `gray`
    pub fn new_gray() -> Self {
        Self::new(128, 128, 128, 255)
    }

    /// Create new `RGBA8` that represents `white`
    pub fn new_white() -> Self {
        Self::new(255, 255, 255, 255)
    }

    /// Create new `RGBA8` that represents `clear`
    pub fn new_clear() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Display for RGBA8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGBA8: {}, {}, {}, {}",
            self.r(), self.g(), self.b(), self.a()
        )
    }
}

/// Create `RGBA8` from `RGB8`
/// 
/// `a` component set to **255**
impl From<RGB8> for RGBA8 {
    fn from(c: RGB8) -> Self {
        Self::new(c[0], c[1], c[2], 255)
    }
}

/// Create `RGBA8` from `RGB`
/// 
/// `a` component set to **255**
impl From<RGB> for RGBA8 {
    fn from(c_f32:RGB) -> Self {
        let c = c_f32.as_array_8();
        Self::new(c[0], c[1], c[2], 255)
    }
}

/// Create `RGBA8` from `HSV`
/// 
/// `a` component set to **255**
impl From<HSV> for RGBA8 {
    fn from(c_hsv:HSV) -> Self {
        let c = c_hsv.as_rgb_array();
        Self::new(
            color_float_to_color_byte(c[0]),
            color_float_to_color_byte(c[1]),
            color_float_to_color_byte(c[2]),
            255
        )
    }
}

impl From<RGBA> for RGBA8 {
    fn from(c:RGBA) -> Self {
        Self::new(
            color_float_to_color_byte(c[0]),
            color_float_to_color_byte(c[1]),
            color_float_to_color_byte(c[2]),
            color_float_to_color_byte(c[3]),
        )
    }
}

impl Index<usize> for RGBA8 {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.data[index]
    }
}

impl IndexMut<usize> for RGBA8 {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.data[index]
    }
}

impl Add for RGBA8 {
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

impl Sub for RGBA8 {
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

impl Mul for RGBA8 {
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

impl Div for RGBA8 {
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