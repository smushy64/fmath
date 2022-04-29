use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div
};

use crate::functions::hexadecimal::{
    decode_hex_rgb, encode_hex
};

use super::{
    color_float_to_color_byte,
    color_byte_to_color_float
};

/// `RGB`
/// 
/// 32-bit RGB color (*little endian*)
/// 
/// Implements: `Clone`, `Copy`, `PartialEq`, `Debug`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGB {
    c:u32
}

impl RGB {

    /// Create new `RGB`
    pub fn new_rgb( r:u8, g:u8, b:u8 ) -> Self {
        Self::new_rgba(r, g, b, 0)
    }

    /// Create new `RGBA`
    pub fn new_rgba( r:u8, g:u8, b:u8, a:u8 ) -> Self {
        Self {
            c:
                ( (r as u32) <<   0 ) |
                ( (g as u32) <<   8 ) |
                ( (b as u32) <<  16 ) |
                ( (a as u32) <<  24 )
        }
    }

    /// Create new `RGB` from `[u8;3]`
    pub fn from_array_rgb( rgb:[u8;3] ) -> Self {
        Self::new_rgb(rgb[0], rgb[1], rgb[2])
    }

    /// Create new `RGB` from `[u8;4]`
    pub fn from_array_rgba( rgba:[u8;4] ) -> Self {
        Self::new_rgba(rgba[0], rgba[1], rgba[2], rgba[3] )
    }

    /// Create new `RGB` from `[f32;3]`
    pub fn from_float_array_rgb( rgb:[f32;3] ) -> Self {
        Self::from_float_rgb(rgb[0], rgb[1], rgb[2])
    }

    /// Create new `RGB` from `[f32;4]`
    pub fn from_float_array_rgba( rgba:[f32;4] ) -> Self {
        Self::from_float_rgba(rgba[0], rgba[1], rgba[2], rgba[3] )
    }

    /// Create new `RGB` from hexadecimal code
    /// 
    /// Returns: `RGB` if provided hex was valid
    /// 
    /// Returns: `String` error if provided hex was **not** valid
    pub fn from_hex( hex:&str ) -> Result<Self, String> {
        Ok( Self::from_array_rgb(decode_hex_rgb(hex)?) )
    }

    /// Create new `RGB` from `f32`
    pub fn from_float_rgb( r:f32, g:f32, b:f32 ) -> Self {
        Self::new_rgb(
            color_float_to_color_byte(r),
            color_float_to_color_byte(g),
            color_float_to_color_byte(b),
        )
    }

    /// Create new `RGBA` from `f32`
    pub fn from_float_rgba( r:f32, g:f32, b:f32, a:f32 ) -> Self {
        Self::new_rgba(
            color_float_to_color_byte(r),
            color_float_to_color_byte(g),
            color_float_to_color_byte(b),
            color_float_to_color_byte(a),
        )
    }

    /// Returns `R` `G` `B` as `[u8;3]`
    pub fn as_rgb_array(&self) -> [u8;3] {
        [ self.r(), self.g(), self.b() ]
    }

    /// Returns `R` `G` `B` `A` as `[u8;4]`
    pub fn as_rgba_array(&self) -> [u8;4] {
        [ self.r(), self.g(), self.b(), self.a() ]
    }

    /// Returns `R` `G` `B` as `u8` tuple
    pub fn as_rgb_tuple( &self ) -> ( u8, u8, u8 ) {
        ( self.r(), self.g(), self.b() )
    }

    /// Returns `R` `G` `B` `A` as `u8` tuple
    pub fn as_rgba_tuple(&self) -> ( u8, u8, u8, u8 ) {
        ( self.r(), self.g(), self.b(), self.a() )
    }

    /// Returns hexadecimal encoding as `String`
    pub fn as_hex_rgb(&self) -> String {
        format!(
            "#{}",
            encode_hex(&self.as_rgb_array())
        )
    }

    /// Returns `R` `G` `B` as `[f32;3]`
    pub fn as_float_rgb_array(&self) -> [f32;3] {
        [
            color_byte_to_color_float(self.r()),
            color_byte_to_color_float(self.g()),
            color_byte_to_color_float(self.b()),
        ]
    }

    /// Returns `R` `G` `B` `A` as `[f32;4]`
    pub fn as_float_rgba_array(&self) -> [f32;4] {
        [
            color_byte_to_color_float(self.r()),
            color_byte_to_color_float(self.g()),
            color_byte_to_color_float(self.b()),
            color_byte_to_color_float(self.a()),
        ]
    }

    /// Returns `R` `G` `B` as `f32` tuple
    pub fn as_float_rgb_tuple( &self ) -> ( f32, f32, f32 ) {
        (
            color_byte_to_color_float(self.r()),
            color_byte_to_color_float(self.g()),
            color_byte_to_color_float(self.b()),
        )
    }

    /// Returns `R` `G` `B` `A` as `f32` tuple
    pub fn as_float_rgba_tuple( &self ) -> ( f32, f32, f32, f32 ) {
        (
            color_byte_to_color_float(self.r()),
            color_byte_to_color_float(self.g()),
            color_byte_to_color_float(self.b()),
            color_byte_to_color_float(self.a()),
        )
    }

    /// Returns `R` as `u8`
    pub fn r(&self) -> u8 {
        (self.c >> 0) as u8
    }

    /// Returns `G` as `u8`
    pub fn g(&self) -> u8 {
        (self.c >> 8) as u8
    }

    /// Returns `B` as `u8`
    pub fn b(&self) -> u8 {
        (self.c >> 16) as u8
    }

    /// Returns `A` as `u8`
    pub fn a(&self) -> u8 {
        (self.c >> 24) as u8
    }


    /// Set `R` using `u8` value
    pub fn set_r(&mut self, r:u8) {
        // clear bits first
        self.c &= !(255_u32 << 0);
        // then push new value
        self.c |= (r as u32) << 0;
    }

    /// Set `G` using `u8` value
    pub fn set_g(&mut self, g:u8) {
        // clear bits first
        self.c &= !(255_u32 << 8);
        // then push new value
        self.c |= (g as u32) << 8;
    }

    /// Set `B` using `u8` value
    pub fn set_b(&mut self, b:u8) {
        // clear bits first
        self.c &= !(255_u32 << 16);
        // then push new value
        self.c |= (b as u32) << 16;
    }

    /// Set `A` using `u8` value
    pub fn set_a(&mut self, a:u8) {
        // clear bits first
        self.c &= !(255_u32 << 24);
        // then push new value
        self.c |= (a as u32) << 24;
    }


    /// Returns `R` as `f32`
    pub fn r_f32(&self) -> f32 {
        color_byte_to_color_float((self.c >> 0) as u8)
    }

    /// Returns `G` as `f32`
    pub fn g_f32(&self) -> f32 {
        color_byte_to_color_float((self.c >> 8) as u8)
    }

    /// Returns `B` as `f32`
    pub fn b_f32(&self) -> f32 {
        color_byte_to_color_float((self.c >> 16) as u8)
    }

    /// Returns `A` as `f32`
    pub fn a_f32(&self) -> f32 {
        color_byte_to_color_float((self.c >> 24) as u8)
    }


    /// Set `R` using `f32` value
    pub fn set_r_f32(&mut self, r:f32) {
        self.set_r(color_float_to_color_byte(r))
    }

    /// Set `G` using `f32` value
    pub fn set_g_f32(&mut self, g:f32) {
        self.set_g(color_float_to_color_byte(g))
    }

    /// Set `B` using `f32` value
    pub fn set_b_f32(&mut self, b:f32) {
        self.set_b(color_float_to_color_byte(b))
    }

    /// Set `A` using `f32` value
    pub fn set_a_f32(&mut self, a:f32) {
        self.set_a(color_float_to_color_byte(a))
    }

    /// Returns: `String` representation of data in 0.0-1.0 range
    pub fn format_as_float_rgb(&self) -> String {
        format!(
            "RGB: ( {}, {}, {} )",
            self.r_f32(), self.g_f32(), self.b_f32()
        )
    }

    /// Returns: `String` representation of data as bytes
    pub fn format_as_bytes(&self) -> String {
        format!("{:#034b}", self.c)
    }

    /// Create new `RGB` with `R` and `A` set to **255** 
    pub fn new_red() -> Self {
        Self {
            c:
                ( ( 255 ) <<   0 ) |
                ( (   0 ) <<   8 ) |
                ( (   0 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `G` and `A` set to **255** 
    pub fn new_green() -> Self {
        Self {
            c:
                ( (   0 ) <<   0 ) |
                ( ( 255 ) <<   8 ) |
                ( (   0 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `B` and `A` set to **255** 
    pub fn new_blue() -> Self {
        Self {
            c:
                ( (   0 ) <<   0 ) |
                ( (   0 ) <<   8 ) |
                ( ( 255 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `G` and `A` set to **255** 
    pub fn new_yellow() -> Self {
        Self {
            c:
                ( ( 255 ) <<   0 ) |
                ( ( 255 ) <<   8 ) |
                ( (   0 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `G` `B` and `A` set to **255** 
    pub fn new_cyan() -> Self {
        Self {
            c:
                ( (   0 ) <<   0 ) |
                ( ( 255 ) <<   8 ) |
                ( ( 255 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `B` and `A` set to **255** 
    pub fn new_magenta() -> Self {
        Self {
            c:
                ( ( 255 ) <<   0 ) |
                ( (   0 ) <<   8 ) |
                ( ( 255 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `G` `B` and `A` set to **255** 
    pub fn new_white() -> Self {
        Self {
            c:
                ( ( 255 ) <<   0 ) |
                ( ( 255 ) <<   8 ) |
                ( ( 255 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `G` `B` set to **0** and `A` set to **255** 
    pub fn new_black() -> Self {
        Self {
            c:
                ( (   0 ) <<   0 ) |
                ( (   0 ) <<   8 ) |
                ( (   0 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `G` `B` set to **127** and `A` set to **255** 
    pub fn new_gray() -> Self {
        Self {
            c:
                ( ( 127 ) <<   0 ) |
                ( ( 127 ) <<   8 ) |
                ( ( 127 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `G` `B` set to **127** and `A` set to **255** 
    pub fn new_grey() -> Self {
        Self {
            c:
                ( ( 127 ) <<   0 ) |
                ( ( 127 ) <<   8 ) |
                ( ( 127 ) <<  16 ) |
                ( ( 255 ) <<  24 )
        }
    }

    /// Create new `RGB` with `R` `G` `B` and `A` set to **0** 
    pub fn new_clear() -> Self {
        Self {
            c:
                ( ( 0 ) <<   0 ) |
                ( ( 0 ) <<   8 ) |
                ( ( 0 ) <<  16 ) |
                ( ( 0 ) <<  24 )
        }
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB: ( {}, {}, {}, {} )",
            self.r(), self.g(), self.b(), self.a()
        )
    }
}

impl Add for RGB {
    type Output = RGB;
    fn add(self, rhs: Self) -> Self {
        Self::new_rgba(
            u8::saturating_add( self.r(), rhs.r() ),
            u8::saturating_add( self.g(), rhs.g() ),
            u8::saturating_add( self.b(), rhs.b() ),
            u8::saturating_add( self.a(), rhs.a() ),
        )
    }
}

impl Sub for RGB {
    type Output = RGB;
    fn sub(self, rhs: Self) -> Self {
        Self::new_rgba(
            u8::saturating_sub( self.r(), rhs.r() ),
            u8::saturating_sub( self.g(), rhs.g() ),
            u8::saturating_sub( self.b(), rhs.b() ),
            u8::saturating_sub( self.a(), rhs.a() ),
        )
    }
}

impl Mul for RGB {
    type Output = RGB;
    fn mul(self, rhs: Self) -> Self {
        Self::new_rgba(
            u8::saturating_mul( self.r(), rhs.r() ),
            u8::saturating_mul( self.g(), rhs.g() ),
            u8::saturating_mul( self.b(), rhs.b() ),
            u8::saturating_mul( self.a(), rhs.a() ),
        )
    }
}

impl Mul<f32> for RGB {
    type Output = RGB;
    fn mul(self, rhs: f32) -> Self {

        let scale = | b: u8, f: f32 | {
            let result = b as f32 * f;
            result.clamp(0.0, 255.0) as u8
        };

        Self::new_rgba(
            scale( self.r(), rhs ),
            scale( self.g(), rhs ),
            scale( self.b(), rhs ),
            scale( self.a(), rhs ),
        )
    }
}

impl Div for RGB {
    type Output = RGB;
    fn div(self, rhs: Self) -> Self {
        Self::new_rgba(
            u8::saturating_div( self.r(), rhs.r() ),
            u8::saturating_div( self.g(), rhs.g() ),
            u8::saturating_div( self.b(), rhs.b() ),
            u8::saturating_div( self.a(), rhs.a() ),
        )
    }
}

impl Div<f32> for RGB {
    type Output = RGB;
    fn div(self, rhs: f32) -> Self {

        let divide = | b: u8, f: f32 | {
            let result = b as f32 / f;
            result.clamp(0.0, 255.0) as u8
        };

        Self::new_rgba(
            divide( self.r(), rhs ),
            divide( self.g(), rhs ),
            divide( self.b(), rhs ),
            divide( self.a(), rhs ),
        )
    }
}
