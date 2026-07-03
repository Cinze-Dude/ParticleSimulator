#![allow(dead_code)]

use serde::Serialize;

use crate::particle;

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

    pub fn step() {}

    pub fn add_particle(&mut self, particle: particle::Particle) {
        self.particles.push(particle);
    }
}
