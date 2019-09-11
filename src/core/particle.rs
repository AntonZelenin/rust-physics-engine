use crate::core::vector::Vec3;
use crate::core::types::Real;

pub struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    /// Holds the amount of damping applied to linear motion.
    /// Damping is required to remove energy added through
    /// numerical instability in the integer
    /// The values close to 1 are good ones
    damping: Real,
    /// It's more useful to hold the inverse mass because integration is simpler, and
    /// because in real-time simulation it is more useful to have objects with
    /// infinite mass (immovable) than zero mass (completely unstable in numerical simulation)
    /// The formula is a = (1 / m) * f, where a is acceleration, f - force applied
    /// to the particle. So particles with 0 mass are impossible (we cannot represent
    /// infinity) and we can represent objects with infinite mass saying inverse mass = 0
    /// this can be used for immovable objects like walls or floor
    inverse_mass: Real,
}

impl Particle {
    pub fn new() -> Self {
        Particle {
            position: Vec3::new(),
            velocity: Vec3::new(),
            acceleration: Vec3::new(),
            damping: 0.999,
            inverse_mass: 1.0,
        }
    }

    pub fn set_mass(&mut self, mass: Real) -> &mut Self {
        if mass <= 0.0 {
            panic!("Mass cannot be less or greater then 0");
        }
        self.inverse_mass = 1.0 / mass;
        self
    }

    pub fn set_inverse_mass(&mut self, inverse_mass: Real) -> &mut Self {
        if inverse_mass < 0.0 {
            panic!("Mass cannot be less or greater then 0");
        }
        self.inverse_mass = inverse_mass;
        self
    }

    /// Integrates the particle forward in time by the given amount.
    /// This function uses a Newton-Euler integration method, which is a
    /// linear approximation to the correct integral. For this reason it
    /// mey be inaccurate in some cases
    pub fn integrate(&mut self, duration: Real) -> &mut Self {
        if duration <= 0.0 {
            panic!("Time between frames cannot be <= 0");
        }
        if self.is_infinite_mass() {
            return self;
        }
        self.position.add_scaled(&self.velocity, duration);
        self.velocity.add_scaled(&self.acceleration, duration);
        self.velocity *= self.damping.powf(duration);
        self.clear_accumulator();
        self
    }

    pub fn get_kinetic_energy(&self) -> Real {
        if self.is_infinite_mass() {
            return 0.0;
        }
        (1.0 / 2.0) * (1.0 / self.inverse_mass) * self.velocity.square_magnitude()
    }

    fn is_infinite_mass(&self) -> bool {
        self.inverse_mass == 0.0
    }

    pub fn get_velocity_magnitude(&self) -> Real {
        self.velocity.magnitude()
    }

    pub fn add_acceleration(&mut self, acceleration: Vec3) -> &mut Self {
        self.acceleration += acceleration;
        self
    }

    pub fn clear_accumulator(&mut self) -> &mut Self {
        self.acceleration.set_to_zero();
        self
    }
}
