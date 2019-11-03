use kiss3d::scene::SceneNode;
use nalgebra::{Translation3, Vector3};
use rust_physics_engine::core::particle::particle_trait::ParticleTrait;
use rust_physics_engine::core::particle::Particle;
use rust_physics_engine::core::types::Real;
use rust_physics_engine::core::vector::Vec3;
use std::borrow::BorrowMut;

#[derive(Clone)]
pub(crate) struct Firework {
    particle: Particle,
    pub(crate) firework_type: i32,
    pub(crate) age: Real,
    pub(crate) scene_node: Option<SceneNode>,
}

impl Firework {
    pub(crate) fn new() -> Self {
        Self {
            particle: Particle::new(),
            firework_type: 0,
            age: 0.0,
            scene_node: None,
        }
    }

    /// duration in seconds?
    pub(crate) fn update(&mut self, duration: Real) -> &mut Self {
        self.integrate(duration);
        let position = self.get_position();
        if let Some(node) = &mut self.scene_node {
            node.append_translation(&Translation3::from(Vector3::new(
                position.x, position.y, position.z,
            )));
        };
        self.age -= duration;
        self
    }

    pub(crate) fn is_alive(&self) -> bool {
        let p = self.particle.get_position();
        self.age > 0.0 && self.particle.get_position().y > -70.0
    }

    pub(crate) fn set_type(&mut self, firework_type: i32) -> &mut Self {
        self.firework_type = firework_type;
        self
    }

    pub(crate) fn set_mass(&mut self, mass: Real) -> &mut Self {
        self.particle.set_mass(mass);
        self
    }

    pub(crate) fn set_damping(&mut self, damping: Real) -> &mut Self {
        self.particle.set_damping(damping);
        self
    }

    pub(crate) fn add_acceleration(&mut self, acceleration: Vec3) -> &mut Self {
        self.particle.add_acceleration(acceleration);
        self
    }
}

impl ParticleTrait for Firework {
    fn is_infinite_mass(&self) -> bool {
        self.particle.is_infinite_mass()
    }

    fn get_position(&self) -> Vec3 {
        self.particle.get_position()
    }

    fn set_position(&mut self, p: Vec3) -> &mut Self {
        self.particle.set_position(p);
        self
    }

    fn get_velocity(&self) -> Vec3 {
        self.particle.get_velocity()
    }

    fn set_velocity(&mut self, v: Vec3) -> &mut Self {
        self.particle.set_velocity(v);
        self
    }

    fn get_acceleration(&self) -> Vec3 {
        self.particle.get_acceleration()
    }

    fn get_damping(&self) -> Real {
        self.particle.get_damping()
    }

    fn clear_accumulator(&mut self) -> &mut Self {
        self.particle.clear_accumulator();
        self
    }
}
