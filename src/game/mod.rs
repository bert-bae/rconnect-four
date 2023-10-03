mod player;
mod tile;
use crate::game::tile::{Tile, TileState};
use crate::game::player::Player;
use colored::Colorize;
use std::io::{stdin, stdout};

#[derive(Debug)]
pub struct Game {
    players: (Option<Player>, Option<Player>),
    board: Option<Vec<Vec<Tile>>>
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: (None, None),
            board: None
        }
    }

    pub fn start(&mut self) {
        self.set_players();
        self.set_board();
    }
    
    fn set_board(&mut self) {
        let mut size = String::new();
        println!("{}", "How large do you want your board to be?".to_string().blue());
        stdin().read_line(&mut size).unwrap();
        
        match size.trim().parse::<u32>() {
            Ok(num) => println!("{num}"),
            Err(_) => {
                println!("{}", "Please enter a valid number".to_string().red());
                self.set_board();
            }
        }
    }

    fn set_players(&mut self) {
        let mut p1 = String::new();
        println!("{}", "Enter name of player 1".to_string().blue());
        stdin().read_line(&mut p1).unwrap();
        
        let mut p2 = String::new();
        println!("{}", "Enter name of player 2".to_string().blue());
        stdin().read_line(&mut p2).unwrap();

        self.players.0 = Some(Player::new(p1));
        self.players.1 = Some(Player::new(p2));
    }
}
