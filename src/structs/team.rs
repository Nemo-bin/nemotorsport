#[derive(Debug)]
pub struct Team {
    pub name                     : String,
    pub team_id                  : u32,
    pub team_average_performance : u16,
}

impl Team {
    pub fn new(name: String, team_id: u32, team_average_performance: u16) -> Self {
        Team {
            name,
            team_id,
            team_average_performance,
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team {
            name                     : "Default".to_string(),
            team_id                  : 0,
            team_average_performance : 0,
        }
    }
}