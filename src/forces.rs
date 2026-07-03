use crate::particle;

pub fn calc_velocity(particles: &mut [particle::Particle], dt: f64) {
    for particle in particles {
        particle.state.transform.rotation += particle.state.motion.velocity * dt;
    }
}

pub fn calc_acceleration(particles: &mut [particle::Particle], dt: f64) {
    for particle in particles {
        particle.state.motion.velocity += particle.state.motion.acceleration * dt;
    }
}
