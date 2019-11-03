use crate::fireworks_demo::firework::Firework;
use crate::fireworks_demo::payload::Payload;
use rand::prelude::*;
use rust_physics_engine::core::particle::particle_trait::ParticleTrait;
use rust_physics_engine::core::random;
use rust_physics_engine::core::types::Real;
use rust_physics_engine::core::vector::Vec3;
use rust_physics_engine::core::GRAVITY;

#[derive(Clone)]
pub(crate) struct FireworkRule {
    firework_type: i32,
    min_age: Real,
    max_age: Real,
    min_velocity: Vec3,
    max_velocity: Vec3,
    damping: Real,
    payload_count: u32,
    pub(crate) payloads: Vec<Payload>,
}

impl FireworkRule {
    pub(crate) fn new(
        firework_type: i32,
        min_age: Real,
        max_age: Real,
        min_velocity: Vec3,
        max_velocity: Vec3,
        damping: Real,
        payload_count: u32,
        payloads: Vec<Payload>,
    ) -> Self {
        Self {
            firework_type,
            min_age,
            max_age,
            min_velocity,
            max_velocity,
            damping,
            payload_count,
            payloads,
        }
    }

    pub(crate) fn create(&self, parent: Option<&Firework>) -> Firework {
        let mut firework = Firework::new();
        firework.set_type(self.firework_type);
        firework.age = self.generate_age(self.min_age, self.max_age);

        let mut velocity = Vec3::new();
        match parent {
            Some(parent) => {
                firework.set_position(parent.get_position());
                velocity += parent.get_velocity();
            }
            None => {
                let mut start = Vec3::new();
                let mut rng = rand::thread_rng();
                // form -1 to 2 inclusively, high bound is exclusive
                let x = rng.gen_range(-1.0, 3.0);
                start.x = 5.0 * x;
                firework.set_position(start);
            }
        }
        velocity += random::random_vector(self.min_velocity, self.max_velocity);
        firework.set_velocity(velocity);
        firework.set_mass(1.0);
        firework.set_damping(self.damping);
        firework.add_acceleration(GRAVITY);
//        firework.clear_accumulator();
        firework
    }

    fn generate_age(&self, min: Real, max: Real) -> Real {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
    }
}
