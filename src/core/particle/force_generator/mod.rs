pub mod anchored_spring;
pub mod bangee;
pub mod buoyancy;
pub mod drag;
pub mod gravity;
pub mod spring;

use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

pub trait ForceGenerator {
    fn update_force<P: ParticleTrait>(&self, particle: &mut P, duration: Real);
}
