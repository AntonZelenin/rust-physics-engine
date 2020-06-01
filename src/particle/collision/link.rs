use crate::particle::collision::contact::{Contact, ContactGenerator};
use crate::particle::particle_trait::ParticleTrait;
use crate::types::Real;

/**
* Links connect two particles together, generating a contact if
* they violate the constraints of their link. It is used
* for cables and rods, and could be used
* for springs with a limit to their extension
*/

struct Cable<'a, P: ParticleTrait> {
    particles: (&'a mut P, &'a mut P),
    max_length: Real,
    restitution: Real,
}

impl<'a, P: ParticleTrait> Cable<'a, P> {
    fn current_length(&self) -> Real {
        (self.particles.0.get_position() - self.particles.1.get_position()).magnitude()
    }
}

impl<'a, P: ParticleTrait> ContactGenerator<'a, P> for Cable<'a, P> {
    fn add_contact(&'a mut self) -> Option<Contact<'a, P>> {
        let length = self.current_length();
        if length <= self.max_length {
            return None;
        }
        let mut normal = self.particles.1.get_position() - self.particles.0.get_position();
        normal.normalize();
        Some(Contact {
            particles: (self.particles.0, Some(self.particles.1)),
            restitution: self.restitution,
            contact_normal: normal,
            penetration: length - self.max_length,
        })
    }
}

// struct Rod<'a, P: ParticleTrait> {
//     particles: (&'a mut P, &'a mut P),
//     length: Real,
// }

// impl<'a, P: ParticleTrait> Rod<'a, P> {
//     fn current_length(&self) -> Real {
//         (self.particles.0.get_position() - self.particles.1.get_position()).magnitude()
//     }
// }
//
// impl<'a, P: ParticleTrait> ContactGenerator<'a, P> for Rod<'a, P> {
//     fn add_contact(&mut self) -> Option<Contact<'a, P>> {
//         let current_length = self.current_length();
//         if current_length == self.length {
//             return None;
//         }
//         let mut normal = self.particles.1.get_position() - self.particles.0.get_position();
//         normal.normalize();
//         let mut penetration = 0.0;
//         if current_length > self.length {
//             penetration = current_length - self.length;
//         } else {
//             normal *= -1.0;
//             penetration = self.length - current_length;
//         }
//         Some(Contact {
//             particles: (self.particles.0, Some(self.particles.1)),
//             restitution: 0.0,
//             contact_normal: normal,
//             penetration,
//         })
//     }
// }
