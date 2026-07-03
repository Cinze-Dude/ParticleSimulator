mod math;
mod particle;
mod simulation;

fn main() {
    let sim = simulation::Simulation {
        particles: vec![particle::Particle {
            state: particle::ParticleState {
                transform: particle::Transform {
                    position: math::Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    rotation: math::Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                motion: particle::Motion {
                    velocity: math::Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    ang_vel: math::Vec3 {
                        x: 0.1,
                        y: 0.1,
                        z: 0.1,
                    },
                },
            },
            attr: particle::ParticleAttr {
                id: 1,
                mass: 1.0,
                radius: 1.0,
                charge: 1.0,
                color: math::Color {
                    r: 000,
                    g: 000,
                    b: 255,
                },
            },
        }],
    };
    let json = serde_json::to_string(&sim).unwrap();
    println!("{json}");
}
