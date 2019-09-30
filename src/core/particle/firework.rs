use crate::core::particle::particle::Particle;
use crate::core::types::Real;

enum Type {

}

struct Firework {
    particle: Particle,
    firework_type: Type,
    age: Real,
}

struct FireworkRule {
    firework_type: Type,
    min_age: Real,
    max_age: Real,
    min_velocity: Real,
    max_velocity: Real,
    damping: Real,
    payload_count: u32,
    payloads: Vec<Payload>,
}

struct Payload {
    firework_type: Type,
    count: u32,
}

impl Payload {
    fn new(firework_type: Type, count: u32) -> Self {
        Self {
            firework_type,
            count,
        }
    }
}
