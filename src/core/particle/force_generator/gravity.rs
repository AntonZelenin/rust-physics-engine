use crate::core::vector::Vec3;
use crate::core::particle::force_generator::ForceGenerator;
use crate::core::types::Real;
use crate::core::particle::particle_trait::ParticleTrait;

pub struct Gravity {
    gravity: Vec3,
}

impl Gravity {
    pub fn new(gravity: Vec3) -> Gravity {
        Gravity {
            gravity,
        }
    }
}

impl ForceGenerator for Gravity {
    fn update_force<P: ParticleTrait>(&self, particle: &mut P, duration: Real) {
        if particle.is_infinite_mass() {
            return;
        }
        particle.add_force(self.gravity * particle.get_mass());
    }
}
