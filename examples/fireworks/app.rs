use kiss3d::window::Window;
use rust_physics_engine::core::timing::TimingData;
use std::time::{Duration, Instant};

pub trait App {
    fn run(&mut self) {
        let mut window = Window::new_with_size(&self.get_title(), 1024, 1024);
        let mut timing = TimingData {
            last_frame_duration: Duration::new(0, 0),
            last_frame_time: Instant::now(),
        };
        while window.render() {
            timing.update();
            self.update(&timing);
        }
    }

//    fn init_graphics(&self) {}

    fn update(&mut self, timing: &TimingData);

//    fn display() {}

//    fn key(&self, key: char) {}

    fn get_title(&self) -> String;
}
