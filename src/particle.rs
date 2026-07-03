#![allow(dead_code)]

use crate::math;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: math::Vec3,
    pub rotation: math::Vec3,
}

#[derive(Debug, Clone)]
pub struct Motion {
    pub velocity: math::Vec3,
    pub ang_vel: math::Vec3,
}

#[derive(Debug, Clone)]
pub struct ParticleState {
    pub transform: Transform,
    pub motion: Motion,
}

#[derive(Debug, Clone)]
pub struct ParticleAttr {
    pub id: u32,
    pub mass: f64,
    pub radius: f64,
    pub charge: f64,
    pub color: math::Color,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub state: ParticleState,
    pub attr: ParticleAttr,
}
