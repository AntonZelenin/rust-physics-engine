pub mod app;
pub mod fireworks_demo;

//use rand::prelude::*;
use crate::app::App;
use crate::fireworks_demo::FireworksDemo;

fn main() {
    let mut demo = FireworksDemo::new();
    demo.init_rules().init_firework().run();
    //    let mut particle = Particle::new();
    //
    //    let gravitation = Vec3::new().set_values(0.0, 9.8, 0.0).build();
    //
    //    let mut rng = rand::thread_rng();
    //    let mut timer = 0.0;
    //    let mut duration;
    //    let mut frames_number = 0;
    //    while timer < 1.0 {
    //        frames_number += 1;
    //        duration = 1.0 / rng.gen_range(60.0, 70.0);
    //        timer += duration;
    //        particle.add_acceleration(gravitation);
    //        particle.integrate(duration);
    //    }
    //    println!(
    //        "Particle velocity is: {}",
    //        particle.get_velocity_magnitude()
    //    );
    //    println!("Frames number: {}", frames_number);

    //    let mut window = Window::new("Kiss3d: cube");
    ////    let mut c = window.add_cube(1.0, 1.0, 1.0);
    ////    c.append_translation(&Translation3::from(Vector3::new(-1.0, 0.0, 5.0)));
    ////    c.set_color(1.0, 0.0, 0.0);
    //    let mut d = window.add_sphere(1.0);
    //    d.append_translation(&Translation3::from(Vector3::new(200.0, 0.0, 200.0)));
    //    d.set_color(1.0, 0.0, 0.0);
    //
    //    window.set_light(Light::StickToCamera);
    //
    ////    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    ////    let pos = Translation3::from(Vector3::new(0.0, 0.0, 0.0));
    //
    //    let gravitation = Vec3::new().set_values(0.0, 0.5, 0.0).build();
    //    let mut particle = Particle::new();
    //    particle.set_velocity(Vec3::new().set_values(25.0, 0.0, 0.0).build());
    //    let mut start = Instant::now();
    //
    //    let mut skip = true;
    //    while window.render() {
    //        if skip {
    //            std::thread::sleep(std::time::Duration::from_millis(3000));
    //            skip = false;
    //        }
    //        let elapsed = start.elapsed().as_nanos() as Real / 1000000000.0;
    //        particle.add_acceleration(gravitation);
    //        particle.integrate(elapsed);
    //        start = Instant::now();
    //        d.append_translation(&Translation3::from(Vector3::new(-particle.get_position().x, -particle.get_position().y, -particle.get_position().z)));
    ////        d.prepend_to_local_rotation(&rot));
    ////        c.append_translation(&pos);
}
