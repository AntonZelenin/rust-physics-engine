#[path = "../app.rs"]
pub mod app;
mod spring_demo;

use crate::app::App;
use crate::spring_demo::SpringDemo;

fn main() {
    let mut demo = SpringDemo::new();
    demo.run();
}
