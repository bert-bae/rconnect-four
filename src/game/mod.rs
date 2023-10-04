mod player;
mod tile;
use crate::game::player::Player;
use crate::game::tile::{Tile, TileState};
use colored::{ColoredString, Colorize};
use std::fmt::{self, Write};
use std::io::{stdin, stdout};

#[derive(Debug)]
pub struct Game {
    pub players: (Option<Player>, Option<Player>),
    pub board: Option<Vec<Vec<Tile>>>,
    pub next: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: (None, None),
            board: None,
            next: 0,
        }
    }

    pub fn start(&mut self) {
        self.set_players();
        self.set_board();
        println!("{}", "Populating board...".to_string().green());
        self.draw().unwrap();

        let mut end = false;
        while !end {
            let (player_name, state) = self.switch();
            println!("It is {}'s turn", player_name);

            let mut column = String::new();
            stdin().read_line(&mut column).unwrap();
            self.select(column.trim().parse().unwrap(), state);
            self.draw().unwrap();
        }
    }

    fn set_board(&mut self) {
        let mut size = String::new();
        println!(
            "{}",
            "How large do you want your board to be?".to_string().blue()
        );
        stdin().read_line(&mut size).unwrap();

        match size.trim().parse::<usize>() {
            Ok(num) => {
                if num < 6 || num > 12 {
                    println!(
                        "{}",
                        "Please enter a number between 6 - 12".to_string().red()
                    );
                    self.set_board();
                }
                let mut grid: Vec<Vec<Tile>> = vec![];
                for _ in 0..num {
                    grid.push(vec![Tile::new(); num]);
                }
                self.board = Some(grid);
            }
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
        self.players.0 = Some(Player::new(p1.trim().to_string()));

        let mut p2 = String::new();
        println!("{}", "Enter name of player 2".to_string().blue());
        stdin().read_line(&mut p2).unwrap();
        self.players.1 = Some(Player::new(p2.trim().to_string()));
    }

    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let tile_width = 3;
        let board_size = self.board.as_ref().unwrap().len();
        let grid_width = board_size * (tile_width + 1);
        writeln!(&mut buffer, "-{:-^grid_width$}", "")?;
        for row in self.board.as_ref().unwrap() {
            for tile in row {
                let mut state: Option<ColoredString> = None;
                if tile.state == TileState::P1 {
                    state = Some(String::from("o").red());
                }

                if tile.state == TileState::P2 {
                    state = Some(String::from("o").green());
                }
                write!(
                    &mut buffer,
                    "|{:^tile_width$}",
                    if state.is_none() {
                        String::new().normal()
                    } else {
                        state.unwrap()
                    }
                )?;
            }
            write!(&mut buffer, "|\n")?;
            writeln!(&mut buffer, "-{:-^grid_width$}", "")?;
        }

        // Populate column identifiers
        for i in 0..board_size {
            write!(&mut buffer, " {: ^tile_width$}", i + 1)?;
        }
        println!("{buffer}");
        return Ok(());
    }

    fn switch(&mut self) -> (&str, TileState) {
        let next = if self.next == 0 { 1 } else { 0 };
        self.next = next;
        if next == 1 {
            return (&self.players.0.as_ref().unwrap().name, TileState::P1);
        } else {
            return (&self.players.1.as_ref().unwrap().name, TileState::P2);
        };
    }

    fn select(&mut self, column: usize, state: TileState) {
        let mutable_board = self.board.as_mut().unwrap();
        match mutable_board.get_mut(column - 1) {
            Some(row) => {
                let position = row.iter().position(|t| t.state == TileState::Empty);
                if position.is_none() {
                    println!(
                        "Invalid selection at column {column}. This column as no more empty slots."
                    );
                } else {
                    row[position.unwrap()].set_state(state);
                }
            }
            None => {
                println!("Invalid column {column}");
            }
        }
    }
}
