// todo not only for particles?

pub mod air_buoyancy;
pub mod anchored_spring;
pub mod bangee;
pub mod buoyancy;
pub mod deformable_spring;
pub mod drag;
pub mod gravity;
pub mod spring;

use crate::particle::particle_trait::ParticleTrait;
use crate::types::Real;

pub trait ForceGenerator {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, duration: Real);
}
