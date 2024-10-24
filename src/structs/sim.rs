use crate::structs::{
    driver::{Driver, DriverStatBlock},
    car::{Car, CarStatBlock},
    track::Track,
    team::Team,
};
use rand::{Rng, seq::SliceRandom, thread_rng};
use std::{fs, io};

pub struct Simulation {
    year     : u16,
    week     : u16,
    drivers  : [Driver; 20],
    teams    : [Team; 10],
    calendar : [Track; 24],
}

impl Simulation {    
    fn generate_new_driver(&self, team_id: u32) -> Driver {
        let name = generate_new_driver_name();
        let driver_id = self.generate_new_driver_id();
        let statblock = DriverStatBlock::default();
        let car = Car::default();

        Driver::new(name, team_id, driver_id, statblock, car)
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