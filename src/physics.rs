use crate::particle;

pub fn calc_force(particle: &particle::Particle) -> [f64; 3] {
    [
        particle.attr.mass * particle.state.motion.acceleration.x,
        particle.attr.mass * particle.state.motion.acceleration.y,
        particle.attr.mass * particle.state.motion.acceleration.z,
    ]
}
