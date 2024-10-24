use crate::structs::car::Car;
use crate::structs::race::{Race, DrivingStyle, EngineMode, Tyre};
use crate::structs::track::Track;

use rand::{Rng, seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Driver {
    pub name      : String,
    pub team_id   : u32,
    pub driver_id : u32, 
    pub statblock : DriverStatBlock,
    pub car       : Car,
    pub race      : Race,
}

impl Driver {
    pub fn new(name: String, team_id: u32, driver_id: u32, statblock: DriverStatBlock, car: Car) -> Self {
        Driver {
            name,
            team_id,
            driver_id,
            statblock,
            car,
            race : Race::new(),
        }
    }

    pub fn run_lap(&mut self, track: &Track) {
        // We want to combine the driver stats, car stats, track weights, a random factor and race status to create a lap time.
        let eng = (self.car.statblock.engine.performance as f64 * track.stat_weights.engine) / 100.0;
        let gb = (self.car.statblock.gearbox.performance as f64 * track.stat_weights.gearbox) / 100.0; 
        let fw = (self.car.statblock.front_wing.performance as f64 * track.stat_weights.front_wing) / 100.0; 
        let rw = (self.car.statblock.rear_wing.performance as f64 * track.stat_weights.rear_wing) / 100.0; 
        let sus = (self.car.statblock.suspension.performance as f64 * track.stat_weights.suspension) / 100.0; 
        let br = (self.car.statblock.brakes.performance as f64 * track.stat_weights.brakes) / 100.0;
        let car_performance = 1.0 - (eng + gb + fw + rw + sus + br) / 6.0; // Float from 0.0 to 1.0

        let driver_performance = 1.0 - self.statblock.pace as f64 / 100.0; // Float from 0.0 to 1.0

        let driving_style_impact = self.race.get_driving_style_impact();
        let engine_mode_impact = self.race.get_engine_mode_impact();
        let tyre_compound_impact = self.race.get_tyre_compound_impact();
        let tyre_life_impact = self.race.get_tyre_life_impact();
        let fuel_weight_impact = self.race.get_fuel_weight_impact();

        let mut rng = thread_rng();
        let random_factor_impact: f64 = rng.gen();

        let total_mean_impact = (
            car_performance      +
            driver_performance   +
            driving_style_impact +
            engine_mode_impact   +
            tyre_compound_impact +
            tyre_life_impact     +
            fuel_weight_impact   +
            random_factor_impact
        ) / 8.0; // You can adjust the impact a specific factor has by multiplying it. 

        let laptime = track.base_laptime + (0.1 * track.base_laptime) * total_mean_impact;
        
        self.race.lap += 1;
        self.race.lap_times.push(laptime);
        self.race.total_race_time = self.race.sum_lap_times();

        self.race.fuel = if (self.race.fuel - 100.0 / track.laps as f64 > 0.0) {
            self.race.fuel - 100.0 / track.laps as f64
        } else { 0.0 };

        self.race.tyre_life = if (self.race.tyre_life - 100.0 / track.laps as f64 > 0.0) {
            self.race.tyre_life - 100.0 / track.laps as f64
        } else { 0.0 };

    }
}

impl Default for Driver {
    fn default() -> Driver {
        Driver {
            name      : "Default".to_string(),
            team_id   : 0,
            driver_id : 0,
            statblock : DriverStatBlock::default(),
            car       : Car::default(),
            race      : Race::default(),
        }
    }
}

#[derive(Debug)]
pub struct DriverStatBlock {
    pub pace       : u16,
    pub awareness  : u16,
    pub racecraft  : u16,
    pub experience : u16,
}

impl DriverStatBlock {
    pub fn new(p: u16, a: u16, r: u16, e: u16) -> Self {
        DriverStatBlock {
            pace       : p,
            awareness  : a,
            racecraft  : r,
            experience : e,
        }
    }
}

impl Default for DriverStatBlock {
    fn default() -> Self {
        DriverStatBlock {
            pace       : 0,
            awareness  : 0,
            racecraft  : 0,
            experience : 0,
        }
    }
}