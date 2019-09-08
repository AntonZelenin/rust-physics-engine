pub mod core;

use crate::core::vector::Vec3;

fn main() {
    let mut vec1 = Vec3::new().set_values(0.0, 1.0, 2.0).build();
    let vec2 = Vec3::new().set_values(2.0, 1.0, 0.0).build();
    vec1 += vec2;
    println!("{:?}", vec1);
}
