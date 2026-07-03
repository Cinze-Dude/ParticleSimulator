#![allow(dead_code)]

use crate::forces;
use crate::particle;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Simulation {
    pub particles: Vec<particle::Particle>,
    pub time: f64,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            time: 0.0,
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.time += dt;
        forces::calc_velocity(&mut self.particles, dt);
        forces::calc_acceleration(&mut self.particles, dt);
    }

    pub fn add_particle(&mut self, particle: particle::Particle) {
        self.particles.push(particle);
    }
}
