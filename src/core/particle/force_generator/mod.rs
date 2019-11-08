pub mod gravity;
pub mod drag;

use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

pub trait ForceGenerator {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, duration: Real);
}
