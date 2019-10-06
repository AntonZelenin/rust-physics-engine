use crate::core::particle::particle::Particle;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::random;
use crate::core::types::Real;
use crate::core::vector::Vec3;
use crate::core::GRAVITY;
use crate::demos::app::App;
use rand::Rng;

pub struct FireworksDemo {
    title: String,
    max_fireworks: u32,
    next_firework: u32,
    rule_count: u32,
    rules: Vec<FireworkRule>,
}

impl FireworksDemo {
    fn new() -> Self {
        let rule_count = 9;
        FireworksDemo {
            title: "Cyclone > Fireworks demo".to_string(),
            max_fireworks: 1024,
            next_firework: 0,
            rule_count,
            rules: Vec::with_capacity(rule_count as usize),
        }
    }

    fn init_rules(&mut self) {
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

    fn init_fireworks_graphics(&self) {
        self.init_graphics();

        // TODO implement override the clear color
//        glClearColor(0.0f, 0.0f, 0.1f, 1.0f);
    }

    fn update() {}

    fn display() {}
}

impl App for FireworksDemo {
    fn get_title(&self) -> String {
        self.title.clone()
    }
}

struct Firework {
    particle: Particle,
    firework_type: i32,
    age: Real,
}

struct FireworkRule {
    firework_type: i32,
    min_age: Real,
    max_age: Real,
    min_velocity: Vec3,
    max_velocity: Vec3,
    damping: Real,
    payload_count: u32,
    payloads: Vec<Payload>,
}

impl FireworkRule {
    fn new(
        firework_type: i32,
        min_age: Real,
        max_age: Real,
        min_velocity: Vec3,
        max_velocity: Vec3,
        damping: Real,
        payload_count: u32,
        payloads: Vec<Payload>,
    ) -> Self {
        Self {
            firework_type,
            min_age,
            max_age,
            min_velocity,
            max_velocity,
            damping,
            payload_count,
            payloads,
        }
    }
}

struct Payload {
    firework_type: i32,
    count: u32,
}

impl Firework {
    fn update(&mut self, duration: Real) -> &mut Self {
        self.integrate(duration);
        self.age -= duration;
        self
    }

    fn is_alive(&self) -> bool {
        self.age > 0.0 || self.particle.get_position().y > 0.0
    }

    fn set_type(&mut self, firework_type: i32) -> &mut Self {
        self.firework_type = firework_type;
        self
    }

    fn set_mass(&mut self, mass: Real) -> &mut Self {
        self.particle.set_mass(mass);
        self
    }

    fn set_damping(&mut self, damping: Real) -> &mut Self {
        self.particle.set_damping(damping);
        self
    }

    fn add_acceleration(&mut self, acceleration: Vec3) -> &mut Self {
        self.particle.add_acceleration(acceleration);
        self
    }
}

impl ParticleTrait for Firework {
    fn is_infinite_mass(&self) -> bool {
        self.particle.is_infinite_mass()
    }

    fn get_position(&self) -> Vec3 {
        self.particle.get_position()
    }

    fn set_position(&mut self, p: Vec3) -> &mut Self {
        self.particle.set_position(p);
        self
    }

    fn get_velocity(&self) -> Vec3 {
        self.particle.get_velocity()
    }

    fn set_velocity(&mut self, v: Vec3) -> &mut Self {
        self.particle.set_velocity(v);
        self
    }

    fn get_acceleration(&self) -> Vec3 {
        self.particle.get_acceleration()
    }

    fn get_damping(&self) -> Real {
        self.particle.get_damping()
    }

    fn clear_accumulator(&mut self) -> &mut Self {
        self.particle.clear_accumulator();
        self
    }
}

impl FireworkRule {
    fn create(&self, mut firework: Firework, parent: Option<Firework>) {
        firework.set_type(self.firework_type);
        firework.age = self.generate_age(self.min_age, self.max_age);

        let mut velocity = Vec3::new();
        match parent {
            Some(parent) => {
                firework.set_position(parent.get_position());
                velocity += parent.get_velocity();
            }
            None => {
                let mut start = Vec3::new();
                let mut rng = rand::thread_rng();
                // form -1 to 2 inclusively? high bound is exclusive
                let x = rng.gen_range(-1.0, 3.0);
                start.x = 5.0 * x;
                firework.set_position(start);
            }
        }
        velocity += random::random_vector(self.min_velocity, self.max_velocity);
        firework.set_velocity(velocity);
        firework.set_mass(1.0);
        firework.set_damping(self.damping);
        firework.add_acceleration(GRAVITY);
        firework.clear_accumulator();
    }

    fn generate_age(&self, min: Real, max: Real) -> Real {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
    }
}

impl Payload {
    fn new(firework_type: i32, count: u32) -> Self {
        Self {
            firework_type,
            count,
        }
    }
}
