use crate::particle;

pub fn calc_velocity(particles: &mut [particle::Particle]) {
    for particle in particles {
        particle.state.motion.velocity.x += particle.state.motion.velocity.x;
        particle.state.motion.velocity.y += particle.state.motion.velocity.y;
        particle.state.motion.velocity.z += particle.state.motion.velocity.z;
    }
}
