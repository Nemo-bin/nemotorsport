#[derive(Debug)]
pub struct Team {
    pub name                     : String,
    pub team_id                  : u32,
    pub team_average_performance : u16,
    pub control                  : Control
}

impl Team {
    pub fn new(name: String, team_id: u32, team_average_performance: u16, control: Control) -> Self {
        Team {
            name,
            team_id,
            team_average_performance,
            control
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team {
            name                     : "Default".to_string(),
            team_id                  : 0,
            team_average_performance : 0,
            control                  : Control::Npc,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Control {
    Npc,
    Player,
}