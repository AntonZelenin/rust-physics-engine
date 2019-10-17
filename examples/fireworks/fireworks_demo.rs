mod firework;
mod firework_rule;
mod payload;

use crate::app::App;
use crate::fireworks_demo::firework::Firework;
use crate::fireworks_demo::payload::Payload;
use crate::fireworks_demo::firework_rule::FireworkRule;
use rust_physics_engine::core::vector::Vec3;
use rust_physics_engine::core::timing::TimingData;
use rust_physics_engine::core::types::Real;

pub struct FireworksDemo {
    title: String,
    max_fireworks: u32,
    fireworks: Vec<Firework>,
    next_firework: u32,
    rule_count: u32,
    rules: Vec<FireworkRule>,
}

impl FireworksDemo {
    pub fn new() -> Self {
        let rule_count = 9;
        let max_fireworks = 1024;
        FireworksDemo {
            title: "Cyclone > Fireworks demo".to_string(),
            max_fireworks,
            fireworks: Vec::with_capacity(max_fireworks as usize),
            next_firework: 0,
            rule_count,
            rules: Vec::with_capacity(rule_count as usize),
        }
    }

    pub fn init_rules(&mut self) {
        let payloads = vec![Payload::new(3, 5), Payload::new(5, 5)];
        self.rules[0] = FireworkRule::new(
            1,
            0.5,
            1.4,
            Vec3::from_values(-5.0, 25.0, -5.0),
            Vec3::from_values(5.0, 28.0, 5.0),
            0.1,
            2,
            payloads,
        );

        let payloads = vec![Payload::new(4, 2)];
        self.rules[1] = FireworkRule::new(
            2,
            0.5,
            1.0,
            Vec3::from_values(-5.0, 10.0, -5.0),
            Vec3::from_values(5.0, 20.0, 5.0),
            0.8,
            1,
            payloads,
        );

        let payloads = vec![];
        self.rules[2] = FireworkRule::new(
            3,
            0.5,
            1.5,
            Vec3::from_values(-5.0, -5.0, -5.0),
            Vec3::from_values(5.0, 5.0, 5.0),
            0.8,
            0,
            payloads,
        );

        let payloads = vec![];
        self.rules[3] = FireworkRule::new(
            4,
            0.25,
            0.5,
            Vec3::from_values(-20.0, 5.0, -5.0),
            Vec3::from_values(20.0, 5.0, 5.0),
            0.2,
            0,
            payloads,
        );

        let payloads = vec![Payload::new(3, 5)];
        self.rules[4] = FireworkRule::new(
            5,
            0.5,
            1.0,
            Vec3::from_values(-20.0, 2.0, -5.0),
            Vec3::from_values(20.0, 18.0, 5.0),
            0.01,
            1,
            payloads,
        );

        let payloads = vec![];
        self.rules[5] = FireworkRule::new(
            6,
            3.0,
            5.0,
            Vec3::from_values(-5.0, 50.0, -5.0),
            Vec3::from_values(5.0, 60.0, 5.0),
            0.95,
            0,
            payloads,
        );

        let payloads = vec![Payload::new(8, 10)];
        self.rules[6] = FireworkRule::new(
            7,
            4.0,
            5.0,
            Vec3::from_values(-5.0, 50.0, -5.0),
            Vec3::from_values(5.0, 60.0, 5.0),
            0.01,
            1,
            payloads,
        );

        let payloads = vec![];
        self.rules[7] = FireworkRule::new(
            8,
            0.25,
            0.5,
            Vec3::from_values(-1.0, -1.0, -1.0),
            Vec3::from_values(1.0, 1.0, 1.0),
            0.01,
            0,
            payloads,
        );

        let payloads = vec![];
        self.rules[8] = FireworkRule::new(
            9,
            3.0,
            5.0,
            Vec3::from_values(-15.0, 10.0, -5.0),
            Vec3::from_values(15.0, 15.0, 5.0),
            0.95,
            0,
            payloads,
        );
    }

//    fn init_fireworks_graphics(&self) {
//        self.init_graphics();
//
//         TODO implement override the clear color
//        glClearColor(0.0f, 0.0f, 0.1f, 1.0f);
//    }

    fn create(&mut self, firework_type: i32, parent: &Firework) {
        let rule: &FireworkRule = &self.rules[(firework_type - 1) as usize];
        rule.create(&mut self.fireworks[self.next_firework as usize], Some(&parent));
        self.next_firework = (self.next_firework + 1) % self.max_fireworks;
    }

    fn create_multiple(&mut self, firework_type: i32, number: u32, parent: &Firework) {
        for _ in 0..number {
            self.create(firework_type, parent);
        }
    }

    fn display() {}

    fn update_fireworks(&mut self, duration: Real) {
        self.fireworks.iter_mut()
            .filter(|f| f.firework_type != 0)
            .map(|f| f.update(duration));
    }

    fn process_dead_fireworks(&mut self) {
        self.fireworks
            .iter_mut()
            .filter(|f| !f.is_alive())
            .map(|f| {
                let rule = self.rules[(f.firework_type - 1) as usize].clone();

                // Delete the current firework (this doesn't affect its
                // position and velocity for passing to the create function,
                // just whether or not it is processed for rendering or
                // physics.
                f.set_type(0);

                for p in rule.payloads.iter() {
                    self.create(p.firework_type, &f);
                }
            });
    }
}

impl App for FireworksDemo {
    fn update(&mut self, timing: &TimingData) {
        let duration = timing.get_last_frame_duration().as_secs_f64() as Real;
        if duration <= 0.0 {
            return;
        }
        self.update_fireworks(duration);
        self.process_dead_fireworks();

//        for firework in self.fireworks.iter_mut() {
//            if firework.firework_type <= 0 {
//                continue;
//            }
//
//            firework.update(duration);
//            if !firework.is_alive() {
//                let rule = &self.rules[(firework.firework_type - 1) as usize];
//
//                 Delete the current firework (this doesn't affect its
//                 position and velocity for passing to the create function,
//                 just whether or not it is processed for rendering or
//                 physics.
//                firework.firework_type = 0;
//
//                for p in rule.payloads.iter() {
//                    self.create(p.firework_type, &firework);
//                }
//            }
//        }
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }
}
