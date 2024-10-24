mod structs;

fn main() {
    println!("Hello, world!");

    let mut sim = structs::sim::Simulation::new();
    sim.populate();

    sim.run_race();
}
