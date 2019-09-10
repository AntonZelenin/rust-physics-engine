use crate::core::vector::Vec3;
use crate::core::types::Real;

struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    /// Holds the amount of damping applied to linear motion.
    /// Damping is required to remove energy added through
    /// numerical instability in the integer
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
    pub fn set_mass(&mut self, mass: Real) -> &mut self {
        if mass <= 0.0 {
            panic!("Mass cannot be less or greater then 0");
        }
        self.inverse_mass = 1.0 / mass;
        self
    }

    pub fn set_inverse_mass(&mut self, inverse_mass: Real) -> &mut self {
        if mass <= 0.0 {
            panic!("Mass cannot be less or greater then 0");
        }
        self.inverse_mass = inverse_mass;
        self
    }
}
