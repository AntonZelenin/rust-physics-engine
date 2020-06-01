use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

pub struct Bangee<'a, PT: ParticleTrait> {
    other: &'a PT,
    spring_constant: Real,
    rest_length: Real,
}

impl<'a, PT: ParticleTrait> Bangee<'a, PT> {
    pub fn new(other: &'a PT, spring_constant: Real, rest_length: Real) -> Self {
        Bangee {
            other,
            spring_constant,
            rest_length,
        }
    }
}

impl<'a, PT: ParticleTrait> ForceGenerator for Bangee<'a, PT> {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, _duration: Real) {
        // calculate the vector of the spring_cube
        let mut force = particle.get_position() - self.other.get_position();

        let mut magnitude = force.magnitude();
        if magnitude <= self.rest_length {
            return;
        }
        // calculate the magnitude of the force
        magnitude = self.spring_constant * (magnitude - self.rest_length).abs();
        // calculate the final force and apply it
        force.normalize();
        particle.add_force(&force * -magnitude);
    }
}
