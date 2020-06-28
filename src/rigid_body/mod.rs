use crate::matrix::{Matrix3, Matrix4};
use crate::quaternion::Quaternion;
use crate::types::Real;
use crate::vector::Vec3;

pub struct RigidBody {
    inverse_mass: Real,
    linear_dumping: Real,
    angular_damping: Real,
    position: Vec3,
    orientation: Quaternion,
    velocity: Vec3,
    acceleration: Vec3,
    // holds the angular velocity
    rotation: Vec3,
    /**
     * Holds a transform matrix for converting body space into
     * world space and vice versa. This can be achieved by calling
     * the getPointIn*Space functions.
     */
    transform_matrix: Matrix4,
    /**
     * Holds the inverse of the body’s inertia tensor. The
     * inertia tensor provided must not be degenerate
     * (that would mean the body had zero inertia for
     * spinning along one axis). As long as the tensor is
     * finite, it will be invertible. The inverse tensor
     * is used for similar reasons to the use of inverse
     * mass.
     *
     * The inertia tensor, unlike the other variables that
     * define a rigid body, is given in body space.
     */
    inverse_inertia_tensor: Matrix3,
    inverse_inertia_tensor_world: Matrix3,
    is_awake: bool,
    force_accum: Vec3,
    torque_accum: Vec3,
}

impl RigidBody {
    pub fn integrate(&mut self, duration: Real) {
        let mut last_frame_acceleration = self.acceleration.clone();
        last_frame_acceleration.add_scaled(&self.force_accum, self.inverse_mass);

        let angular_acceleration = self
            .inverse_inertia_tensor_world
            .transform(&self.torque_accum);

        self.rotation.add_scaled(&angular_acceleration, duration);

        self.velocity *= self.linear_dumping.powf(duration);
        self.rotation *= self.angular_damping.powf(duration);

        self.position.add_scaled(&self.velocity, duration);
        self.orientation.add_scaled_vector(&self.rotation, duration);

        self.calculate_derived_data();
        self.clear_accumulators();
    }

    /**
     * Adds the given force to the given point on the rigid body.
     * The direction of the force is given in world coordinates,
     * but the application point is given in body space. This is
     * useful for spring forces, or other forces fixed to the
     * body.
     */
    pub fn add_force_at_body_point(&mut self, force: &Vec3, point: &Vec3) {
        // Convert to coordinates relative to center of mass.
        self.get_point_in_world_space(point);
        self.add_force_at_point(force, point);
        self.is_awake = true;
    }

    /**
     * Adds the given force to the given point on the rigid body.
     * Both the force and the application point are given in world
     * space. Because the force is not applied at the center of
     * mass, it may be split into both a force and torque.
     */
    pub fn add_force_at_point(&mut self, force: &Vec3, point: &Vec3) {
        let mut p = point.clone();
        p -= self.position;

        self.force_accum += *force;
        self.torque_accum += p % *force;

        self.is_awake = true;
    }

    pub fn get_point_in_world_space(&self, point: &Vec3) -> Vec3 {
        self.transform_matrix.transform(point)
    }

    pub fn clear_accumulators(&mut self) {
        self.force_accum.set_to_zero();
        self.torque_accum.set_to_zero();
    }

    pub fn add_force(&mut self, f: &Vec3) {
        self.force_accum += *f;
        self.is_awake = true;
    }

    // TODO pure function?
    fn calculate_derived_data(&mut self) {
        self.orientation.normalize();
        calculate_transform_matrix(&self.position, &self.orientation);
        // TODO first argument must be inverse_inertia_tensor_world
        transform_inertia_tensor(
            &mut self.inverse_inertia_tensor_world,
            &self.orientation,
            &self.inverse_inertia_tensor,
            &self.transform_matrix,
        )
    }

    pub fn set_inertia_tensor(&mut self, mut inertia_tensor: Matrix3) {
        inertia_tensor.invert();
        self.inverse_inertia_tensor = inertia_tensor;
    }
}

fn calculate_transform_matrix(position: &Vec3, orientation: &Quaternion) -> Matrix4 {
    let mut transform_matrix = Matrix4::new();
    transform_matrix.data[0] =
        1.0 - 2.0 * orientation.j * orientation.j - 2.0 * orientation.k * orientation.k;
    transform_matrix.data[1] =
        2.0 * orientation.i * orientation.j - 2.0 * orientation.r * orientation.k;
    transform_matrix.data[2] =
        2.0 * orientation.i * orientation.k + 2.0 * orientation.r * orientation.j;
    transform_matrix.data[3] = position.x;
    transform_matrix.data[4] =
        2.0 * orientation.i * orientation.j + 2.0 * orientation.r * orientation.k;
    transform_matrix.data[5] =
        1.0 - 2.0 * orientation.i * orientation.i - 2.0 * orientation.k * orientation.k;
    transform_matrix.data[6] =
        2.0 * orientation.j * orientation.k - 2.0 * orientation.r * orientation.i;
    transform_matrix.data[7] = position.y;
    transform_matrix.data[8] =
        2.0 * orientation.i * orientation.k - 2.0 * orientation.r * orientation.j;
    transform_matrix.data[9] =
        2.0 * orientation.j * orientation.k + 2.0 * orientation.r * orientation.i;
    transform_matrix.data[10] =
        1.0 - 2.0 * orientation.i * orientation.i - 2.0 * orientation.j * orientation.j;
    transform_matrix.data[11] = position.z;
    transform_matrix
}

fn transform_inertia_tensor(
    iit_world: &mut Matrix3,
    q: &Quaternion,
    iit_body: &Matrix3,
    rotmat: &Matrix4,
) {
    let t4 = rotmat.data[0] * iit_body.data[0]
        + rotmat.data[1] * iit_body.data[3]
        + rotmat.data[2] * iit_body.data[6];
    let t9 = rotmat.data[0] * iit_body.data[1]
        + rotmat.data[1] * iit_body.data[4]
        + rotmat.data[2] * iit_body.data[7];
    let t14 = rotmat.data[0] * iit_body.data[2]
        + rotmat.data[1] * iit_body.data[5]
        + rotmat.data[2] * iit_body.data[8];
    let t28 = rotmat.data[4] * iit_body.data[0]
        + rotmat.data[5] * iit_body.data[3]
        + rotmat.data[6] * iit_body.data[6];
    let t33 = rotmat.data[4] * iit_body.data[1]
        + rotmat.data[5] * iit_body.data[4]
        + rotmat.data[6] * iit_body.data[7];
    let t38 = rotmat.data[4] * iit_body.data[2]
        + rotmat.data[5] * iit_body.data[5]
        + rotmat.data[6] * iit_body.data[8];
    let t52 = rotmat.data[8] * iit_body.data[0]
        + rotmat.data[9] * iit_body.data[3]
        + rotmat.data[10] * iit_body.data[6];
    let t57 = rotmat.data[8] * iit_body.data[1]
        + rotmat.data[9] * iit_body.data[4]
        + rotmat.data[10] * iit_body.data[7];
    let t62 = rotmat.data[8] * iit_body.data[2]
        + rotmat.data[9] * iit_body.data[5]
        + rotmat.data[10] * iit_body.data[8];
    iit_world.data[0] = t4 * rotmat.data[0] + t9 * rotmat.data[1] + t14 * rotmat.data[2];
    iit_world.data[1] = t4 * rotmat.data[4] + t9 * rotmat.data[5] + t14 * rotmat.data[6];
    iit_world.data[2] = t4 * rotmat.data[8] + t9 * rotmat.data[9] + t14 * rotmat.data[10];
    iit_world.data[3] = t28 * rotmat.data[0] + t33 * rotmat.data[1] + t38 * rotmat.data[2];
    iit_world.data[4] = t28 * rotmat.data[4] + t33 * rotmat.data[5] + t38 * rotmat.data[6];
    iit_world.data[5] = t28 * rotmat.data[8] + t33 * rotmat.data[9] + t38 * rotmat.data[10];
    iit_world.data[6] = t52 * rotmat.data[0] + t57 * rotmat.data[1] + t62 * rotmat.data[2];
    iit_world.data[7] = t52 * rotmat.data[4] + t57 * rotmat.data[5] + t62 * rotmat.data[6];
    iit_world.data[8] = t52 * rotmat.data[8] + t57 * rotmat.data[9] + t62 * rotmat.data[10];
}
