mod structs;

fn main() {
    println!("Hello, world!");

    let mut sim = structs::sim::Simulation::new();
    sim.populate();

    println!("{:#?}", sim.teams.iter().find(|t| t.control == structs::team::Control::Player).unwrap());
}
