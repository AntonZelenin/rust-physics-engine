use kiss3d::window::Window;
use rust_physics_engine::core::timing::TimingData;

pub trait App {
    fn run(&mut self) {
        let mut window = Window::new_with_size(&self.get_title(), 1024, 1024);
        let mut timing = TimingData::new();
        while window.render() {
            timing.update();
            self.update(&timing);
            self.display(&mut window);
        }
    }

    fn update(&mut self, timing: &TimingData);
    fn display(&self, window: &mut Window);
    fn get_title(&self) -> String;
}
