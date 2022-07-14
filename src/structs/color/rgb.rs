use core::fmt;
use core::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
};
use crate::{
    functions::hexadecimal::{ decode_hex_rgb, encode_hex },
    structs::{ Vector3, Vector3Int, Vector3f64, Vector4, Vector4Int, Vector4f64, color::HSV }
};

/// 32-bit RGB color (*little endian*)
/// 
/// A B G R
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB { c:u32 }
impl RGB {
    /// Create new RGB color
    pub fn new_rgb( r:u8, g:u8, b:u8 ) -> Self { Self::new_rgba(r, g, b, 255) }
    /// Create new RGBA color
    pub fn new_rgba( r:u8, g:u8, b:u8, a:u8 ) -> Self {
        Self { c:
            ( (r as u32) <<   0 ) |
            ( (g as u32) <<   8 ) |
            ( (b as u32) <<  16 ) |
            ( (a as u32) <<  24 )
        }
    }
    /// Create new RGB color
    pub fn new_rgb_f32( r:f32, g:f32, b:f32 ) -> Self {
        Self::new_rgb(
            ( r * 255.0 ).clamp( 0.0, 255.0 ) as u8,
            ( g * 255.0 ).clamp( 0.0, 255.0 ) as u8,
            ( b * 255.0 ).clamp( 0.0, 255.0 ) as u8,
        )
    }
    /// Create new RGBA color
    pub fn new_rgba_f32( r:f32, g:f32, b:f32, a:f32 ) -> Self {
        Self::new_rgba(
            ( r * 255.0 ).clamp( 0.0, 255.0 ) as u8,
            ( g * 255.0 ).clamp( 0.0, 255.0 ) as u8,
            ( b * 255.0 ).clamp( 0.0, 255.0 ) as u8,
            ( a * 255.0 ).clamp( 0.0, 255.0 ) as u8,
        )
    }
    /// Create new RGB from byte slice
    pub fn from_byte_slice( bytes:&[u8] ) -> Self {
        let mut components = [0u8;4];
        for ( byte, component ) in bytes.iter().zip( components.iter_mut() ) { *component = *byte; }
        Self::new_rgba( components[0], components[1], components[2], components[3] )
    }
    /// Create new RGB from float slice
    pub fn from_f32_slice( floats:&[f32] ) -> Self {
        let bytes = floats.iter()
            .map( |x| ( *x * 255.0 ).clamp( 0.0, 255.0 ) as u8 )
            .collect::<Vec<u8>>();
        Self::from_byte_slice(&bytes)
    }
    /// Create new `RGB` from hexadecimal `&str`
    /// 
    /// Returns: `RGB` if provided hex was valid
    /// 
    /// Returns: `String` error if provided hex was **not** valid
    pub fn from_hex( hex:&str ) -> Result<Self, String> { Ok( Self::from_byte_slice(&decode_hex_rgb(hex)?) ) }
    /// Returns: 32-bit `RGBA` as *big-endian*
    pub fn to_be( &self ) -> u32 { self.c.to_be() }
    /// RGB as byte array
    pub fn as_bytes_rgb(&self) -> [u8;3] { [ self.r(), self.g(), self.b() ] }
    /// RGBA as byte array
    pub fn as_bytes_rgba(&self) -> [u8;4] { [ self.r(), self.g(), self.b(), self.a() ] }
    /// RGB as f32 array
    pub fn as_f32_rgb(&self) -> [f32;3] { [
        ( self.r() as f32 ) / 255.0,
        ( self.g() as f32 ) / 255.0,
        ( self.b() as f32 ) / 255.0,
    ] }
    /// RGBA as f32 array
    pub fn as_f32_rgba(&self) -> [f32;4] { [
        ( self.r() as f32 ) / 255.0,
        ( self.g() as f32 ) / 255.0,
        ( self.b() as f32 ) / 255.0,
        ( self.a() as f32 ) / 255.0,
    ] }
    /// Returns hexadecimal encoding as `String`
    pub fn as_hex_rgb(&self) -> String { format!( "#{}", encode_hex( &self.as_bytes_rgb() ) ) }
    /// Returns `R` as `u8`
    pub fn r(&self) -> u8 { (self.c >> 0) as u8 }
    /// Returns `G` as `u8`
    pub fn g(&self) -> u8 { (self.c >> 8) as u8 }
    /// Returns `B` as `u8`
    pub fn b(&self) -> u8 { (self.c >> 16) as u8 }
    /// Returns `A` as `u8`
    pub fn a(&self) -> u8 { (self.c >> 24) as u8 }
    /// Set `R` `G` `B` and `A` to **0**
    pub fn clear( &mut self ) { self.c = 0; }
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
    pub fn r_f32(&self) -> f32 { ( self.r() as f32 ) / 255.0 }
    /// Returns `G` as `f32`
    pub fn g_f32(&self) -> f32 { ( self.g() as f32 ) / 255.0 }
    /// Returns `B` as `f32`
    pub fn b_f32(&self) -> f32 { ( self.b() as f32 ) / 255.0 }
    /// Returns `A` as `f32`
    pub fn a_f32(&self) -> f32 { ( self.a() as f32 ) / 255.0 }
    /// Set `R` using `f32` value
    pub fn set_r_f32(&mut self, r:f32) { self.set_r( ( r * 255.0 ).clamp( 0.0, 255.0 ) as u8 ) }
    /// Set `G` using `f32` value
    pub fn set_g_f32(&mut self, g:f32) { self.set_g( ( g * 255.0 ).clamp( 0.0, 255.0 ) as u8 ) }
    /// Set `B` using `f32` value
    pub fn set_b_f32(&mut self, b:f32) { self.set_b( ( b * 255.0 ).clamp( 0.0, 255.0 ) as u8 ) }
    /// Set `A` using `f32` value
    pub fn set_a_f32(&mut self, a:f32) { self.set_a( ( a * 255.0 ).clamp( 0.0, 255.0 ) as u8 ) }
    /// Returns: `String` representation of data in 0.0-1.0 range
    pub fn format_rgb_f32(&self) -> String {
        format!(
            "RGB: ( {:5.3}, {:5.3}, {:5.3} )",
            self.r_f32(), self.g_f32(), self.b_f32()
        )
    }

    /// Returns: `String` representation of data in 0.0-1.0 range
    pub fn format_rgba_f32(&self) -> String {
        format!(
            "RGBA: ( {:5.3}, {:5.3}, {:5.3}, {:5.3} )",
            self.r_f32(), self.g_f32(), self.b_f32(), self.a_f32()
        )
    }

    /// Returns: `String` representation of data as bytes
    pub fn format_bytes(&self) -> String {
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
    pub fn new_clear() -> Self { Self { c:0 } }
}
impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGBA: ( {:0>3}, {:0>3}, {:0>3}, {:0>3} )",
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
impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs }
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
impl SubAssign for RGB {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
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
impl MulAssign for RGB {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}
impl Mul<f32> for RGB {
    type Output = RGB;
    fn mul(self, rhs: f32) -> Self {
        let scale = | lhs: u8, rhs: f32 | {
            ( ( ( lhs as f32 / 255.0 ) * rhs ) * 255.0 ).clamp( 0.0, 255.0 ) as u8
        };
        Self::new_rgba(
            scale( self.r(), rhs ),
            scale( self.g(), rhs ),
            scale( self.b(), rhs ),
            scale( self.a(), rhs ),
        )
    }
}
impl MulAssign<f32> for RGB {
    fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; }
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
impl DivAssign for RGB {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}
impl Div<f32> for RGB {
    type Output = RGB;
    fn div(self, rhs: f32) -> Self {

        let divide = | lhs: u8, rhs: f32 | {
            ( ( ( lhs as f32 / 255.0 ) / rhs ) * 255.0 ).clamp( 0.0, 255.0 ) as u8
        };

        Self::new_rgba(
            divide( self.r(), rhs ),
            divide( self.g(), rhs ),
            divide( self.b(), rhs ),
            divide( self.a(), rhs ),
        )
    }
}
impl DivAssign<f32> for RGB {
    fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; }
}

impl From<Vector3> for RGB {
    fn from(v: Vector3) -> Self { Self::new_rgb_f32( v[0], v[1], v[2] ) }
}
impl From<Vector3f64> for RGB {
    fn from(v: Vector3f64) -> Self { Self::new_rgb_f32( v[0] as f32, v[1] as f32, v[2] as f32 ) }
}
impl From<Vector3Int> for RGB {
    fn from(v: Vector3Int) -> Self { Self::from_byte_slice( &v.to_u8_array() ) }
}

impl From<Vector4> for RGB {
    fn from(v: Vector4) -> Self { Self::new_rgba_f32( v[0], v[1], v[2], v[3] ) }
}
impl From<Vector4f64> for RGB {
    fn from(v: Vector4f64) -> Self { Self::new_rgba_f32( v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32 ) }
}
impl From<Vector4Int> for RGB {
    fn from(v: Vector4Int) -> Self { Self::from_byte_slice( &v.to_u8_array() ) }
}

impl From<HSV> for RGB { fn from(h: HSV) -> Self { h.to_rgb() } }
