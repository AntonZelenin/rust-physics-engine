use crate::core::types::Real;
use crate::core::vector::Vec3;

pub trait ParticleTrait {
    /// Integrates the particle forward in time by the given amount (in seconds?).
    /// This function uses a Newton-Euler integration method, which is a
    /// linear approximation to the correct integral. For this reason it
    /// mey be inaccurate in some cases
    fn integrate(&mut self, duration: Real) -> &mut Self {
        if duration < 0.0 {
            panic!("Time between frames cannot be less then 0");
        }
        if self.is_infinite_mass() || duration == 0.0 {
            return self;
        }

        let mut next_position = self.get_position();
        next_position.add_scaled(self.get_velocity(), duration);
        self.set_position(next_position);

//        println!("{:?}", self.get_position());

        let mut resulting_acceleration = self.get_acceleration();
        resulting_acceleration.add_scaled(self.get_force_accum().clone(), self.get_inverse_mass());

        let mut next_velocity = self.get_velocity();
        next_velocity.add_scaled(resulting_acceleration, duration);
        next_velocity *= self.get_damping().powf(duration);
        self.set_velocity(next_velocity);

        self.clear_accumulator();
        self
    }

    fn is_infinite_mass(&self) -> bool;
    fn get_inverse_mass(&self) -> Real;
    fn get_position(&self) -> Vec3;
    fn set_position(&mut self, p: Vec3) -> &mut Self;
    fn get_velocity(&self) -> Vec3;
    fn set_velocity(&mut self, v: Vec3) -> &mut Self;
    fn get_acceleration(&self) -> Vec3;
    fn get_damping(&self) -> Real;
    fn clear_accumulator(&mut self) -> &mut Self;
    fn add_force(&mut self, f: Vec3) -> &mut Self;
    // TODO should I return links from all getters?
    fn get_force_accum(&self) -> &Vec3;
}
