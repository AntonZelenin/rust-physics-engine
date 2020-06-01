use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

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

impl<'a, PT: ParticleTrait> ForceGenerator for Spring<'a, PT> {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, _duration: Real) {
        // calculate the vector of the spring_cube
        let mut force = particle.get_position() - self.other.get_position();
        let magnitude = self.spring_constant * (force.magnitude() - self.rest_length).abs();
        force.normalize();
        particle.add_force(&force * -magnitude);
    }
}
