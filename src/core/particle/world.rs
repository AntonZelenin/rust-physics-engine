use crate::core::particle::collision::contact_resolver::ContactResolver;
use crate::core::particle::collision::contact::{ContactGenerator, Contact};
use crate::core::particle::force_registry::ForceRegistry;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::particle::force_generator::ForceGenerator;
use std::marker::PhantomData;
use crate::core::types::Real;

struct World<'a, P, C, FR, FG>
where
    P: ParticleTrait,
    C: ContactGenerator<'a, P>,
    FR: ForceRegistry<P, FG>,
    FG: ForceGenerator
{
    particles: Vec<P>,
    force_registry: FR,
    contact_resolver: ContactResolver,
    contact_generators: Vec<C>,
    contacts: Vec<Contact<'a, P>>,
    max_contacts: usize,
    marker: PhantomData<FG>,
    calculate_iterations: bool,
}

impl<'a, P, C, FR, FG> World<'a, P, C, FR, FG>
where
    P: ParticleTrait,
    C: ContactGenerator<'a, P>,
    FR: ForceRegistry<P, FG>,
    FG: ForceGenerator
{
    pub fn run_physics(&'a mut self, duration: Real) {
        self.force_registry.update_forces(duration);
        self.integrate(duration);
        if self.calculate_iterations {
            self.contact_resolver.set_iterations((self.contacts.len() * 2) as u32);
        }
        let mut contacts = Vec::with_capacity(0);
        for cg in self.contact_generators.iter_mut() {
            // contact generators will already contain particles
            if let Some(contact) = cg.add_contact() {
                // please remove side effects
                contacts.push(contact);
                if self.contacts.len() == self.max_contacts {
                    break;
                }
            }
        }
        self.contact_resolver.resolve_contacts(contacts, duration);
    }

    // unfortunately this doesn't work. If explicit lifetimes are removed - we'll get
    // cannot infer lifetime due to conflicting environments
    //
    // fn generate_contacts(&'a mut self) -> Vec<Contact<'a, P>> {
    //     let mut contacts = Vec::with_capacity(0);
    //     for cg in self.contact_generators.iter_mut() {
    //         // contact generators will already contain particles
    //         if let Some(contact) = cg.add_contact() {
    //             // please remove side effects
    //             contacts.push(contact);
    //             if self.contacts.len() == self.max_contacts {
    //                 break;
    //             }
    //         }
    //     }
    //     contacts
    // }

    fn integrate(&mut self, duration: Real) {
        for particle in self.particles.iter_mut() {
            particle.integrate(duration);
        }
    }
}
