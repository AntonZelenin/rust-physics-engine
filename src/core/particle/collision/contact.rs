use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;
use crate::core::vector::Vec3;

pub trait ContactGenerator<'a, P: ParticleTrait> {
    fn add_contact(&'a mut self) -> Option<Contact<'a, P>>;
}

pub struct Contact<'a, P: ParticleTrait> {
    // particles involved in the contact. The second of these could be null for contacts with the scenery
    pub particles: (&'a mut P, Option<&'a mut P>),
    // normal restitution coefficient at the contact
    pub restitution: Real,
    // the direction of the contact in world coordinates from the first object’s perspective
    pub contact_normal: Vec3,
    // holds the depth of penetration at the contact.
    pub penetration: Real,
}

impl<'a, P: ParticleTrait> Contact<'a, P> {
    pub fn resolve(&mut self, duration: Real) {
        self.resolve_velocity(duration);
        self.resolve_interpenetration();
    }

    fn resolve_velocity(&mut self, duration: Real) {
        let separating_velocity = self.calculate_separating_velocity();
        // the contact is either separating, or stationary, no impulse is required.
        if separating_velocity >= 0.0 {
            return;
        }
        let mut new_sep_velocity = -separating_velocity * self.restitution;
        // Check the velocity buildup due to acceleration only.
        let mut acc_caused_velocity = self.particles.0.get_acceleration();
        if let Some(p) = &mut self.particles.1 {
            acc_caused_velocity -= p.get_acceleration();
        }
        let acc_caused_sep_velocity = &acc_caused_velocity * &self.contact_normal * duration;
        // If we’ve got a closing velocity due to acceleration buildup,
        // remove it from the new separating velocity, it's a resting contact
        if acc_caused_sep_velocity < 0.0 {
            new_sep_velocity += self.restitution * acc_caused_sep_velocity;
            // Make sure we haven’t removed more than was
            // there to remove
            if new_sep_velocity < 0.0 {
                new_sep_velocity = 0.0;
            }
        }
        let delta_velocity = new_sep_velocity - separating_velocity;
        let total_inverse_mass = self.get_total_inverse_mass();
        let total_impulse = delta_velocity / total_inverse_mass;
        let impulse_per_mass = &self.contact_normal * total_impulse;
        self.particles.0.set_velocity(
            self.particles.0.get_velocity()
                + &impulse_per_mass * self.particles.0.get_inverse_mass(),
        );
        if let Some(p) = &mut self.particles.1 {
            p.set_velocity(p.get_velocity() + &impulse_per_mass * p.get_inverse_mass());
        };
    }

    pub fn calculate_separating_velocity(&self) -> Real {
        let relative_velocity = self.particles.0.get_velocity()
            - match &self.particles.1 {
                Some(p) => p.get_velocity(),
                None => Vec3::new(),
            };
        &relative_velocity * &self.contact_normal
    }

    /// When two objects are interpenetrating, we need to move them back
    /// in proportion to their masses
    fn resolve_interpenetration(&mut self) {
        if self.penetration <= 0.0 {
            return;
        }
        let total_inverse_mass = self.get_total_inverse_mass();
        let move_per_mass = &self.contact_normal * (self.penetration / total_inverse_mass);
        // TODO implement append position. Do I need it in trait?
        self.particles.0.set_position(
            self.particles.0.get_position() + &move_per_mass * self.particles.0.get_inverse_mass(),
        );
        if let Some(p) = &mut self.particles.1 {
            p.set_position(p.get_position() - &move_per_mass * p.get_inverse_mass());
        }
    }

    fn get_total_inverse_mass(&self) -> Real {
        self.particles.0.get_inverse_mass()
            + match &self.particles.1 {
                Some(p) => p.get_inverse_mass(),
                None => 0.0,
            }
    }
}
