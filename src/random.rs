use crate::vector::Vec3;
use rand::Rng;

pub fn random_vector(min: Vec3, max: Vec3) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::from_values(
        rng.gen_range(min.x, max.x),
        rng.gen_range(min.y, max.y),
        rng.gen_range(min.z, max.z),
    )
}
