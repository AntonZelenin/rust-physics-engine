pub mod gravity;
pub mod drag;
pub mod spring;
pub mod anchored_spring;

use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

pub trait ForceGenerator {
    fn update_force<P: ParticleTrait>(&self, particle: &mut P, duration: Real);
}
