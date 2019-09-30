use rust_physics_engine::core::vector::Vec3;
use rust_physics_engine::core::particle::Particle;
use rand::prelude::*;
use rust_physics_engine::core::types::Real;

fn main() {
    let mut particle = Particle::new();

    let gravitation = Vec3::new().set_values(0.0, 9.8, 0.0).build();

    let mut rng = rand::thread_rng();
    let mut timer = 0.0;
    let mut duration;
    let mut frames_number = 0;
    while timer < 1.0 {
        frames_number += 1;
        duration = 1.0 / rng.gen_range(60, 70) as Real;
        timer += duration;
        particle.add_acceleration(gravitation);
        particle.integrate(duration);
    }
    println!("Particle velocity is: {}", particle.get_velocity_magnitude());
    println!("Frames number: {}", frames_number);
}
