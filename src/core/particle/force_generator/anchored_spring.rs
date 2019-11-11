use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;
use crate::core::vector::Vec3;

pub struct AnchoredSpring<'a> {
    anchor: &'a Vec3,
    spring_constant: Real,
    rest_length: Real,
}

impl<'a> AnchoredSpring<'a> {
    pub fn new(anchor: &'a Vec3, spring_constant: Real, rest_length: Real) -> Self {
        AnchoredSpring {
            anchor,
            spring_constant,
            rest_length,
        }
    }
}

impl<'a> ForceGenerator for AnchoredSpring<'a> {
    fn update_force<P: ParticleTrait>(&self, particle: &mut P, _duration: Real) {
        // calculate the vector of the spring
        let mut force = particle.get_position() - *self.anchor;
        // calculate the magnitude of the force
        let magnitude = self.spring_constant * (force.magnitude() - self.rest_length).abs();
        // calculate the final force and apply it
        force.normalize();
        particle.add_force(force * -magnitude);
    }
}