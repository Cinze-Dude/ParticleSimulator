#![allow(dead_code)]

use serde::Serialize;

use crate::math;

#[derive(Debug, Clone, Serialize)]
pub enum Force {
    Gravity(math::Vec3),
    Initial(math::Vec3),
}

#[derive(Debug, Clone, Serialize)]
pub struct Transform {
    pub position: math::Vec3,
    pub rotation: math::Vec3,
}

#[derive(Debug, Clone, Serialize)]
pub struct Motion {
    pub velocity: math::Vec3,
    pub ang_vel: math::Vec3,
    pub acceleration: math::Vec3,
    pub ang_accel: math::Vec3,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParticleState {
    pub transform: Transform,
    pub motion: Motion,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParticleAttr {
    pub id: u32,
    pub mass: f64,
    pub radius: f64,
    pub charge: f64,
    pub color: math::Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct Particle {
    pub state: ParticleState,
    pub attr: ParticleAttr,
    pub forces: Vec<Force>,
}

pub struct ParticleBuilder {
    position: math::Vec3,
    rotation: math::Vec3,

    velocity: math::Vec3,
    ang_velocity: math::Vec3,

    acceleration: math::Vec3,
    ang_acceleration: math::Vec3,

    mass: f64,
    radius: f64,
    charge: f64,

    color: math::Color,
}

impl Particle {
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder {
            position: math::Vec3::ZERO(),
            rotation: math::Vec3::ZERO(),

            velocity: math::Vec3::ZERO(),
            ang_velocity: math::Vec3::ZERO(),

            acceleration: math::Vec3::ZERO(),
            ang_acceleration: math::Vec3::ZERO(),

            mass: 1.0,
            radius: 1.0,
            charge: 0.0,

            color: math::Color {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}

impl ParticleBuilder {
    pub fn position(mut self, position: math::Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn rotation(mut self, rotation: math::Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn velocity(mut self, velocity: math::Vec3) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn ang_velocity(mut self, ang_velocity: math::Vec3) -> Self {
        self.ang_velocity = ang_velocity;
        self
    }

    pub fn acceleration(mut self, acceleration: math::Vec3) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn ang_acceleration(mut self, ang_acceleration: math::Vec3) -> Self {
        self.ang_acceleration = ang_acceleration;
        self
    }

    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn charge(mut self, charge: f64) -> Self {
        self.charge = charge;
        self
    }

    pub fn color(mut self, color: math::Color) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> Particle {
        Particle {
            state: ParticleState {
                transform: Transform {
                    position: self.position,
                    rotation: self.rotation,
                },
                motion: Motion {
                    velocity: self.velocity,
                    ang_vel: self.ang_velocity,
                    acceleration: self.acceleration,
                    ang_accel: self.ang_acceleration,
                },
            },
            attr: ParticleAttr {
                id: 0,
                mass: self.mass,
                radius: self.radius,
                charge: self.charge,
                color: self.color,
            },
            forces: Vec::new(),
        }
    }
}
