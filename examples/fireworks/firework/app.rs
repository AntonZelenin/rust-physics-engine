use kiss3d::window::Window;

pub trait App {
    fn run(&self) {
        let mut window = Window::new(&self.get_title());
        while window.render() {}
    }

    fn init_graphics(&self) {}

    fn update(&self) {}

    fn display() {}

    fn key(&self, key: char) {}

    fn get_title(&self) -> String;
}
