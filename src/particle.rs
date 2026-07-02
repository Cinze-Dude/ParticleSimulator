use crate::math;

#[derive(Debug)]
pub struct Transform {
    pub position: math::Vec3,
    pub rotation: math::Vec3,
}

#[derive(Debug)]
pub struct Motion {
    pub velocity: math::Vec3,
    pub ang_vel: math::Vec3,
}

#[derive(Debug)]
pub struct ParticleState {
    pub transform: Transform,
    pub motion: Motion,
}

#[derive(Debug)]
pub struct ParticleAttr {
    pub id: u32,
    pub mass: f64,
    pub radius: f64,
    pub charge: f64,
    pub color: math::Color,
}
#[derive(Debug)]
pub struct Particle {
    pub state: ParticleState,
    pub attr: ParticleAttr,
}
