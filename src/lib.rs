use crate::vector::Vec3;

pub mod matrix;
pub mod particle;
pub mod quaternion;
pub mod random;
pub mod rigid_body;
pub mod timing;
pub mod types;
pub mod vector;

pub const GRAVITY: Vec3 = Vec3 {
    x: 0.0,
    y: -9.81,
    z: 0.0,
    pad: 0.0,
};
