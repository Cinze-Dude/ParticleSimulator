mod forces;
mod math;
mod particle;
mod physics;
mod simulation;

use std::io::Write;

fn main() {
    let dt = 1.0 / 60.0;

    let mut sim = simulation::Simulation::new();

    sim.add_particle(
        particle::Particle::builder()
            .position(math::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
            .velocity(math::Vec3 {
                x: 10.0,
                y: 12.25,
                z: 0.0,
            })
            .acceleration(math::Vec3 {
                x: -1.0,
                y: -9.8,
                z: 0.0,
            })
            .mass(1.0)
            .radius(1.0)
            .charge(1.0)
            .color(math::Color::new(0, 0, 255))
            .build(),
    );

    sim.add_particle(
        particle::Particle::builder()
            .position(math::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
            .velocity(math::Vec3 {
                x: 10.0,
                y: 18.325,
                z: 0.0,
            })
            .acceleration(math::Vec3 {
                x: -1.0,
                y: -9.8,
                z: 0.0,
            })
            .mass(1.0)
            .radius(1.0)
            .charge(1.0)
            .color(math::Color::new(0, 0, 255))
            .build(),
    );

    loop {
        sim.step(dt);

        let json = serde_json::to_string(&sim).unwrap();
        println!("{json}");
        std::io::stdout().flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
