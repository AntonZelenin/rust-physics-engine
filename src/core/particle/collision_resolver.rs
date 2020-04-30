use crate::core::particle::particle_trait::ParticleTrait;
use crate::core::types::Real;
use crate::core::vector::Vec3;

struct CollisionResolver<P: ParticleTrait> {
    // particles involved in the contact. The second of these could be null for contacts with the scenery
    particles: [P; 2],
    // normal restitution coefficient at the contact
    restitution: Real,
    // the direction of the contact in world coordinates
    contact_normal: Vec3,
}
