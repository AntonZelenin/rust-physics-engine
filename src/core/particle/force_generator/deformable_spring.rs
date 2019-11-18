use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

struct DeformableSpring<'a, PT: ParticleTrait> {
    other: &'a PT,
    spring_constant: Real,
    rest_length: Real,
    // the maximum length to which the spring could be stretched before deforming and losing elasticity
    limit_of_elasticity: Real,
    elasticity_loss_coefficient: Real,
    elasticity_divider: Real,
}

impl<'a, PT: ParticleTrait> Spring<'a, PT> {
    pub fn new(
        other: &'a PT,
        spring_constant: Real,
        rest_length: Real,
        limit_of_elasticity: Real,
        max_elasticity_loss_coefficient: Real,
    ) -> Self {
        DeformableSpring {
            other,
            spring_constant,
            rest_length,
            limit_of_elasticity,
            elasticity_loss_coefficient: max_elasticity_loss_coefficient,
            elasticity_divider: 1.0,
        }
    }
}

impl<'a, PT: ParticleTrait> ForceGenerator for DeformableSpring<'a, PT> {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, _duration: Real) {
        // calculate the vector of the spring
        let mut force = particle.get_position() - self.other.get_position();
        let spring_length = force.magnitude();
        if spring_length > self.limit_of_elasticity {
            self.elasticity_divider = self.elasticity_loss_coefficient
                * ((spring_length - self.limit_of_elasticity) / spring_length);
            self.limit_of_elasticity = spring_length;
        }
        let magnitude =
            (self.spring_constant / divider) * (force.magnitude() - self.rest_length).abs();
        // calculate the final force and apply it
        force.normalize();
        particle.add_force(force * -magnitude);
    }
}
