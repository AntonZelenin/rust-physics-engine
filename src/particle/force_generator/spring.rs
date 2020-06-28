// use crate::particle::force_generator::ForceGenerator;
// use crate::particle::particle_trait::ParticleTrait;
// use crate::rigid_body::RigidBody;
// use crate::types::Real;
// use crate::vector::Vec3;
//
// struct Spring<'a> {
//     connection_point: Vec3,
//     other_connection_point: Vec3,
//     other: &'a RigidBody,
//     spring_constant: Real,
//     rest_length: Real,
// }
//
// impl<'a> Spring<'a> {
//     pub fn new(
//         connection_point: Vec3,
//         other: &'a RigidBody,
//         other_connection_point: Vec3,
//         spring_constant: Real,
//         rest_length: Real,
//     ) -> Self {
//         Spring {
//             connection_point,
//             other,
//             other_connection_point,
//             spring_constant,
//             rest_length,
//         }
//     }
// }
//
// impl ForceGenerator for Spring<'_> {
//     fn update_force(&mut self, body: &mut RigidBody, _duration: Real) {
//         let lws = body.get_point_in_world_space(&self.connection_point);
//         let ows = self
//             .other
//             .get_point_in_world_space(&self.other_connection_point);
//
//         let mut force = lws - ows;
//         let magnitude = self.spring_constant * (force.magnitude() - self.rest_length).abs();
//         force.normalize();
//         body.add_force(&(&force * -magnitude));
//     }
// }
