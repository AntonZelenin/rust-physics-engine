use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;

pub type Registry<P, F> = Vec<ForceRegistration<P, F>>;

pub trait ForceRegistry<P: ParticleTrait, F: ForceGenerator> {
    fn update_forces(&mut self, duration: Real) {
        self.get_registry()
            .iter_mut()
            .for_each(|fr| fr.force_generator.update_force(&mut fr.particle, duration));
    }

    fn get_registry(&self) -> Registry<P, F>;
    fn add(&mut self, particle: &P, force_generator: &F);
    fn remove(&mut self, particle: &P, force_generator: &F);
    fn clear(&mut self);
}

pub struct ForceRegistration<P: ParticleTrait, F: ForceGenerator> {
    particle: P,
    force_generator: F,
}
