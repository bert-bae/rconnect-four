#[derive(Debug)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
        }
    }
}
