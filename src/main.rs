mod forces;
mod math;
mod particle;
mod physics;
mod simulation;

use std::io::Write;

fn main() {
    let dt = 1.0 / 60.0;

    let mut sim = simulation::Simulation::new();
    sim.add_particle(particle::Particle {
        state: particle::ParticleState {
            transform: particle::Transform {
                position: math::Vec3 {
                    x: 0.0,
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
                    x: 10.0,
                    y: 12.25,
                    z: 0.0,
                },
                ang_vel: math::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                acceleration: math::Vec3 {
                    x: -1.0,
                    y: -9.8,
                    z: 0.0,
                },
                ang_accel: math::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        },
        attr: particle::ParticleAttr {
            id: 1,
            mass: 10000.0,
            radius: 1.0,
            charge: 1.0,
            color: math::Color {
                r: 000,
                g: 000,
                b: 255,
            },
        },
    });
    sim.add_particle(particle::Particle {
        state: particle::ParticleState {
            transform: particle::Transform {
                position: math::Vec3 {
                    x: 0.0,
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
                    x: 10.0,
                    y: 18.325,
                    z: 0.0,
                },
                ang_vel: math::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                acceleration: math::Vec3 {
                    x: -1.0,
                    y: -9.8,
                    z: 0.0,
                },
                ang_accel: math::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        },
        attr: particle::ParticleAttr {
            id: 1,
            mass: 10000.0,
            radius: 1.0,
            charge: 1.0,
            color: math::Color {
                r: 000,
                g: 255,
                b: 0,
            },
        },
    });
    loop {
        sim.step(dt);

        let json = serde_json::to_string(&sim).unwrap();
        println!("{json}");
        std::io::stdout().flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
