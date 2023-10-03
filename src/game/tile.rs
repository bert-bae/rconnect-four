#[derive(Debug)]
pub enum TileState {
    Empty,
    P1,
    P2
}

#[derive(Debug)]
pub struct Tile {
   pub state: TileState
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            state: TileState::Empty
        }
    }

    pub fn set_state(&mut self, state: TileState) {
        self.state = state
    }
}
