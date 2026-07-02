mod math;
mod particle;
mod simulation;

fn main() {
    let sim = simulation::Simulation::new();
    println!("{:?}", sim.particles);
}
