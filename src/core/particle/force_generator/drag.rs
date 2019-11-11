use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

struct Drag {
    // velocity drag coefficient
    k1: Real,
    // velocity squared drag coefficient
    k2: Real,
}

impl Drag {
    pub fn new(k1: Real, k2: Real) -> Self {
        Self { k1, k2 }
    }
}

impl ForceGenerator for Drag {
    fn update_force<P: ParticleTrait>(&self, particle: &mut P, _duration: Real) {
        let mut force = particle.get_velocity();
        let magnitude = force.magnitude();
        let drag_coefficient = self.k1 * magnitude + self.k2 * magnitude * magnitude;

        force.normalize();
        force *= -drag_coefficient;
        particle.add_force(force);
    }
}
