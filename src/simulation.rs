#![allow(dead_code)]

use crate::forces;
use crate::particle;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Simulation {
    pub particles: Vec<particle::Particle>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn step(&mut self, dt: f64) {
        forces::calc_velocity(&mut self.particles, dt);
        forces::calc_acceleration(&mut self.particles, dt);
    }

    pub fn add_particle(&mut self, particle: particle::Particle) {
        self.particles.push(particle);
    }
}
