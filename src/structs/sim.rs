use crate::structs::{
    driver::{Driver, DriverStatBlock},
    car::{Car, CarStatBlock, Engine, Gearbox, FrontWing, RearWing, Suspension, Brakes},
    track::Track,
    team::Team,
};
use rand::{Rng, seq::SliceRandom, thread_rng};
use std::{fs, io};

pub struct Simulation {
    pub year     : u16,
    pub week     : u16,
    pub drivers  : [Driver; 20],
    pub teams    : [Team; 10],
    // pub calendar : [Track; 24],
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            year    : 0,
            week    : 0,
            drivers : [Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default(), Driver::default()],
            teams   : [Team::default(), Team::default(), Team::default(), Team::default(), Team::default(), Team::default(), Team::default(), Team::default(), Team::default(), Team::default()],
        }
    }

    pub fn populate(&mut self) {
        self.teams = self.populate_team_array();
        self.drivers = self.populate_driver_array();
    }

    fn populate_team_array(&self) -> [Team; 10] {
        [
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
            self.generate_new_team(),
        ]
    }

    fn generate_new_team(&self) -> Team {
        let mut rng = thread_rng();
        let name = "Default".to_string();
        let team_id = self.generate_new_team_id();
        let team_average_performance = rng.gen_range(5..=95);
        Team::new(name, team_id, team_average_performance)
    }

    fn populate_driver_array(&self) -> [Driver; 20] {
        let mut driver_array = [self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0), self.generate_new_driver(0)];
        for t in 0..self.teams.len() {
            let team_id = self.teams[t].team_id;
            driver_array[2*t] = self.generate_new_driver(team_id);
            driver_array[2*t + 1] = self.generate_new_driver(team_id);
        }

        return driver_array;
    }

    fn generate_new_driver(&self, team_id: u32) -> Driver {
        if team_id != 0 {
            let name = generate_new_driver_name();
            let driver_id = self.generate_new_driver_id();
            let team_average_performance = self.teams.iter()
                                                .find(|t| t.team_id == team_id)
                                                .unwrap()
                                                .team_average_performance;
    
            let statblock = generate_driver_statblock(team_average_performance);
            let car = Car::new(generate_car_statblock(team_average_performance));
            return Driver::new(name, team_id, driver_id, statblock, car);
        } else {
            return Driver::default();
        }
    }

    fn generate_new_driver_id(&self) -> u32 {
        let mut rng = rand::thread_rng();

        loop {
            let new_driver_id = rng.gen_range(1..1000);
            let mut id_is_unique = true; // Assume the ID is unique

            for d in &self.drivers { // Check it doesnt match any existing ids
                if d.driver_id == new_driver_id {
                    id_is_unique = false;
                    break; // If it does, then the id is not unique and break
                }
            }

            if id_is_unique { // After the above for loop, the id is either unique or not. 
                return new_driver_id;
            } // Loop back and restart if not unique.
        }
    }

    fn generate_new_team_id(&self) -> u32 {
        let mut rng = rand::thread_rng();

        loop {
            let new_team_id = rng.gen_range(1..1000);
            let mut id_is_unique = true; // Assume the ID is unique

            for t in &self.teams { // Check it doesnt match any existing ids
                if t.team_id == new_team_id {
                    id_is_unique = false;
                    break; // If it does, then the id is not unique and break
                }
            }

            if id_is_unique { // After the above for loop, the id is either unique or not. 
                return new_team_id;
            } // Loop back and restart if not unique.
        }
    }
}

fn generate_new_driver_name() -> String {
    let first_names = read_names_from_file("src\\assets\\names\\first_names.txt").expect("File not found");
    let surnames = read_names_from_file("src\\assets\\names\\surnames.txt").expect("File not found");

    let mut rng = thread_rng();

    let random_first_name = first_names.choose(&mut rng).unwrap();
    let random_surname = surnames.choose(&mut rng).unwrap();

    let full_name = format!("{} {}", random_first_name, random_surname);
    
    return full_name;
}

fn read_names_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(file_path)?;

    let names: Vec<String> = contents
        .split(',')
        .map(|name| name.trim().to_string())
        .collect();
        
    Ok(names)
}

fn generate_driver_statblock(team_average_performance: u16) -> DriverStatBlock { 
    let mut rng = thread_rng();
    let lower_bound = if team_average_performance >= 10 { team_average_performance - 10 } else { 0 };
    let upper_bound = if team_average_performance + 10 < 100 { team_average_performance + 10 } else { 100 };
    let p = rng.gen_range(lower_bound..upper_bound);
    let a = rng.gen_range(lower_bound..upper_bound);
    let r = rng.gen_range(lower_bound..upper_bound);
    let e = rng.gen_range(lower_bound..upper_bound);
    DriverStatBlock::new(p, a, r, e)
}

fn generate_car_statblock(team_average_performance: u16) -> CarStatBlock {
    let mut rng = thread_rng();
    let lower_bound = if team_average_performance >= 10 { team_average_performance - 10 } else { 0 };
    let upper_bound = if team_average_performance + 10 <= 100 { team_average_performance + 10 } else { 100 };

    let engine_performance = rng.gen_range(lower_bound..upper_bound);
    let gearbox_performance = rng.gen_range(lower_bound..upper_bound);
    let front_wing_performance = rng.gen_range(lower_bound..upper_bound);
    let rear_wing_performance = rng.gen_range(lower_bound..upper_bound);
    let suspension_performance = rng.gen_range(lower_bound..upper_bound);
    let brakes_performance = rng.gen_range(lower_bound..upper_bound);

    let engine_reliability = rng.gen_range(10..=100);
    let gearbox_reliability = rng.gen_range(10..=100);
    let front_wing_reliability = rng.gen_range(10..=100);
    let rear_wing_reliability = rng.gen_range(10..=100);
    let suspension_reliability = rng.gen_range(10..=100);
    let brakes_reliability = rng.gen_range(10..=100);

    let engine = Engine::new(engine_performance, engine_reliability);
    let gearbox = Gearbox::new(gearbox_performance, gearbox_reliability);
    let front_wing = FrontWing::new(front_wing_performance, front_wing_reliability);
    let rear_wing = RearWing::new(rear_wing_performance, rear_wing_reliability);
    let suspension = Suspension::new(suspension_performance, suspension_reliability);
    let brakes = Brakes::new(brakes_performance, brakes_reliability);

    CarStatBlock::new(engine, gearbox, front_wing, rear_wing, suspension, brakes)
}