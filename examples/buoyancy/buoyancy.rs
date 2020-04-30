use crate::app::App;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::{Point3, Translation2};
use rust_physics_engine::core::particle::force_generator::buoyancy::Buoyancy;
use rust_physics_engine::core::particle::force_generator::ForceGenerator;
use rust_physics_engine::core::particle::particle_trait::ParticleTrait;
use rust_physics_engine::core::particle::Particle;
use rust_physics_engine::core::timing::TimingData;
use rust_physics_engine::core::types::Real;
use rust_physics_engine::core::vector::Vec3;
use std::time::Duration;
use rust_physics_engine::core::particle::force_generator::gravity::Gravity;
use rust_physics_engine::core::GRAVITY;

pub struct BuoyancyDemo {
    window: Window,
    particle: Particle,
    buoyancy_fg: Buoyancy,
    gravity_fg: Gravity,
}

impl BuoyancyDemo {
    pub fn new() -> Self {
        let mut window = Window::new_with_size("Cyclone > Spring demo", 1024, 1024);
        window.set_light(Light::Absolute(Point3::new(0.0, 0.0, -10.0)));
        let mut particle = Particle::new();
        particle.set_position(Vec3::from_values(0.0, 5.0, 15.0)).set_damping(0.5);
        BuoyancyDemo {
            window,
            particle,
            buoyancy_fg: Buoyancy::new(1.0, 1.0, 0.0, 20.0),
            gravity_fg: Gravity::new(GRAVITY),
        }
    }
}

impl App for BuoyancyDemo {
    fn init(&mut self) {
        self.window.set_point_size(10.0);
    }

    fn update(&mut self, timing: &TimingData) {
        let duration = timing.get_last_frame_duration().as_secs_f64() as Real;
        self.buoyancy_fg.update_force(&mut self.particle, duration);
        self.gravity_fg.update_force(&mut self.particle, duration);
        self.particle.integrate(duration);
    }

    fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn display(&mut self) {
        let Vec3 { x, y, z, .. } = self.particle.get_position();
        self.window.draw_point(
            &Point3::new(x as f32, y as f32, z as f32),
            &Point3::new(1.0, 0.0, 0.0),
        )
    }
}
