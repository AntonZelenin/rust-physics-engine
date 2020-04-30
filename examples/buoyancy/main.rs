#[path = "../app.rs"]
pub mod app;
mod buoyancy;

use crate::app::App;
use crate::buoyancy::BuoyancyDemo;

fn main() {
    let mut demo = BuoyancyDemo::new();
    demo.run();
}
