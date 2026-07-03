use crate::particle;

pub fn calc_velocity(particles: &mut [particle::Particle], dt: f64) {
    for particle in particles {
        particle.state.transform.position.x += particle.state.motion.velocity.x * dt;
        particle.state.transform.position.y += particle.state.motion.velocity.y * dt;
        particle.state.transform.position.z += particle.state.motion.velocity.z * dt;
    }
}

pub fn calc_acceleration(particles: &mut [particle::Particle], dt: f64) {
    for particle in particles {
        particle.state.motion.velocity.x += particle.state.motion.acceleration.x * dt;
        particle.state.motion.velocity.y += particle.state.motion.acceleration.y * dt;
        particle.state.motion.velocity.z += particle.state.motion.acceleration.z * dt;
    }
}
