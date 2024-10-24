use crate::structs::car::Car;

pub struct Driver {
    pub name: String,
    pub team_id: u32,
    pub driver_id: u32, 
    pub statblock: DriverStatBlock,
    pub car: Car,
}

impl Driver {
    pub fn new(name: String, team_id: u32, driver_id: u32, statblock: DriverStatBlock, car: Car) -> Self {
        Driver {
            name,
            team_id,
            driver_id,
            statblock,
            car,
        }
    }
}

pub struct DriverStatBlock {
    pub pace: u16,
    pub awareness: u16,
    pub racecraft: u16,
    pub experience: u16,
}

impl DriverStatBlock {
    pub fn new(p: u16, a: u16, r: u16, e: u16) -> Self {
        DriverStatBlock {
            pace: p,
            awareness: a,
            racecraft: r,
            experience: e,
        }
    }
}

impl Default for DriverStatBlock {
    fn default() -> Self {
        DriverStatBlock {
            pace: 0,
            awareness: 0,
            racecraft: 0,
            experience: 0,
        }
    }
}