extern crate ordered_float;

use crate::core::particle::collision_resolution::particle_contact::Contact;
use crate::core::types::Real;
use crate::core::particle::particle_trait::ParticleTrait;
use ordered_float::OrderedFloat;

struct ContactResolver {
    iterations: u32,
}

impl ContactResolver {
    fn resolve_contacts<P: ParticleTrait>(&self, contacts: &mut Vec<Contact<P>>, duration: Real) {
        for i in 0..self.iterations {
            let heaviest_contact = contacts.iter_mut().min_by_key(
                // TODO can I avoid OrderedFloat to get rid of a dependency?
                // btw, it should be the slowest place in the engine
                |c| OrderedFloat(c.calculate_separating_velocity())
            );
            if let Some(c) = heaviest_contact {
                c.resolve(duration);
            }
        }
    }
}
