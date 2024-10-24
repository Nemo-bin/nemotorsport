pub struct Team {
    pub name: String,
    pub team_id: u32,
}

impl Team {
    pub fn new(name: String, team_id: u32) -> Self {
        Team {
            name,
            team_id,
        }
    }
}