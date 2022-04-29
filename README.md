# fmath
Math Library for my personal Rust Projects

## About

This is **not** a general-purpose math library. It is specifically written for my purposes only.

Feel free to use it in your projects if it fits your needs, don't even worry about attribution :)

- [documentation](https://docs.rs/fmath/latest/fmath/index.html)
- [repo](https://github.com/smushy64/fmath)
- [crates page](https://crates.io/crates/fmath)
- [change log](https://github.com/smushy64/fmath/blob/main/CHANGELOG.md)
- [to-do list](https://github.com/smushy64/fmath/blob/main/TODO.md)

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

## Example
```rust

use fmath::{
    types::{
        vector::*,
        matrix::*,
        color::*,
        Angle
    },
    functions::{
        *,
        hexadecimal::*,
        angles::*,
    }
};

// Vector
let v1 = Vector2::new( 1.2, 3.4 ); // Vector2: 1.2, 3.4
let v2 = Vector2::new( 4.5, 6.7 ); // Vector2: 4.5, 6.7
let v3 = v1 + v2; // Vector2: 5.7, 10.1
let dot_prod = Vector2::dot( &v1, &v2 ); // v1 ⋅ v2

// Matrix4x4
let m1 = Matrix4x4::new_identity();
let m2 = Matrix4x4::new_translate(&[0.2, 1.0, 2.0]);
let m3 = m1 * m2; // Matrix multiplication

let mut v4 = Vector3::new(1.0, 1.0, 1.0);
// multiply vector by matrix, m3 is a translation matrix
// result = Vector3: 1.2, 2.0, 3.0
v4 = Matrix4x4::mul_vector3(&m3, &v4);

// Color
let rgb = RGB::new_yellow();
let hsv = HSV::from_rgb(rgb);
println!("{}", hsv); // 60, 1.0, 1.0
println!("{}", RGB::from(hsv)); // 255, 255, 0.0, 255

let rgb_2 = RGB::from_hex("994cd4").unwrap();

println!("{}", rgb); // 153, 77, 212


```