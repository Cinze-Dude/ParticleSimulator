#![allow(dead_code)]

use crate::{forces::calc_velocity, particle};
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

    pub fn step(&mut self) {
        calc_velocity(&mut self.particles);
    }

    pub fn add_particle(&mut self, particle: particle::Particle) {
        self.particles.push(particle);
    }
}
