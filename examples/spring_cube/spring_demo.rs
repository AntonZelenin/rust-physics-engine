use crate::app::App;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::{Point3, Translation3};
use rust_physics_engine::timing::TimingData;
use rust_physics_engine::types::Real;
use std::time::Duration;

pub struct SpringDemo {
    window: Window,
    point: Point3<f32>,
    cube: SceneNode,
}

impl SpringDemo {
    pub fn new() -> Self {
        let mut window = Window::new_with_size("Cyclone > Spring demo", 1024, 1024);
        window.set_light(Light::Absolute(Point3::new(0.0, 0.0, -10.0)));
        let mut cube = window.add_cube(1.0, 1.0, 1.0);
        cube.set_color(1.0, 0.0, 0.0);
        SpringDemo {
            window,
            point: Point3::new(0.0, 3.0, 0.0),
            cube,
        }
    }
}

impl App for SpringDemo {
    fn init(&mut self) {}

    fn update(&mut self, timing: &TimingData) {
        let duration = timing.get_last_frame_duration().as_secs_f64() as Real;
        // self.cube.set_local_translation(Translation3::new(timing.get_from_start().sin(), 0.0, 0.0));
    }

    fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn display(&mut self) {
        // let position = self.cube.data().world_transformation();
        // let p = &position * &Point3::new(0.0, 0.0, 0.0);
        // self.window.draw_line(
        //     &self.point,
        //     &p,
        //     &Point3::new(1.0, 0.0, 0.0)
        // )
    }
}
