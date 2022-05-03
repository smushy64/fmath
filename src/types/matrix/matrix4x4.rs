use core::fmt::Display;
use core::ops::{
    Add, Sub, Mul, Div, Index, IndexMut
};
use crate::{
    types::vector::{
        add_components,
        sub_components,
        scale_components,
        Vector4,
        Vector3,
    },
};

/// 4 by 4 Matrix in *column-major* order
/// 
/// Indexable with **[ ]** (*as 1D array*)
/// 
/// Use associated fn's `nm_index` and `nm_mut_index` to
/// index with 2D coordinates (*column major*)
/// 
/// The following table shows what cell each index corresponds to
/// 
/// `0`&nbsp;&nbsp;`4`&nbsp;&nbsp;&nbsp;`8`&nbsp;&nbsp;&nbsp;`12`
/// 
/// `1`&nbsp;&nbsp;`5`&nbsp;&nbsp;&nbsp;`9`&nbsp;&nbsp;&nbsp;`13`
/// 
/// `2`&nbsp;&nbsp;`6`&nbsp;&nbsp;`10`&nbsp;&nbsp;`14`
/// 
/// `3`&nbsp;&nbsp;`7`&nbsp;&nbsp;`11`&nbsp;&nbsp;`15`
/// 
/// Implements `Copy`, `Clone`, `Debug`
/// 
#[derive(Copy, Clone, Debug)]
pub struct Matrix4x4 {
    data:[f32;16]
}

impl Matrix4x4 {

    /// Creates a new `Matrix4x4` with all cells set to **0.0**
    pub fn new_zero() -> Self {
        Self {
            data: [
                0.0,0.0,0.0,0.0,
                0.0,0.0,0.0,0.0,
                0.0,0.0,0.0,0.0,
                0.0,0.0,0.0,0.0,
            ]
        }
    }

    /// Creates a new `Matrix4x4` with diagonal cells set to **1.0**
    pub fn new_identity() -> Self {
        Self {
            data: [
                1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0,
            ]
        }
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates
    /// 
    /// `t`: translation
    /// 
    /// `r`: rotation in **Radians**
    /// 
    /// `s`: scale
    pub fn new_trs( t:&[f32;3], r:&[f32;3], s:&[f32;3] ) -> Self {
        return 
            Self::new_translate(t) *
            Self::new_rotate(r) *
            Self::new_scale(s);
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates with default values
    pub fn new_default_trs() -> Self {
        let zero = [0.0, 0.0, 0.0];
        let one = [1.0, 1.0, 1.0];
        Self::new_trs(&zero, &zero, &one)
    }

    /// Creates a new `Matrix4x4` for **translating** coordinates
    pub fn new_translate( t:&[f32;3] ) -> Self {
        let mut result = Self::new_identity();

        // data indeces( 12, 13, 14 ) are where translation values go
        result.data[12] = t[0];
        result.data[13] = t[1];
        result.data[14] = t[2];

        return result;
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates from *euler angles*
    /// 
    /// Angles are in **Radians**
    pub fn new_rotate( r:&[f32;3] ) -> Self {
        Self::new_x_rotate(r[0]) *
        Self::new_y_rotate(r[1]) *
        Self::new_z_rotate(r[2])
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates around *x axis*
    /// 
    /// Angle is in **Radians**
    pub fn new_x_rotate( theta_rad:f32 ) -> Self {
        let mut result = Self::new_identity();

        result.data[5]  =  theta_rad.cos();
        result.data[6]  =  theta_rad.sin();
        result.data[9]  = -theta_rad.sin();
        result.data[10] =  theta_rad.cos();

        return result;
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates around *y axis*
    /// 
    /// Angle is in **Radians**
    pub fn new_y_rotate( theta_rad:f32 ) -> Self {
        let mut result = Self::new_identity();

        result.data[0]  =  theta_rad.cos();
        result.data[2]  = -theta_rad.sin();
        result.data[8]  =  theta_rad.sin();
        result.data[10] =  theta_rad.cos();

        return result;
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates around *z axis*
    /// 
    /// Angle is in **Radians**
    pub fn new_z_rotate( theta_rad:f32 ) -> Self {
        let mut result = Self::new_identity();

        result.data[0] =  theta_rad.cos();
        result.data[1] =  theta_rad.sin();
        result.data[4] = -theta_rad.sin();
        result.data[5] =  theta_rad.cos();

        return result;
    }

    /// Creates a new `Matrix4x4` for scaling coordinates
    pub fn new_scale( s:&[f32;3] ) -> Self {
        let mut result = Self::new_identity();

        // data indeces( 0, 5, 10 ) are where scale values go
        result.data[0]  = s[0];
        result.data[5]  = s[1];
        result.data[10] = s[2];
        
        return result;
    }



    /// Creates `Matrix4x4` from `array` in *column-major* order
    pub fn from_array(array:[f32;16]) -> Self {
        Self { data:array }
    }

    /// Creates `Matrix4x4` from `array` in *row-major* order
    /// 
    /// *Column-major* order is preferred as it's how the underlying
    /// data is actually ordered.
    pub fn from_array_row_major(array:[f32;16]) -> Self {
        Self{
            data:[
                array[0], array[4],  array[8], array[12],
                array[1], array[5],  array[9], array[13],
                array[2], array[6], array[10], array[14],
                array[3], array[7], array[11], array[15],
            ]
        }
    }

    /// Returns: `reference` to data `array` in *column-major* order
    pub fn as_array(&self) -> &[f32;16] {
        &self.data
    }

    /// Returns: `mutable reference` to data `array` in *column-major* order
    pub fn as_mut_array(&mut self) -> &mut[f32;16] {
        &mut self.data
    }

    /// Returns: **new** `array` of data in *row-major* order
    /// 
    /// *Column-major* ordering is preferred as that is how the underlying data is ordered.
    pub fn as_array_row_major(&self) -> [f32;16] {
        [
            self.data[0], self.data[4],  self.data[8], self.data[12],
            self.data[1], self.data[5],  self.data[9], self.data[13],
            self.data[2], self.data[6], self.data[10], self.data[14],
            self.data[3], self.data[7], self.data[11], self.data[15],
        ]
    }

    /// Index `Matrix4x4` with `row` and `column` index instead of 1D index
    /// 
    /// Returns: `reference` to value at given index
    pub fn nm_index( &self, row:usize, column:usize ) -> &f32 {
        debug_assert!(row < 4 && column < 4);
        &self[row + (column * 4)]
    }

    /// Index `Matrix4x4` with `row` and `column` index instead of 1D index
    /// 
    /// Returns: `mutable reference` to value at given index
    pub fn nm_mut_index( &mut self, row:usize, column:usize ) -> &mut f32 {
        debug_assert!(row < 4 && column < 4);
        &mut self[row + (column * 4)]
    }

    /// Multiply `Vector4` by `Matrix4x4`
    /// 
    /// Returns: `Vector4`
    pub fn mul_vector4( &self, v:&Vector4 ) -> Vector4 {
        Vector4::from_array([
            ( self[0] * v[0] ) + ( self[4] * v[1] ) + ( self[8]  * v[2] ) + ( self[12] * v[3] ),
            ( self[1] * v[0] ) + ( self[5] * v[1] ) + ( self[9]  * v[2] ) + ( self[13] * v[3] ),
            ( self[2] * v[0] ) + ( self[6] * v[1] ) + ( self[10] * v[2] ) + ( self[14] * v[3] ),
            ( self[3] * v[0] ) + ( self[7] * v[1] ) + ( self[11] * v[2] ) + ( self[15] * v[3] ),
        ])
    }

    /// Multiply `Vector3` by `Matrix4x4`
    /// 
    /// Adds **1.0** to end of `Vector3` when calculating result ( *homogenous coordinate* )
    /// 
    /// Returns: `Vector3`
    pub fn mul_vector3( &self, v:&Vector3 ) -> Vector3 {
        Vector3::from_array([
            ( self[0] * v[0] ) + ( self[4] * v[1] ) + ( self[8]  * v[2] ) + self[12],
            ( self[1] * v[0] ) + ( self[5] * v[1] ) + ( self[9]  * v[2] ) + self[13],
            ( self[2] * v[0] ) + ( self[6] * v[1] ) + ( self[10] * v[2] ) + self[14],
            // ( m[3] * v[0] ) + ( m[7] * v[1] ) + ( m[11] * v[2] ) + ( m[15] * 1.0 ),
        ])
    }

}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix4x4:\n   {:7.2} {:7.2} {:7.2} {:7.2}\n   {:7.2} {:7.2} {:7.2} {:7.2}\n   {:7.2} {:7.2} {:7.2} {:7.2}\n   {:7.2} {:7.2} {:7.2} {:7.2}",
            self[0], self[4], self[8],  self[12],
            self[1], self[5], self[9],  self[13],
            self[2], self[6], self[10], self[14],
            self[3], self[7], self[11], self[15],
        )
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = f32;

    fn index( &self, index:usize ) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut( &mut self, index:usize ) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Add for Matrix4x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new_zero();
        add_components(self.as_array(), rhs.as_array(), result.as_mut_array() );
        return result;
    }
}

impl Sub for Matrix4x4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new_zero();
        sub_components(self.as_array(), rhs.as_array(), result.as_mut_array() );
        return result;
    }
}

impl Mul<f32> for Matrix4x4 {

    type Output = Self;

    fn mul( self, scalar:f32 ) -> Self::Output {
        let mut result = Self::new_zero();
        scale_components(self.as_array(), scalar, result.as_mut_array());
        return result;
    }

}

impl Div<f32> for Matrix4x4 {
    
    type Output = Self;

    fn div( self, scalar:f32 ) -> Self::Output {
        let mut result = Self::new_zero();
        scale_components(self.as_array(), 1.0 / scalar, result.as_mut_array());
        return result;
    }

}

impl Mul<Self> for Matrix4x4 {

    type Output = Self;

    fn mul( self, _rhs:Self ) -> Self::Output {
        Self {
            data:[
                // column - 0
                ( self[0] * _rhs[0] ) + ( self[4] * _rhs[1] ) + ( self[8]  * _rhs[2] ) + ( self[12] * _rhs[3] ),
                ( self[1] * _rhs[0] ) + ( self[5] * _rhs[1] ) + ( self[9]  * _rhs[2] ) + ( self[13] * _rhs[3] ),
                ( self[2] * _rhs[0] ) + ( self[6] * _rhs[1] ) + ( self[10] * _rhs[2] ) + ( self[14] * _rhs[3] ),
                ( self[3] * _rhs[0] ) + ( self[7] * _rhs[1] ) + ( self[11] * _rhs[2] ) + ( self[15] * _rhs[3] ),
                // column - 1
                ( self[0] * _rhs[4] ) + ( self[4] * _rhs[5] ) + ( self[8]  * _rhs[6] ) + ( self[12] * _rhs[7] ),
                ( self[1] * _rhs[4] ) + ( self[5] * _rhs[5] ) + ( self[9]  * _rhs[6] ) + ( self[13] * _rhs[7] ),
                ( self[2] * _rhs[4] ) + ( self[6] * _rhs[5] ) + ( self[10] * _rhs[6] ) + ( self[14] * _rhs[7] ),
                ( self[3] * _rhs[4] ) + ( self[7] * _rhs[5] ) + ( self[11] * _rhs[6] ) + ( self[15] * _rhs[7] ),
                // column - 2
                ( self[0] * _rhs[8] ) + ( self[4] * _rhs[9] ) + ( self[8]  * _rhs[10] ) + ( self[12] * _rhs[11] ),
                ( self[1] * _rhs[8] ) + ( self[5] * _rhs[9] ) + ( self[9]  * _rhs[10] ) + ( self[13] * _rhs[11] ),
                ( self[2] * _rhs[8] ) + ( self[6] * _rhs[9] ) + ( self[10] * _rhs[10] ) + ( self[14] * _rhs[11] ),
                ( self[3] * _rhs[8] ) + ( self[7] * _rhs[9] ) + ( self[11] * _rhs[10] ) + ( self[15] * _rhs[11] ),
                // column - 3
                ( self[0] * _rhs[12] ) + ( self[4] * _rhs[13] ) + ( self[8]  * _rhs[14] ) + ( self[12] * _rhs[15] ),
                ( self[1] * _rhs[12] ) + ( self[5] * _rhs[13] ) + ( self[9]  * _rhs[14] ) + ( self[13] * _rhs[15] ),
                ( self[2] * _rhs[12] ) + ( self[6] * _rhs[13] ) + ( self[10] * _rhs[14] ) + ( self[14] * _rhs[15] ),
                ( self[3] * _rhs[12] ) + ( self[7] * _rhs[13] ) + ( self[11] * _rhs[14] ) + ( self[15] * _rhs[15] ),
            ]
        }

    }

}
