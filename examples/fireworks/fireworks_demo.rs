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
use kiss3d::window::Window;

#[derive(Clone)]
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

    pub fn init_rules(&mut self) -> &mut Self {
        let payloads = vec![Payload::new(3, 5), Payload::new(5, 5)];
        self.rules.push(FireworkRule::new(
            1,
            0.5,
            1.4,
            Vec3::from_values(-5.0, 25.0, -5.0),
            Vec3::from_values(5.0, 28.0, 5.0),
            0.1,
            2,
            payloads,
        ));

        let payloads = vec![Payload::new(4, 2)];
        self.rules.push(FireworkRule::new(
            2,
            0.5,
            1.0,
            Vec3::from_values(-5.0, 10.0, -5.0),
            Vec3::from_values(5.0, 20.0, 5.0),
            0.8,
            1,
            payloads,
        ));

        let payloads = vec![];
        self.rules.push(FireworkRule::new(
            3,
            0.5,
            1.5,
            Vec3::from_values(-5.0, -5.0, -5.0),
            Vec3::from_values(5.0, 5.0, 5.0),
            0.8,
            0,
            payloads,
        ));

        let payloads = vec![];
        self.rules.push(FireworkRule::new(
            4,
            0.25,
            0.5,
            Vec3::from_values(-20.0, 5.0, -5.0),
            Vec3::from_values(20.0, 5.0, 5.0),
            0.2,
            0,
            payloads,
        ));

        let payloads = vec![Payload::new(3, 5)];
        self.rules.push(FireworkRule::new(
            5,
            0.5,
            1.0,
            Vec3::from_values(-20.0, 2.0, -5.0),
            Vec3::from_values(20.0, 18.0, 5.0),
            0.01,
            1,
            payloads,
        ));

        let payloads = vec![];
        self.rules.push(FireworkRule::new(
            6,
            3.0,
            5.0,
            Vec3::from_values(-5.0, 50.0, -5.0),
            Vec3::from_values(5.0, 60.0, 5.0),
            0.95,
            0,
            payloads,
        ));

        let payloads = vec![Payload::new(8, 10)];
        self.rules.push(FireworkRule::new(
            7,
            4.0,
            5.0,
            Vec3::from_values(-5.0, 50.0, -5.0),
            Vec3::from_values(5.0, 60.0, 5.0),
            0.01,
            1,
            payloads,
        ));

        let payloads = vec![];
        self.rules.push(FireworkRule::new(
            8,
            0.25,
            0.5,
            Vec3::from_values(-1.0, -1.0, -1.0),
            Vec3::from_values(1.0, 1.0, 1.0),
            0.01,
            0,
            payloads,
        ));

        let payloads = vec![];
        self.rules.push(FireworkRule::new(
            9,
            3.0,
            5.0,
            Vec3::from_values(-15.0, 10.0, -5.0),
            Vec3::from_values(15.0, 15.0, 5.0),
            0.95,
            0,
            payloads,
        ));
        self
    }

    pub fn init_firework(&mut self) -> &mut Self {
        let firework = self.rules[0].create(None);
        self.fireworks.push(firework);
        self
    }

//    fn create_multiple(&mut self, firework_type: i32, number: u32, parent: &Firework) {
//        for _ in 0..number {
//            self.create(firework_type, parent);
//        }
//    }

    fn update_fireworks(&mut self, duration: Real) {
        self.fireworks
            .iter_mut()
            .for_each(|f| { f.update(duration); });
    }

    fn create_child_fireworks(&mut self) -> Vec<Firework> {
        let mut child_fireworks: Vec<Firework> = Vec::new();
        for firework in &mut self.fireworks {
            if !firework.is_alive() {
                let rule = &self.rules[(firework.firework_type - 1) as usize];
                for payload in rule.payloads.iter() {
                    child_fireworks.push(rule.create(Some(&firework)));
                }
            }
        }
        child_fireworks
    }

    fn remove_dead_fireworks(&mut self) {
        self.fireworks.retain(|x| x.is_alive());
    }

    fn get_rule_by_type(&self, firework_type: i32) -> &FireworkRule {
        &self.rules[(firework_type - 1) as usize]
    }
}

impl App for FireworksDemo {
    fn update(&mut self, timing: &TimingData) {
        let duration = timing.get_last_frame_duration().as_secs_f64() as Real;
        if duration <= 0.0 {
            return;
        }
        self.update_fireworks(duration);
        let mut child_fireworks = self.create_child_fireworks();
        self.fireworks.append(&mut child_fireworks);
        self.remove_dead_fireworks();
    }

    fn display(&self, window: &mut Window) {

    }

    fn get_title(&self) -> String {
        self.title.clone()
    }
}
