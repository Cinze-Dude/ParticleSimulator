use crate::particle;

pub fn intg_velocity(particles: &mut [particle::Particle], dt: f64) {
    for particle in particles {
        particle.state.transform.position += particle.state.motion.velocity * dt;
    }
}

pub fn intg_acceleration(particles: &mut [particle::Particle], dt: f64) {
    for particle in particles {
        particle.state.motion.velocity += particle.state.motion.acceleration * dt;
    }
}
