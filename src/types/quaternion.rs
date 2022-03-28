#![allow(unused_imports)]

use core::ops::{
    Mul, Index, IndexMut
};

use super::{
    vector::{
        Vector3,
        magnitude_components,
    },
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quaternion {
    components:[f32;4]
}

impl Quaternion {

    pub fn new( x:f32, y:f32, z:f32, w:f32 ) -> Self {
        Self { components:[x,y,z,w] }
    }

    pub fn as_euler(&self) -> Vector3 {
        todo!()
    }
}

impl Index<usize> for Quaternion {
    type Output = f32;

    fn index(&self, index:usize) -> &Self::Output {
        &self.components[index]
    }
}

impl IndexMut<usize> for Quaternion {
    fn index_mut(&mut self, index:usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}