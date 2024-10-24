#[derive(Debug)]
pub struct Track {
    pub name         : String,
    pub stat_weights : TrackStatWeights,
}

impl Track {
    pub fn new(name: String, stat_weights: TrackStatWeights) -> Self {
        Track {
            name,
            stat_weights,
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Track {
            name: "Default".to_string(),
            stat_weights: TrackStatWeights::default(),
        }
    }
}
#[derive(Debug)]
pub struct TrackStatWeights {
    pub engine     : u16,
    pub gearbox    : u16,
    pub front_wing : u16,
    pub rear_wing  : u16,
    pub suspension : u16,
    pub brakes     : u16,
}

impl TrackStatWeights {
    pub fn new(
        engine     : u16,
        gearbox    : u16,
        front_wing : u16,
        rear_wing  : u16,
        suspension : u16,
        brakes     : u16,
    ) -> Self {
        TrackStatWeights {
            engine,
            gearbox,
            front_wing,
            rear_wing,
            suspension,
            brakes,
        }
    }
}

impl Default for TrackStatWeights {
    fn default() -> Self {
        TrackStatWeights {
            engine     : 0,
            gearbox    : 0,
            front_wing : 0,
            rear_wing  : 0,
            suspension : 0,
            brakes     : 0,
        }
    }
}