use std::fmt;

#[derive(Debug, Clone)]
pub struct Player {
    /// cuid
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub host: bool,
    pub score: i32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}