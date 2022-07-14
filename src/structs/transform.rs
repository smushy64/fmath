use super::{ Vector3, Quaternion, Matrix3x3, Matrix4x4 };
use std::cell::{ RefCell, Ref };
use core::fmt;

/// Transform matrix from position, rotation and scale
#[derive(Debug, Clone)]
pub struct Transform {
    position: Vector3,
    rotation: Quaternion,
    scale:    Vector3,

    matrix:        RefCell< Matrix4x4 >,
    normal_matrix: RefCell< Matrix3x3 >,
    matrix_dirty:  RefCell< bool >,
    normal_dirty:  RefCell< bool >,

    basis_forward: RefCell< Option<Vector3> >,
    basis_right:   RefCell< Option<Vector3> >,
    basis_up:      RefCell< Option<Vector3> >,
}
impl Transform {
    /// Create new transform
    pub fn new( position:Vector3, rotation:Quaternion, scale:Vector3 ) -> Self {
        let matrix = Matrix4x4::new_trs( &position, &rotation, &scale );
        let normal = match Matrix3x3::new_normal_matrix( &matrix ) {
            Some(result) => result,
            None => Matrix3x3::new_identity(),
        };

        let basis_forward = rotation * Vector3::new_forward();
        let basis_right   = rotation * Vector3::new_right();
        let basis_up      = rotation * Vector3::new_up();

        Self {
            position, rotation, scale,
            matrix:        RefCell::new( matrix ),
            normal_matrix: RefCell::new( normal ),
            matrix_dirty:  RefCell::new( false ),
            normal_dirty:  RefCell::new( false ),

            basis_forward: RefCell::new( Some( basis_forward ) ),
            basis_right:   RefCell::new( Some( basis_right ) ),
            basis_up:      RefCell::new( Some( basis_up ) ),
        }
    }

    /// Position
    pub fn position(&self) -> Vector3 { self.position }
    /// Position mutable
    pub fn position_mut(&mut self) -> &mut Vector3 { self.set_matrices_dirty(); &mut self.position }
    /// Set position
    pub fn set_position(&mut self, new_position:Vector3) {
        self.position = new_position;
        self.set_matrices_dirty();
    }
    /// Apply translation
    pub fn translate( &mut self, delta:Vector3 ) {
        self.set_position( self.position() + delta );
    }

    /// Rotation
    pub fn rotation(&self) -> Quaternion { self.rotation }
    /// Rotation mutable
    pub fn rotation_mut(&mut self) -> &mut Quaternion { self.set_matrices_dirty(); self.set_basis_dirty(); &mut self.rotation }
    /// Set rotation
    pub fn set_rotation(&mut self, new_rotation:Quaternion) {
        self.rotation = new_rotation;
        self.set_matrices_dirty();
        self.set_basis_dirty();
    }
    /// Apply rotation
    pub fn rotate(&mut self, delta:Quaternion) {
        self.set_rotation( delta * self.rotation() )
    }

    /// Scale
    pub fn scale(&self) -> Vector3 { self.scale }
    /// Scale mutable
    pub fn scale_mut(&mut self) -> &mut Vector3 { self.set_matrices_dirty(); &mut self.scale }
    /// Set scale
    pub fn set_scale(&mut self, new_scale:Vector3) {
        self.scale = new_scale;
        self.set_matrices_dirty();
    }
    /// Apply scaling equally on all axis
    pub fn uniform_scale( &mut self, scalar:f32 ) {
        self.set_scale( self.scale() * scalar )
    }
    /// Apply different scaling on individual axis
    pub fn non_uniform_scale( &mut self, delta:Vector3 ) {
        self.set_scale( self.scale() * delta )
    }

    /// Transform matrix
    pub fn matrix(&self) -> Ref<Matrix4x4> {
        if self.matrix_dirty() { self.recalculate_matrix() }
        self.matrix.borrow()
    }
    /// Normal matrix
    pub fn normal_matrix(&self) -> Ref<Matrix3x3> {
        if self.normal_dirty() { self.recalculate_normal() }
        self.normal_matrix.borrow()
    }

    /// Forward basis vector
    pub fn forward(&self) -> Vector3 {
        if self.forward_dirty() { self.recalculate_forward() };
        (*self.basis_forward.borrow()).unwrap()
    }
    /// Right basis vector
    pub fn right(&self) -> Vector3 {
        if self.right_dirty() { self.recalculate_right() };
        (*self.basis_right.borrow()).unwrap()
    }
    /// Up basis vector
    pub fn up(&self) -> Vector3 {
        if self.up_dirty() { self.recalculate_up() };
        (*self.basis_up.borrow()).unwrap()
    }

    fn recalculate_matrix(&self) {
        let mut matrix = self.matrix.borrow_mut();
        *matrix = Matrix4x4::new_trs(&self.position, &self.rotation, &self.scale);
        
    }
    fn recalculate_normal(&self) {
        let matrix = self.matrix.borrow();
        let mut normal = self.normal_matrix.borrow_mut();
        *normal = match Matrix3x3::new_normal_matrix( &matrix ) {
            Some(result) => result,
            None => *normal,
        }
    }
    fn matrix_dirty(&self) -> bool { *self.matrix_dirty.borrow() }
    fn normal_dirty(&self) -> bool { *self.normal_dirty.borrow() }
    fn set_matrices_dirty(&self) {
        let mut matrix_dirty = self.matrix_dirty.borrow_mut();
        *matrix_dirty = true;
        let mut normal_dirty = self.normal_dirty.borrow_mut();
        *normal_dirty = true;
    }
    fn set_basis_dirty(&self) {
        let mut forward = self.basis_forward.borrow_mut();
        *forward = None;
        let mut right = self.basis_right.borrow_mut();
        *right = None;
        let mut up = self.basis_up.borrow_mut();
        *up = None;
    }
    fn forward_dirty(&self) -> bool { self.basis_forward.borrow().is_none() }
    fn right_dirty(&self)   -> bool { self.basis_right.borrow().is_none() }
    fn up_dirty(&self)      -> bool { self.basis_up.borrow().is_none() }
    fn recalculate_forward(&self) {
        let mut forward = self.basis_forward.borrow_mut();
        *forward = Some(self.rotation * Vector3::new_forward());
    }
    fn recalculate_right(&self) {
        let mut right = self.basis_right.borrow_mut();
        *right = Some(self.rotation * Vector3::new_right());
    }
    fn recalculate_up(&self) {
        let mut up = self.basis_up.borrow_mut();
        *up = Some(self.rotation * Vector3::new_up());
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self::new( Default::default(), Default::default(), Vector3::new_one() )
    }
}
impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "[Position: {} Rotation: {} Scale: {}]",
            self.position(), self.rotation().as_euler_angles(), self.scale()
        )
    }
}
