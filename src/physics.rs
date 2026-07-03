use crate::math::Vec3;
use crate::particle;

pub fn calc_force(particle: &particle::Particle) -> crate::math::Vec3 {
    particle.state.motion.acceleration * particle.attr.mass
}
