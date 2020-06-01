use crate::types::Real;
use crate::vector::Vec3;
use crate::matrix::Matrix4;
use crate::quaternion::Quaternion;

pub struct RigidBody {
    inverse_mass: Real,
    linear_dumping: Real,
    position: Vec3,
    orientation: Quaternion,
    velocity: Vec3,
    // holds the angular velocity
    rotation: Vec3,
    /**
    * Holds a transform matrix for converting body space into
    * world space and vice versa. This can be achieved by calling
    * the getPointIn*Space functions.
    */
    transform_matrix: Matrix4,
}

impl RigidBody {
    // TODO clean function?
    pub fn calculate_derived_data(&mut self) {
        self.orientation.normalize();
        self.calculate_transform_matrix()
    }

    fn calculate_transform_matrix(&mut self) {

    }
}
