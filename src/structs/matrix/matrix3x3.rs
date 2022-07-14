use core::fmt;
use core::ops::{ Add, Sub, Mul, Div, Index, IndexMut };
use crate::{
    structs::vector::{
        add_components,
        sub_components,
        scale_components,
        Vector3,
    },
    functions::{
        array_f32_to_le_bytes,
        array_f32_to_be_bytes,
    },
};

use super::{Matrix4x4, determinant3x3};

/// 3 by 3 Matrix in *column-major* order
/// 
/// Indexable with **[ ]** (*as 1D array*)
/// 
/// Use associated fn's `nm_index` and `nm_mut_index` to
/// index with 2D coordinates (*column major*)
/// 
/// The following table shows what cell each index corresponds to
/// 
/// `0`&nbsp;&nbsp;`3`&nbsp;&nbsp;`6`
/// 
/// `1`&nbsp;&nbsp;`4`&nbsp;&nbsp;`7`
/// 
/// `2`&nbsp;&nbsp;`5`&nbsp;&nbsp;`8`
/// 
/// Implements `Copy`, `Clone`, `Debug`, `Default`
/// 
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Matrix3x3 {
    pub(crate) cells:[f32;9]
}

impl Matrix3x3 {

    /// Creates a new `Matrix3x3` with all cells set to **0.0**
    pub fn new_zero() -> Self { Self { cells:[0f32;9] } }

    /// Creates a new `Matrix3x3` with diagonal cells set to **1.0**
    pub fn new_identity() -> Self {
        Self {
            cells: [
                1.0,0.0,0.0,
                0.0,1.0,0.0,
                0.0,0.0,1.0,
            ]
        }
    }

    /// Creates a new `Matrix3x3` for calculating world-space normals from transform matrix
    /// 
    /// Some: if input `Matrix4x4` can be inverted
    /// 
    /// None: if input `Matrix4x4` **cannot** be inverted
    pub fn new_normal_matrix( transform:&Matrix4x4 ) -> Option<Self> {
        match transform.inverse() {
            Some(inv) => Some(
                Matrix4x4::transpose( inv ).as_matrix3x3()
            ),
            None => None,
        }
    }

    /// Set cells to **0.0**
    pub fn clear(&mut self) { self.cells = [0f32;9]; }

    /// Creates `Matrix3x3` from `array` in *column-major* order
    pub fn from_array(array:[f32;9]) -> Self { Self { cells:array } }

    /// Creates `Matrix3x3` from `Vec` in *column-major* order
    pub unsafe fn from_vec_unchecked(vec:Vec<f32>) -> Self {
        Self{
            cells:[
                vec[0],  vec[1],  vec[2], 
                vec[3],  vec[4],  vec[5], 
                vec[6],  vec[7],  vec[8],
            ]
        }
    }

    /// Creates `Matrix3x3` in *column-major* order from `Vec` in *row-major* order
    /// 
    /// *Column-major* order is preferred as it's how the underlying
    /// data is actually ordered.
    pub unsafe fn from_vec_unchecked_row_major(vec:Vec<f32>) -> Self {
        Self{
            cells:[
                vec[0], vec[3], vec[6],
                vec[1], vec[4], vec[7],
                vec[2], vec[5], vec[8],
            ]
        }
    }

    /// Creates `Matrix3x3` in *column-major* order from `array` in *row-major* order
    /// 
    /// *Column-major* order is preferred as it's how the underlying
    /// data is actually ordered.
    pub fn from_array_row_major(array:[f32;9]) -> Self {
        Self{
            cells:[
                array[0], array[3], array[6],
                array[1], array[4], array[7],
                array[2], array[5], array[8],
            ]
        }
    }

    /// Create `Matrix3x3` from `Matrix4x4`
    /// 
    /// Last column and row is lost
    pub fn from_matrix4x4( mat:Matrix4x4 ) -> Self {
        Self{
            cells:[
                mat[0], mat[1],  mat[2],
                mat[4], mat[5],  mat[6],
                mat[8], mat[9], mat[10],
            ]
        }
    }

    /// Returns: `reference` to data `array` in *column-major* order
    pub fn as_array(&self) -> &[f32;9] { &self.cells }

    /// Returns: data `array` in *column-major* order with each column padded with an extra `f32`
    pub fn as_padded_array(&self) -> [f32;12] {
        [
            self[0], self[1], self[2], 0.0,
            self[3], self[4], self[5], 0.0,
            self[6], self[7], self[8], 0.0, 
        ]
    }

    /// Returns: `mutable reference` to data `array` in *column-major* order
    pub fn as_mut_array(&mut self) -> &mut[f32;9] { &mut self.cells }

    /// Returns: `ptr` to data `array`
    pub fn as_ptr(&self) -> *const f32 { self.cells.as_ptr() }

    /// Returns: mut `ptr` to data `array`
    pub fn as_mut_ptr(&mut self) -> *mut f32 { self.cells.as_mut_ptr() }

    /// Returns: **new** `array` of data in *row-major* order
    /// 
    /// *Column-major* ordering is preferred as that is how the underlying data is ordered.
    pub fn as_array_row_major(&self) -> [f32;9] {
        [
            self[0], self[3], self[6],
            self[1], self[4], self[7],
            self[2], self[5], self[8],
        ]
    }

    /// Returns: data `array` in *row-major* order with each column padded with an extra `f32`
    /// 
    /// *Column-major* ordering is preferred as that is how the underlying data is ordered.
    pub fn as_padded_array_row_major(&self) -> [f32;12] {
        [
            self[0], self[3], self[6],
            self[1], self[4], self[7],
            self[2], self[5], self[8],
            0.0,     0.0,     0.0,
        ]
    }

    /// Returns: `Vec<u8> (36)` **little-endian** byte representation of data in *column-major* order
    pub fn to_le_bytes( &self ) -> Vec<u8> {
        array_f32_to_le_bytes( &self.cells )
    }

    /// Returns: `Vec<u8> (36)` **little-endian** byte representation of data in *row-major* order
    pub fn to_le_bytes_row_major( &self ) -> Vec<u8> {
        Self::from_array( self.as_array_row_major() ).to_le_bytes()
    }

    /// Returns: `Vec<u8> (36)` **big-endian** byte representation of data in *column-major* order
    pub fn to_be_bytes( &self ) -> Vec<u8> {
        array_f32_to_be_bytes( &self.cells )
    }

    /// Returns: `Vec<u8> (36)` **big-endian** byte representation of data in *row-major* order
    pub fn to_be_bytes_row_major( &self ) -> Vec<u8> {
        Self::from_array( self.as_array_row_major() ).to_be_bytes()
    }

    /// Returns: `Vec<u8> (48)` **little-endian** byte representation of data in *column-major* order
    /// 
    /// *Each column is padded with an extra 4 bytes*
    pub fn to_le_bytes_padded( &self ) -> Vec<u8> {
        array_f32_to_le_bytes( &self.as_padded_array() )
    }

    /// Returns: `Vec<u8> (48)` **little-endian** byte representation of data in *row-major* order
    /// 
    /// *Each column is padded with an extra 4 bytes*
    pub fn to_le_bytes_row_major_padded( &self ) -> Vec<u8> {
        array_f32_to_le_bytes( &self.as_padded_array_row_major() )
    }

    /// Returns: `Vec<u8> (48)` **big-endian** byte representation of data in *column-major* order
    /// 
    /// *Each column is padded with an extra 4 bytes*
    pub fn to_be_bytes_padded( &self ) -> Vec<u8> {
        array_f32_to_be_bytes( &self.as_padded_array() )
    }

    /// Returns: `Vec<u8> (48)` **big-endian** byte representation of data in *row-major* order
    /// 
    /// *Each column is padded with an extra 4 bytes*
    pub fn to_be_bytes_row_major_padded( &self ) -> Vec<u8> {
        array_f32_to_be_bytes( &self.as_padded_array_row_major() )
    }

    /// Returns: slice of rows at given column index
    pub fn get_column( &self, idx:usize ) -> &[f32] {
        debug_assert!(idx < 3);
        let i = 3 * idx;
        &self.cells[i..i+3]
    }

    /// Returns: new `array` with references to values in input row index
    pub fn get_row( &self, idx:usize ) -> [&f32;3] {
        debug_assert!(idx < 3);
        [ &self.cells[idx], &self.cells[idx + 3], &self.cells[idx + 6] ]
    }

    /// Switch `columns` with `rows`
    pub fn transpose_self( &mut self ) { self.cells = self.as_array_row_major(); }

    /// Switch `columns` with `rows`
    /// 
    /// Returns: new `Matrix3x3` with data from input with `columns` and `rows` switched
    pub fn transpose( m:Self ) -> Self { Self::from_array( m.as_array_row_major() ) }

    /// Index `Matrix3x3` with `row` and `column` index instead of 1D index
    /// 
    /// Returns: value at given index
    pub fn nm_index( &self, row:usize, column:usize ) -> f32 {
        debug_assert!(row < 3 && column < 3);
        self[row + (column * 3)]
    }

    /// Index `Matrix3x3` with `row` and `column` index instead of 1D index
    /// 
    /// Returns: `mutable reference` to value at given index
    pub fn nm_mut_index( &mut self, row:usize, column:usize ) -> &mut f32 {
        debug_assert!(row < 3 && column < 3);
        &mut self[row + (column * 3)]
    }

    /// Multiply `Vector3` by `Matrix3x3`
    /// 
    /// Returns: `Vector3`
    pub fn mul_vector3( &self, v:&Vector3 ) -> Vector3 {
        Vector3::new(
            ( self[0] * v[0] ) + ( self[3] * v[1] ) + ( self[6] * v[2] ),
            ( self[1] * v[0] ) + ( self[4] * v[1] ) + ( self[7] * v[2] ),
            ( self[2] * v[0] ) + ( self[5] * v[1] ) + ( self[8] * v[2] ),
        )
    }

    /// Calculate determinant
    pub fn determinant( &self ) -> f32 {
        determinant3x3( self.as_array() )
    }

}

impl fmt::Display for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Matrix3x3 [\n" )?;
        let rows = [self.get_row(0), self.get_row(1), self.get_row(2)];
        for row in rows.iter() {
            for column in row.iter() {
                write!( f, "{:12.4}, ", column )?;
            }
            write!( f, "\n" )?;
        }
        write!( f, "]" )?;
        Ok(())
    }
}

impl Index<usize> for Matrix3x3 {
    type Output = f32;

    fn index( &self, index:usize ) -> &f32 {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Matrix3x3 {
    fn index_mut( &mut self, index:usize ) -> &mut f32 {
        &mut self.cells[index]
    }
}

impl Add for Matrix3x3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new_zero();
        add_components(self.as_array(), rhs.as_array(), result.as_mut_array() );
        return result;
    }
}

impl Sub for Matrix3x3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new_zero();
        sub_components(self.as_array(), rhs.as_array(), result.as_mut_array() );
        return result;
    }
}

impl Mul<f32> for Matrix3x3 {

    type Output = Self;

    fn mul( self, scalar:f32 ) -> Self::Output {
        let mut result = Self::new_zero();
        scale_components(self.as_array(), scalar, result.as_mut_array());
        return result;
    }

}

impl Div<f32> for Matrix3x3 {
    
    type Output = Self;

    fn div( self, scalar:f32 ) -> Self::Output {
        let mut result = Self::new_zero();
        scale_components(self.as_array(), 1.0 / scalar, result.as_mut_array());
        return result;
    }

}

impl Mul<Self> for Matrix3x3 {

    type Output = Self;

    fn mul( self, _rhs:Self ) -> Self::Output {
        Self {
            cells:[
                // column - 0
                ( self[0] * _rhs[0] ) + ( self[3] * _rhs[1] ) + ( self[6] * _rhs[2] ),
                ( self[1] * _rhs[0] ) + ( self[4] * _rhs[1] ) + ( self[7] * _rhs[2] ),
                ( self[2] * _rhs[0] ) + ( self[5] * _rhs[1] ) + ( self[8] * _rhs[2] ),
                // column - 1
                ( self[0] * _rhs[3] ) + ( self[3] * _rhs[4] ) + ( self[6] * _rhs[5] ),
                ( self[1] * _rhs[3] ) + ( self[4] * _rhs[4] ) + ( self[7] * _rhs[5] ),
                ( self[2] * _rhs[3] ) + ( self[5] * _rhs[4] ) + ( self[8] * _rhs[5] ),
                // column - 2
                ( self[0] * _rhs[6] ) + ( self[3] * _rhs[7] ) + ( self[6] * _rhs[8] ),
                ( self[1] * _rhs[6] ) + ( self[4] * _rhs[7] ) + ( self[7] * _rhs[8] ),
                ( self[2] * _rhs[6] ) + ( self[5] * _rhs[7] ) + ( self[8] * _rhs[8] ),
            ]
        }

    }

}

impl Default for Matrix3x3 {
    fn default() -> Self { Self::new_identity() }
}
