# fmath
Math Library for my personal Rust Projects

## About

This is not a general-purpose math library. It is specifically written for my purposes only.

Feel free to use it in your projects if it fits your needs, don't even worry about attribution :)

## Example
```rust

use fmath::{
  clamp, max, min,

  Angle,
  degrees_overflow,
  degrees_to_radians,
  radians_to_degrees,

  u8_add_overflow_max_clamp,
  u8_mul_overflow_max_clamp,
  u8_sub_overflow_min_clamp,

  vector::*,
  matrix::{
      Matrix4x4,
      Matrix4x4_consts::*,
  },
  color::*,
  hexadecimal::*
};

// General
let x = 5.0;
let x_clamped = clamp( x, 0.0, 1.0 ); // 1.0

let number_list:[f32;5] = [ 5.0, 7.0, 1.0, 80.0, -2.0 ];
let max_from_list = max( &number_list ); //  80.0
let min_from_list = min( &number_list ); // -2.0

let another_number_list:[u32;4] = [ 2, 3, 4, 1 ];
let max_from_another_list = max( &another_number_list ); // 4
let min_from_another_list = min( &another_number_list ); // 1

let mut angle = 730.2;
angle = degrees_overflow( angle ); // 10.2

let angle_radians = degrees_to_radians( angle ); // 0.1780236
let angle_degrees = radians_to_degrees(angle_radians); // back to degrees (10.2)

let mut max_example_1:u8 = 254;
max_example_1 = u8_add_overflow_max_clamp( max_example_1, 2   ); // 255

let mut max_example_2:u8 = 120;
max_example_2 = u8_mul_overflow_max_clamp( max_example_2, 4   ); // 255

let mut min_example_1:u8 = 0;
min_example_1 = u8_sub_overflow_min_clamp( min_example_1, 2 ); // 0

let hex_encode_example = encode_hex(&[0, 255, 0]); // 00ff00
let hex_decode_example = decode_hex_str("654dbd").unwrap(); // Vec<u8>[101, 77, 189]

// Vector math
let v1 = Vector2::new( 1.0, 0.0 );
let v2 = Vector2::new( 0.0, 0.0 );
let v3 = v1 + v2; // ( 1.0, 1.0 )

let v1_mag = v1.magnitude(); // 1.0

// Matrix math
let m1 = MATRIX4X4_ZERO.clone(); // creates empty Matrix array

let translate = Vector3::new( 0.0, 1.0, 0.0 );
let rotate    = Vector3::new( 0.0, 0.0, 90.0 );
let scale     = Vector3::new( 0.5, 0.5, 0.5 );

// creates Translate-Rotate-Scale Matrix from Vector3's
let m2 = Matrix4x4::new_trs(
  translate.as_array(), // accepts [f32;3] only
  rotate.as_array(),    // accepts [f32;3] only
  scale.as_array(),     // accepts [f32;3] only
  Angle::Degrees // sets angle type to degrees
);

let point1 = Vector3::new( 1.0, 2.0, 3.0 );
// apply transformation Matrix to Vector3
let point1_transformed:Vector3 = Matrix4x4::mul_vector3( &m2, &point1 ); 

let point2 = Vector4::new( 1.0, 2.0, 3.0, 1.0 );
// apply transformation Matrix to Vector4
let point2_transformed:Vector4 = Matrix4x4::mul_vector4( &m2, &point2 ); 

// Colors
let mut rgb = ColorRGB::new( 0.25, 0.25, 0.25 );  // creates RGB with given f32's
let rgb8 = ColorRGB8::new( 160, 255, 0 );        // creates RGB8 with given u8's
let rgba = ColorRGBA::from_hex( "994cd4" ); // creates RGBA from hex, sets alpha to 1.0

rgb = rgb8.into(); // convert RGB8 to RGB
rgb[0] = 0.25; // address each component with index

// creates HSV with given hue, saturation and value f32's
let mut hsv = ColorHSV::new( 25.0, 0.5, 0.9 );

rgb = hsv.as_rgb(); // convert HSV to RGB

hsv = rgb.into(); // convert RGB8 to HSV

// All types (except Angle enum) implement fmt::Display
println!("rgb: {}", rgb); // ColorRGB: {r} {g} {b}


```

## How to use

### Include

in Cargo.toml file...
```rust
  [dependencies]
  fmath = "*current-version*"
```

### Documentation

build documentation with *cargo doc* in the command line
```
  cargo doc
```

## Library Contains...

### Functions

  - Clamp ( Partial Ord )
  - Min and Max ( Partial Ord )
  - Degrees to Radians ( f32 )
  - Radians to Degrees ( f32 )
  - Degrees Overflow ( keeps f32 within 0.0-360.0 range )
  - u8 overflow-safe operations ( clamps within 0 and u8_max (255) )
  - Hexadecimal Encoder ( String ) /Decoder ( Vec\<u8\> )

### Types

- Colors
  - RGB   (struct) ( f32;3 )
  - RGBA  (struct) ( f32;4 )
  - RGB8  (struct) (  u8;3 )
  - RGBA8 (struct) (  u8;4 )
  - HSV   (struct) ( hue: f32, saturation: f32, value: f32 )
- Angle (enum)
  - Degrees
  - Radians
- Vectors
  - Vector2 (struct) ( f32;2 )
  - Vector3 (struct) ( f32;3 )
  - Vector4 (struct) ( f32;4 )
- Matrix4x4 (struct) ( f32;16 )