use crate::particle::force_generator::ForceGenerator;
use crate::particle::particle_trait::ParticleTrait;
use crate::types::Real;
use crate::vector::Vec3;

pub struct Gravity {
    gravity: Vec3,
}

impl Gravity {
    pub fn new(gravity: Vec3) -> Gravity {
        Gravity { gravity }
    }
}

impl ForceGenerator for Gravity {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, _duration: Real) {
        if particle.is_infinite_mass() {
            return;
        }
        particle.add_force(&self.gravity * particle.get_mass());
    }
}
