extern crate ordered_float;

use crate::particle::collision::contact::Contact;
use crate::particle::particle_trait::ParticleTrait;
use crate::types::Real;
use ordered_float::OrderedFloat;

pub struct ContactResolver {
    iterations: u32,
    velocity_iterations: u32,
    position_iterations: u32,
}

impl ContactResolver {
    pub fn resolve_contacts<P: ParticleTrait>(
        &mut self,
        mut contacts: Vec<Contact<P>>,
        duration: Real,
    ) {
        for _ in 0..self.iterations {
            let heaviest_contact = contacts.iter_mut().min_by_key(
                // TODO can I avoid OrderedFloat to get rid of a dependency?
                // btw, it should be the slowest place in the engine
                |c| OrderedFloat(c.calculate_separating_velocity()),
            );
            if let Some(c) = heaviest_contact {
                c.resolve(duration);
            }
        }
    }

    pub fn set_iterations(&mut self, iterations: u32) {
        self.velocity_iterations = iterations;
        self.position_iterations = iterations;
    }
}
