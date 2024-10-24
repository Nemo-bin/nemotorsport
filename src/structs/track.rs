use rand::{Rng, seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Track {
    pub name         : String,
    pub stat_weights : TrackStatWeights,
    pub base_laptime : f64,
    pub laps         : u16,
}

impl Track {
    pub fn new(name: String, stat_weights: TrackStatWeights) -> Self {
        let base_laptime = generate_track_base_laptime();
        let laps = (3600 / base_laptime.round() as u32 ) as u16;
        Track {
            name,
            stat_weights,
            base_laptime,
            laps,
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Track {
            name         : "Default".to_string(),
            stat_weights : TrackStatWeights::default(),
            base_laptime : 0.0,
            laps         : 0,
        }
    }
}

fn generate_track_base_laptime() -> f64 {
    let mut rng = thread_rng();
    let base_laptime: f64 = rng.gen_range(60.0..=100.0);
    let rounded_base_laptime = (base_laptime * 1000.0).round() / 1000.0;
    return rounded_base_laptime;
}

#[derive(Debug)]
pub struct TrackStatWeights {
    pub engine     : f64,
    pub gearbox    : f64,
    pub front_wing : f64,
    pub rear_wing  : f64,
    pub suspension : f64,
    pub brakes     : f64,
}

impl TrackStatWeights {
    pub fn new(
        engine     : f64,
        gearbox    : f64,
        front_wing : f64,
        rear_wing  : f64,
        suspension : f64,
        brakes     : f64,
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
            engine     : 0.0,
            gearbox    : 0.0,
            front_wing : 0.0,
            rear_wing  : 0.0,
            suspension : 0.0,
            brakes     : 0.0,
        }
    }
}