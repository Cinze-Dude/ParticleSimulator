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

pub fn intg_gravity(particles: &mut [particle::Particle], dt: f64) {
    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            const G: f64 = 6.67430e-11;
            let r = particles[i].state.transform.position - particles[j].state.transform.position;
            let distsq = r.length();
            let dirc = r.normalize();
            particles[i].state.motion.acceleration +=
                dirc * G * particles[j].attr.mass / distsq * dt;
            particles[j].state.motion.acceleration -=
                dirc * G * particles[i].attr.mass / distsq * dt;
        }
    }
}
