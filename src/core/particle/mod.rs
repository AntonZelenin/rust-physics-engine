pub mod particle_trait;

use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;
use crate::core::vector::Vec3;

#[derive(Copy, Clone)]
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
        Self {
            position: Vec3::new(),
            velocity: Vec3::new(),
            acceleration: Vec3::new(),
            damping: 0.999,
            inverse_mass: 1.0,
        }
    }

    pub fn set_mass(&mut self, mass: Real) -> &mut Self {
        if mass <= 0.0 {
            panic!("Mass should be greater then 0");
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

    pub fn get_kinetic_energy(&self) -> Real {
        if self.is_infinite_mass() {
            return 0.0;
        }
        (1.0 / 2.0) * (1.0 / self.inverse_mass) * self.velocity.square_magnitude()
    }

    pub fn get_velocity_magnitude(&self) -> Real {
        self.velocity.magnitude()
    }

    pub fn add_acceleration(&mut self, acceleration: Vec3) -> &mut Self {
        self.acceleration += acceleration;
        self
    }

    pub fn set_damping(&mut self, damping: Real) -> &mut Self {
        self.damping = damping;
        self
    }
}

impl ParticleTrait for Particle {
    fn is_infinite_mass(&self) -> bool {
        self.inverse_mass == 0.0
    }

    fn get_position(&self) -> Vec3 {
        self.position.clone()
    }

    fn set_position(&mut self, p: Vec3) -> &mut Self {
        self.position = p;
        self
    }

    fn get_velocity(&self) -> Vec3 {
        self.velocity.clone()
    }

    fn set_velocity(&mut self, v: Vec3) -> &mut Self {
        self.velocity = v;
        self
    }

    fn get_acceleration(&self) -> Vec3 {
        self.acceleration.clone()
    }

    fn get_damping(&self) -> Real {
        self.damping
    }

    fn clear_accumulator(&mut self) -> &mut Self {
        self.acceleration.set_to_zero();
        self
    }
}
