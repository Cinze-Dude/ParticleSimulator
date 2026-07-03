use crate::particle;

pub fn calc_velocity(particles: &mut [particle::Particle]) {
    for particle in particles {
        particle.state.transform.position.x += particle.state.motion.velocity.x;
        particle.state.transform.position.y += particle.state.motion.velocity.y;
        particle.state.transform.position.z += particle.state.motion.velocity.z;
    }
}
