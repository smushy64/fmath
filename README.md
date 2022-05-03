# fmath
Math Library for my personal Rust Projects

## About

This is **not** a general-purpose math library. It is specifically written for my purposes only.

Feel free to use it in your projects if it fits your needs, don't even worry about attribution :)

- [documentation](https://docs.rs/fmath/latest/fmath/index.html)
- repos: [github](https://github.com/smushy64/fmath)
- [crates io](https://crates.io/crates/fmath)
- [changelog](https://github.com/smushy64/fmath/blob/main/CHANGELOG.md)
- [to-do list](https://github.com/smushy64/fmath/blob/main/TODO.md)

## How to use

### Include

in Cargo.toml file...
```rust
  [dependencies]
  fmath = "0.2.4"
```

### Documentation

build documentation with *cargo doc* in the command line ( --open to open in browser )
```
  cargo doc
```

## Example
```rust

use fmath::{
    types::{
        *, color::*
    },
    functions::angles::degrees_to_radians
};

let v1 = Vector2::new_left();
let v2 = Vector2::new_right();
let v3 = Vector2::new( 0.25, 0.247 );

let vector2_addition = v1 + v3;
let mut expected_result  = Vector2::new( -0.75, 0.247 );

println!("v1: {}, v2: {}, v3: {}\nv1 + v3 = {}", v1, v2, v3, vector2_addition);
assert!( vector2_addition == expected_result );

let vector2_subtraction = Vector2::new(0.8743, 123.523) - Vector2::new(12.52, 47.373);
expected_result = Vector2::new(-11.6457, 76.15 );
assert!( vector2_subtraction == expected_result );

let vector2_multiplication = Vector2::new(235.2, 6531.122) * 3.1;
expected_result = Vector2::new( 729.12, 20246.4782 );
assert!( vector2_multiplication == expected_result );

let translate = Vector3::new( 0.1, 4.2, 22.0 );
let euler_rotation = Vector3::new(
    degrees_to_radians( 90.0 ),
    degrees_to_radians( 45.0 ),
    degrees_to_radians( 15.0 ),
);
let scale = Vector3::new_one();

let trs = Matrix4x4::new_trs(
    translate.as_array(),
    euler_rotation.as_array(),
    scale.as_array()
);

let point = Vector3::new( 1.2, 0.5, -2.0 );
println!( "{}", trs.mul_vector3( &point ) );

// COLORS ======================================================

let mut color = RGB::from_hex( "#44FF00" ).unwrap();
println!("{}", color); // print as 8-bit values
color.set_a_f32( 0.5 ); // set alpha to 1.0
println!("{}", color.format_as_float_rgba()); // print as floats

let hsv = HSV::new( 251.0, 0.52, 0.77 );
println!("{}", hsv);
println!("HSV as {}", hsv.as_rgb());


```