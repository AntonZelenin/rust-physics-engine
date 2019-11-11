use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

// TODO: maybe should use the same force generator for both particles and cache calculations
struct Spring<'a, PT: ParticleTrait> {
    other: &'a PT,
    spring_constant: Real,
    rest_length: Real,
}

impl<'a, PT: ParticleTrait> Spring<'a, PT> {
    pub fn new(other: &'a PT, spring_constant: Real, rest_length: Real) -> Self {
        Spring {
            other,
            spring_constant,
            rest_length,
        }
    }
}

// TODO: strange trait bounds PT: ParticleTrait, P: ParticleTrait
impl<'a, PT: ParticleTrait> ForceGenerator for Spring<'a, PT> {
    fn update_force<P: ParticleTrait>(&self, particle: &mut P, _duration: Real) {
        // calculate the vector of the spring
        let mut force = particle.get_position() - self.other.get_position();
        // calculate the magnitude of the force
        let magnitude = self.spring_constant * (force.magnitude() - self.rest_length).abs();
        // calculate the final force and apply it
        force.normalize();
        particle.add_force(force * -magnitude);
    }
}
