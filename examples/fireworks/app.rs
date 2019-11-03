use kiss3d::window::Window;
use rust_physics_engine::core::timing::TimingData;
use std::borrow::BorrowMut;

pub trait App {
    fn run(&mut self) {
        let mut timing = TimingData::new();
        loop {
            if !self.get_window().render() {
                break;
            }
            timing.update();
            self.update(&timing);
        }
    }

    fn update(&mut self, timing: &TimingData);
    fn get_window(&mut self) -> &mut Window;
}
