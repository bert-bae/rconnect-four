#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name }
    }
}
