pub mod app;
pub mod fireworks_demo;

use crate::app::App;
use crate::fireworks_demo::FireworksDemo;
use kiss3d::window::Window;

fn main() {
    let mut demo = FireworksDemo::new();
    demo.init_rules().init_firework().run();
}
