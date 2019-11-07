use kiss3d::window::Window;
use rust_physics_engine::core::timing::TimingData;

pub trait App {
    fn run(&mut self) {
        self.init();
        let mut timing = TimingData::new();
        loop {
            if !self.get_window().render() {
                break;
            }
            self.display();
            timing.update();
            self.update(&timing);
        }
    }

    fn init(&mut self);
    fn update(&mut self, timing: &TimingData);
    fn get_window(&mut self) -> &mut Window;
    fn display(&mut self);
}
