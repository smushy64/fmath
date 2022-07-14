use core::fmt;
use core::ops::{ Add, Sub, Mul, Div, Index, IndexMut };
use crate::structs::Quaternion;
use crate::{
    structs::vector::{
        add_components,
        sub_components,
        scale_components,
        Vector4,
        Vector3,
    },
    functions::{
        array_f32_to_le_bytes,
        array_f32_to_be_bytes,
    },
};

use super::{Matrix3x3, determinant3x3};

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
/// Implements `Copy`, `Clone`, `Debug`, `Default`
/// 
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Matrix4x4 {
    pub(crate) cells:[f32;16]
}

impl Matrix4x4 {

    /// Creates a new `Matrix4x4` with all cells set to **0.0**
    pub fn new_zero() -> Self { Self { cells: [0.0f32;16] } }

    /// Creates a new `Matrix4x4` with diagonal cells set to **1.0**
    pub fn new_identity() -> Self {
        Self {
            cells: [
                1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0,
            ]
        }
    }

    /// Set cells to **0.0**
    pub fn clear(&mut self) { self.cells = [0f32;16]; }

    /// Creates a new `Matrix4x4` for creating a view matrix
    pub fn new_look_at( position:&Vector3, target:&Vector3, up:&Vector3 ) -> Self {
        let mut zaxis = ( *target - *position ).normalized();
        let xaxis = Vector3::cross_product(zaxis, *up).normalized();
        let yaxis = Vector3::cross_product(xaxis, zaxis);

        zaxis = -zaxis;

        Self::from_array([
            xaxis[0], yaxis[0], zaxis[0], 0.0,
            xaxis[1], yaxis[1], zaxis[1], 0.0,
            xaxis[2], yaxis[2], zaxis[2], 0.0,

            // row 3
            -Vector3::dot_product( xaxis, *position ),
            -Vector3::dot_product( yaxis, *position ),
            -Vector3::dot_product( zaxis, *position ),
            1.0
        ])
    }

    /// Creates a new `Matrix4x4` for **orthographic** projection
    pub fn new_orthographic_projection(
        left:  f32, right:f32,
        bottom:f32, top:  f32,
        near:  f32, far:  f32,
    ) -> Self {
        let mut result = Self::new_identity();
        result[ 0] =  2.0 / ( right - left );
        result[ 5] =  2.0 / ( top - bottom );
        result[10] = -2.0 / ( far - near );
        result[12] = -( right + left   ) / ( right - left   );
        result[13] = -( top   + bottom ) / ( top   - bottom );
        result[14] = -( far   + near   ) / ( far   - near   );
        result
    }

    /// Creates a new `Matrix4x4` for **perspective** projection
    pub fn new_perspective_projection(
        fov_rad:f32,
        aspect_ratio:f32,
        near:f32, far:f32,
    ) -> Self {
        let mut result = Self::new_zero();
        result[ 0] = 1.0 / ( aspect_ratio * ( fov_rad / 2.0 ).tan() );
        result[ 5] = 1.0 / ( ( fov_rad / 2.0 ).tan() );
        result[10] = -(( far + near ) / ( far - near ));
        result[11] = -1.0;
        result[14] = -(( 2.0 * far * near ) / ( far - near ));
        result
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates
    /// 
    /// `t`: translation
    /// 
    /// `r`: quaternion rotation
    /// 
    /// `s`: scale
    pub fn new_trs( t:&Vector3, r:&Quaternion, s:&Vector3 ) -> Self {
        return 
            Self::new_translate_from_array( t.as_slice() ) *
            Self::new_rotate_from_array(    r.as_array() ) *
            Self::new_scale_from_array(     s.as_slice() );
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates with default values
    /// 
    /// zero translation
    /// 
    /// zero rotation
    /// 
    /// one scale
    pub fn new_trs_default() -> Self {
        Self::new_trs_from_array_euler( [0f32;3], [0f32;3], [1f32;3])
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates
    /// 
    /// `t`: translation
    /// 
    /// `r`: euler rotation in **Radians**
    /// 
    /// `s`: scale
    pub fn new_trs_euler( t:&Vector3, r:&Vector3, s:&Vector3 ) -> Self {
        return 
            Self::new_translate_from_array(    t.as_slice() ) *
            Self::new_rotate_from_array_euler( r.as_slice() ) *
            Self::new_scale_from_array(        s.as_slice() );
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates
    /// 
    /// `t`: translation
    /// 
    /// `r`: quaternion rotation
    /// 
    /// `s`: scale
    pub fn new_trs_from_array( t:[f32;3], r:[f32;4], s:[f32;3] ) -> Self {
        return 
            Self::new_translate_from_array(&t) *
            Self::new_rotate_from_array(&r) *
            Self::new_scale_from_array(&s);
    }

    /// Creates a new `Matrix4x4` for **transforming** coordinates
    /// 
    /// `t`: translation
    /// 
    /// `r`: euler rotation in **Radians**
    /// 
    /// `s`: scale
    pub fn new_trs_from_array_euler( t:[f32;3], r:[f32;3], s:[f32;3] ) -> Self {
        return 
            Self::new_translate_from_array(&t) *
            Self::new_rotate_from_array_euler(&r) *
            Self::new_scale_from_array(&s);
    }

    /// Creates a new `Matrix4x4` for **translating** coordinates
    pub fn new_translate_from_array( t:&[f32] ) -> Self {
        let mut result = Self::new_identity();

        // data indeces( 12, 13, 14 ) are where translation values go
        result.cells[12] = t[0];
        result.cells[13] = t[1];
        result.cells[14] = t[2];

        return result;
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates from [w,x,y,z]
    /// 
    /// Angles are in **Radians**
    pub fn new_rotate_from_array( r:&[f32] ) -> Self {
        let mut m = Self::new_identity();
        let sqr = |v:f32| -> f32 { v * v };
        m[0] = 1.0 - ( 2.0 * sqr( r[2] ) ) - ( 2.0 * sqr( r[3] ) );
        m[1] = ( 2.0 * ( r[1] * r[2] ) ) + ( 2.0 * ( r[0] * r[3] ) );
        m[2] = ( 2.0 * ( r[1] * r[3] ) ) - ( 2.0 * ( r[0] * r[2] ) );

        m[4] = ( 2.0 * ( r[1] * r[2] ) ) - ( 2.0 * ( r[0] * r[3] ) );
        m[5] = 1.0 - ( 2.0 * sqr( r[1] ) ) - ( 2.0 * sqr( r[3] ) );
        m[6] = ( 2.0 * ( r[2] * r[3] ) ) + ( 2.0 * ( r[0] * r[1] ) );

        m[8]  = ( 2.0 * ( r[1] * r[3] ) ) + ( 2.0 * ( r[0] * r[2] ) );
        m[9]  = ( 2.0 * ( r[2] * r[3] ) ) - ( 2.0 * ( r[0] * r[1] ) );
        m[10] = 1.0 - ( 2.0 * sqr( r[1] ) ) - ( 2.0 * sqr( r[2] ) );

        m
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates from *euler angles*
    /// 
    /// Angles are in **Radians**
    pub fn new_rotate_from_array_euler( r:&[f32] ) -> Self {
        Self::new_x_rotate_euler(r[0]) *
        Self::new_y_rotate_euler(r[1]) *
        Self::new_z_rotate_euler(r[2])
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates around *x axis*
    /// 
    /// Angle is in **Radians**
    pub fn new_x_rotate_euler( theta_rad:f32 ) -> Self {
        let mut result = Self::new_identity();

        result.cells[5]  =  theta_rad.cos();
        result.cells[6]  =  theta_rad.sin();
        result.cells[9]  = -theta_rad.sin();
        result.cells[10] =  theta_rad.cos();

        return result;
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates around *y axis*
    /// 
    /// Angle is in **Radians**
    pub fn new_y_rotate_euler( theta_rad:f32 ) -> Self {
        let mut result = Self::new_identity();

        result.cells[0]  =  theta_rad.cos();
        result.cells[2]  = -theta_rad.sin();
        result.cells[8]  =  theta_rad.sin();
        result.cells[10] =  theta_rad.cos();

        return result;
    }

    /// Creates a new `Matrix4x4` for **rotating** coordinates around *z axis*
    /// 
    /// Angle is in **Radians**
    pub fn new_z_rotate_euler( theta_rad:f32 ) -> Self {
        let mut result = Self::new_identity();

        result.cells[0] =  theta_rad.cos();
        result.cells[1] =  theta_rad.sin();
        result.cells[4] = -theta_rad.sin();
        result.cells[5] =  theta_rad.cos();

        return result;
    }

    /// Creates a new `Matrix4x4` for scaling coordinates
    pub fn new_scale_from_array( s:&[f32] ) -> Self {
        let mut result = Self::new_identity();

        // data indeces( 0, 5, 10 ) are where scale values go
        result.cells[0]  = s[0];
        result.cells[5]  = s[1];
        result.cells[10] = s[2];
        
        return result;
    }



    /// Creates `Matrix4x4` from `array` in *column-major* order
    pub fn from_array(array:[f32;16]) -> Self {
        Self { cells:array }
    }

    /// Creates `Matrix4x4` from `Vec` in *column-major* order
    pub unsafe fn from_vec_unchecked(vec:Vec<f32>) -> Self {
        Self{
            cells:[
                vec[0],  vec[1],  vec[2],  vec[3],
                vec[4],  vec[5],  vec[6],  vec[7],
                vec[8],  vec[9],  vec[10], vec[11],
                vec[12], vec[13], vec[14], vec[15],
            ]
        }
    }

    /// Creates `Matrix4x4` in *column-major* order from `Vec` in *row-major* order
    /// 
    /// *Column-major* order is preferred as it's how the underlying
    /// data is actually ordered.
    pub unsafe fn from_vec_unchecked_row_major(vec:Vec<f32>) -> Self {
        Self{
            cells:[
                vec[0], vec[4],  vec[8], vec[12],
                vec[1], vec[5],  vec[9], vec[13],
                vec[2], vec[6], vec[10], vec[14],
                vec[3], vec[7], vec[11], vec[15],
            ]
        }
    }

    /// Creates `Matrix4x4` in *column-major* order from `array` in *row-major* order
    /// 
    /// *Column-major* order is preferred as it's how the underlying
    /// data is actually ordered.
    pub fn from_array_row_major(array:[f32;16]) -> Self {
        Self{
            cells:[
                array[0], array[4],  array[8], array[12],
                array[1], array[5],  array[9], array[13],
                array[2], array[6], array[10], array[14],
                array[3], array[7], array[11], array[15],
            ]
        }
    }

    /// Returns: `reference` to data `array` in *column-major* order
    pub fn as_array(&self) -> &[f32;16] { &self.cells }

    /// Returns: `mutable reference` to data `array` in *column-major* order
    pub fn as_mut_array(&mut self) -> &mut[f32;16] { &mut self.cells }

    /// Returns: `ptr` to data `array`
    pub fn as_ptr(&self) -> *const f32 { self.cells.as_ptr() }

    /// Returns: mut `ptr` to data `array`
    pub fn as_mut_ptr(&mut self) -> *mut f32 { self.cells.as_mut_ptr() }

    /// Create `Matrix3x3` from `Matrix4x4`
    /// 
    /// Last column and row is lost
    pub fn as_matrix3x3( &self ) -> Matrix3x3 {
        Matrix3x3 {
            cells:[
                self[0], self[1],  self[2],
                self[4], self[5],  self[6],
                self[8], self[9], self[10],
            ]
        }
    }

    /// Returns: **new** `array` of data in *row-major* order
    /// 
    /// *Column-major* ordering is preferred as that is how the underlying data is ordered.
    pub fn as_array_row_major(&self) -> [f32;16] {
        [
            self.cells[0], self.cells[4],  self.cells[8], self.cells[12],
            self.cells[1], self.cells[5],  self.cells[9], self.cells[13],
            self.cells[2], self.cells[6], self.cells[10], self.cells[14],
            self.cells[3], self.cells[7], self.cells[11], self.cells[15],
        ]
    }

    /// Returns: `Vec<u8> (64)` **little-endian** byte representation of data in *column-major* order
    pub fn to_le_bytes( &self ) -> Vec<u8> {
        array_f32_to_le_bytes( &self.cells )
    }

    /// Returns: `Vec<u8> (64)` **little-endian** byte representation of data in *row-major* order
    pub fn to_le_bytes_row_major( &self ) -> Vec<u8> {
        Self::from_array( self.as_array_row_major() ).to_le_bytes()
    }

    /// Returns: `Vec<u8> (64)` **big-endian** byte representation of data in *column-major* order
    pub fn to_be_bytes( &self ) -> Vec<u8> {
        array_f32_to_be_bytes( &self.cells )
    }

    /// Returns: `Vec<u8> (64)` **big-endian** byte representation of data in *row-major* order
    pub fn to_be_bytes_row_major( &self ) -> Vec<u8> {
        Self::from_array( self.as_array_row_major() ).to_be_bytes()
    }

    /// Returns: slice of rows at given column index
    pub fn get_column( &self, idx:usize ) -> &[f32] {
        debug_assert!(idx < 4);
        let i = 4 * idx;
        &self.cells[i..i+4]
    }

    /// Returns: new `array` with references to values in input row index
    pub fn get_row( &self, idx:usize ) -> [&f32;4] {
        debug_assert!(idx < 4);
        [ &self.cells[idx], &self.cells[idx + 4], &self.cells[idx + 8], &self.cells[idx + 12] ]
    }

    /// Switch `columns` with `rows`
    pub fn transpose_self( &mut self ) { self.cells = self.as_array_row_major(); }

    /// Switch `columns` with `rows`
    /// 
    /// Returns: new `Matrix4x4` with data from input with `columns` and `rows` switched
    pub fn transpose( m:Self ) -> Self { Self::from_array( m.as_array_row_major() ) }

    /// Index `Matrix4x4` with `row` and `column` index instead of 1D index
    /// 
    /// Returns: value at given index
    pub fn nm_index( &self, row:usize, column:usize ) -> f32 {
        debug_assert!(row < 4 && column < 4);
        self[row + (column * 4)]
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
        Vector4::new(
            ( self[0] * v[0] ) + ( self[4] * v[1] ) + ( self[8]  * v[2] ) + ( self[12] * v[3] ),
            ( self[1] * v[0] ) + ( self[5] * v[1] ) + ( self[9]  * v[2] ) + ( self[13] * v[3] ),
            ( self[2] * v[0] ) + ( self[6] * v[1] ) + ( self[10] * v[2] ) + ( self[14] * v[3] ),
            ( self[3] * v[0] ) + ( self[7] * v[1] ) + ( self[11] * v[2] ) + ( self[15] * v[3] ),
        )
    }

    /// Multiply `Vector3` by `Matrix4x4`
    /// 
    /// Adds **1.0** to end of `Vector3` when calculating result ( *homogenous coordinate* )
    /// 
    /// Returns: `Vector3`
    pub fn mul_vector3( &self, v:&Vector3 ) -> Vector3 {
        Vector3::new(
            ( self[0] * v[0] ) + ( self[4] * v[1] ) + ( self[8]  * v[2] ) + self[12],
            ( self[1] * v[0] ) + ( self[5] * v[1] ) + ( self[9]  * v[2] ) + self[13],
            ( self[2] * v[0] ) + ( self[6] * v[1] ) + ( self[10] * v[2] ) + self[14],
        )
    }

    /// Calculate determinant
    pub fn determinant( &self ) -> f32 {
        ( self[0]  * determinant3x3( self.nm_submatrix( 0, 0 ).as_array() ) ) -
        ( self[4]  * determinant3x3( self.nm_submatrix( 0, 1 ).as_array() ) ) +
        ( self[8]  * determinant3x3( self.nm_submatrix( 0, 2 ).as_array() ) ) -
        ( self[12] * determinant3x3( self.nm_submatrix( 0, 3 ).as_array() ) ) 
    }

    /// Returns: submatrix `Matrix3x3` of a given cell index
    pub fn nm_submatrix( &self, row:usize, column:usize ) -> Matrix3x3 {
        debug_assert!(row < 4 && column < 4);

        let mut buffer:Vec<f32> = Vec::with_capacity( 9 );

        let collect = | buffer:&mut Vec<f32>, column_slice:&[f32] | {
            for ( r, cell ) in column_slice.iter().enumerate() {
                if r != row { buffer.push( cell.clone() ); }
            }
        };

        let mut col = 0;
        while col < 4 {
            if col == column { col += 1; continue; }
            collect( &mut buffer, self.get_column( col ) );
            col += 1;
        }

        unsafe { Matrix3x3::from_vec_unchecked( buffer ) }
    }

    /// Returns: submatrix `Matrix3x3` of given a cell index
    pub fn submatrix( &self, idx:usize ) -> Matrix3x3 {
        debug_assert!(idx < 16);
        self.nm_submatrix( idx / 4, idx % 4 )
    }

    /// Returns: Minor of a given cell index
    pub fn nm_minor( &self, row:usize, column:usize ) -> f32 {
        debug_assert!(row < 4 && column < 4);
        self.nm_submatrix( row, column ).determinant()
    }

    /// Returns: Minor of a  given cell index
    pub fn minor( &self, idx:usize ) -> f32 {
        debug_assert!(idx < 16);
        self.submatrix( idx ).determinant()
    }

    /// Returns: Cofactor of a given cell index
    pub fn nm_cofactor_idx( &self, row:usize, column:usize ) -> f32 {
        debug_assert!(row < 4 && column < 4);
        let minor = self.nm_minor( row, column );
        (-1f32).powi( (( row + 1 ) + ( column + 1 )) as i32 ) * minor
    }

    /// Returns: Cofactor of a given cell index
    pub fn cofactor_idx( &self, idx:usize ) -> f32 {
        debug_assert!(idx < 16);
        self.nm_cofactor_idx( idx / 4, idx % 4 )
    }

    /// Returns: Cofactor `Matrix4x4`
    pub fn cofactor_matrix( &self ) -> Self {
        // create matrix from cofactors( minor matrix * -1^( row + column ) )
        let mut cofactor_buffer:[f32;16] = [0f32;16];
        let mut idx = 0;
        while idx < 16 {
            let ( row, column ) = ( idx / 4, idx % 4 );
            cofactor_buffer[idx] = self.nm_cofactor_idx( row, column );
            idx += 1;
        }

        Self::from_array_row_major( cofactor_buffer )
    }

    /// Returns: Adjoint of matrix
    pub fn adjoint( &self ) -> Self {
        Self::transpose( self.cofactor_matrix() )
    }

    /// Returns: Inverse of matrix if the determinant is **not** 0.0
    pub fn inverse( &self ) -> Option<Self> {

        let determinant = self.determinant();

        if determinant == 0.0 { return None; }
        else { return Some( self.adjoint() / determinant ); }

    }

}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Matrix4x4 [\n" )?;
        let rows = [self.get_row(0), self.get_row(1), self.get_row(2), self.get_row(3)];
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

impl Index<usize> for Matrix4x4 {
    type Output = f32;

    fn index( &self, index:usize ) -> &f32 {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut( &mut self, index:usize ) -> &mut f32 {
        &mut self.cells[index]
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
            cells:[
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

impl Default for Matrix4x4 {
    fn default() -> Self { Self::new_identity() }
}
