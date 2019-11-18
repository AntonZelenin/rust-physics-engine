use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;
use crate::core::vector::Vec3;

pub struct AirBuoyancy {
    sea_level_height: Real,
    object_volume: Real,
    // kg per cubic meter
    sea_level_air_density: Real,
    // how fast density decreases with height
    density_loss_coefficient: Real,
}

impl AirBuoyancy {
    pub fn new(
        sea_level_height: Real,
        object_volume: Real,
        sea_level_air_density: Real,
        density_loss_coefficient: Real,
    ) -> Self {
        AirBuoyancy {
            sea_level_height,
            sea_level_air_density,
            object_volume,
            density_loss_coefficient,
        }
    }
}

impl ForceGenerator for AirBuoyancy {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, _duration: Real) {
        // TODO: what if a particle is lower then sea level?
        particle.add_force(Vec3::from_values(
            0.0,
            self.sea_level_air_density
                / (particle.get_position().y * self.density_loss_coefficient)
                * self.object_volume,
            0.0,
        ));
    }
}
