#[derive(Debug)]
pub struct Car {
    pub statblock: CarStatBlock,
}

impl Car {
    pub fn new(statblock: CarStatBlock) -> Self {
        Car { 
            statblock
        }
    }
}

impl Default for Car {
    fn default() -> Self {
        let statblock = CarStatBlock::default();
        Car {
            statblock
        }
    }
}

#[derive(Debug)]
pub struct CarStatBlock {
    pub engine     : Engine,
    pub gearbox    : Gearbox,
    pub front_wing : FrontWing,
    pub rear_wing  : RearWing,
    pub suspension : Suspension,
    pub brakes     : Brakes,
}

impl CarStatBlock {
    pub fn new(
        engine     : Engine,
        gearbox    : Gearbox,
        front_wing : FrontWing,
        rear_wing  : RearWing,
        suspension : Suspension,
        brakes: Brakes,
    ) -> Self {
        CarStatBlock {
            engine,
            gearbox,
            front_wing,
            rear_wing,
            suspension,
            brakes,
        }
    }
}

impl Default for CarStatBlock {
    fn default() -> Self {
        CarStatBlock {
            engine     : Engine::default(),
            gearbox    : Gearbox::default(),
            front_wing : FrontWing::default(),
            rear_wing  : RearWing::default(),
            suspension : Suspension::default(),
            brakes     : Brakes::default(),
        }
    }
}

#[derive(Debug)]
pub struct Engine {
    pub performance: u16,
    pub reliability: u16,
}

impl Engine {
    pub fn new(performance: u16, reliability: u16) -> Self {
        Engine {
            performance,
            reliability,
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            performance: 0,
            reliability: 0,
        }
    }
}

#[derive(Debug)]
pub struct Gearbox {
    pub performance: u16,
    pub reliability: u16,
}

impl Gearbox {
    pub fn new(performance: u16, reliability: u16) -> Self {
        Gearbox {
            performance,
            reliability,
        }
    }
}

impl Default for Gearbox {
    fn default() -> Self {
        Gearbox {
            performance: 0,
            reliability: 0,
        }
    }
}

#[derive(Debug)]
pub struct FrontWing {
    pub performance: u16,
    pub reliability: u16,
}

impl FrontWing {
    pub fn new(performance: u16, reliability: u16) -> Self {
        FrontWing {
            performance,
            reliability,
        }
    }
}

impl Default for FrontWing {
    fn default() -> Self {
        FrontWing {
            performance: 0,
            reliability: 0,
        }
    }
}

#[derive(Debug)]
pub struct RearWing {
    pub performance: u16,
    pub reliability: u16,
}

impl RearWing {
    pub fn new(performance: u16, reliability: u16) -> Self {
        RearWing {
            performance,
            reliability,
        }
    }
}

impl Default for RearWing {
    fn default() -> Self {
        RearWing {
            performance: 0,
            reliability: 0,
        }
    }
}

#[derive(Debug)]
pub struct Suspension {
    pub performance: u16,
    pub reliability: u16,
}

impl Suspension {
    pub fn new(performance: u16, reliability: u16) -> Self {
        Suspension {
            performance,
            reliability,
        }
    }
}

impl Default for Suspension {
    fn default() -> Self {
        Suspension {
            performance: 0,
            reliability: 0,
        }
    }
}

#[derive(Debug)]
pub struct Brakes {
    pub performance: u16,
    pub reliability: u16,
}

impl Brakes {
    pub fn new(performance: u16, reliability: u16) -> Self {
        Brakes {
            performance,
            reliability,
        }
    }
}

impl Default for Brakes {
    fn default() -> Self {
        Brakes {
            performance: 0,
            reliability: 0,
        }
    }
}