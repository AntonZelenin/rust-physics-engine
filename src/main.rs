pub mod core;

use crate::core::vector::Vec3;

fn main() {
    let mut vec3 = Vec3::new().set_values(0.0, 1.0, 2.0).invert().build();
    println!("{:?}", vec3.magnitude());
    vec3.normalize();
    println!("{:?}", vec3.magnitude());
    println!("{:?}", (vec3 * 2.0).magnitude());
}
