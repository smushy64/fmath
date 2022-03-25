#[allow(dead_code)]

mod rgb8;
pub use rgb8::{
    ColorRGB8,
    ColorRGBA8,
};

mod rgb;
pub use rgb::{
    ColorRGB,
    ColorRGBA,
};

mod hsv;
pub use hsv::{
    ColorHSV
};

pub(crate) fn color_byte_to_color_float( byte:u8 ) -> f32 {
    ( byte as f32 ) / 255.0
}

pub(crate) fn color_float_to_color_byte( f:f32 ) -> u8 {
    let result = f * 255.0;
    if result > 255.0 {
        return 255_u8;
    } else {
        return result as u8
    }
}