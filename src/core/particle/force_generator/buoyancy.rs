use crate::core::particle::force_generator::ForceGenerator;
use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;
use crate::core::vector::Vec3;

pub struct Buoyancy {
    // the maximum submersion depth of the object before it generates its maximum buoyancy force
    max_depth: Real,
    object_volume: Real,
    // the height of the water plane above y = 0. The plane will be parallel to the XZ plane
    water_height: Real,
    // kg per cubic meter
    liquid_density: Real,
}

impl Buoyancy {
    pub fn new(
        max_depth: Real,
        object_volume: Real,
        water_height: Real,
        liquid_density: Real,
    ) -> Self {
        Buoyancy {
            max_depth,
            object_volume,
            water_height,
            liquid_density,
        }
    }
}

impl ForceGenerator for Buoyancy {
    fn update_force<P: ParticleTrait>(&mut self, particle: &mut P, _duration: Real) {
        let mut submersion_depth = particle.get_position().y;
        if submersion_depth >= self.water_height + self.max_depth {
            return;
        }
        if submersion_depth <= self.water_height - self.max_depth {
            submersion_depth = self.max_depth;
        }
        // "/ 2.0 * self.max_depth" without brackets?
        particle.add_force(Vec3::from_values(
            0.0,
            self.liquid_density
                * self.object_volume
                * (submersion_depth - self.max_depth - self.water_height)
                / (2.0 * self.max_depth),
            0.0,
        ));
    }
}
