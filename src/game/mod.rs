mod player;
mod tile;
use crate::game::player::Player;
use crate::game::tile::{Tile, TileState};
use colored::{ColoredString, Colorize};
use std::fmt::Write;
use std::io::stdin;
use thiserror::Error;

#[derive(Debug, Error)]
enum GameError {
    #[error("Invalid move. Select a column between 1 - {0}.")]
    OutOfBound(usize),
    #[error("Invalid move. The column is full. Please select an empty column.")]
    MoveLimited(),
    #[error("Invalid board size. Please select a number between 6 - 12")]
    BoardSizeLimit(),
}

#[derive(Debug)]
pub struct Game {
    pub players: (Option<Player>, Option<Player>),
    pub board: Option<Vec<Vec<Tile>>>,
    pub board_size: Option<usize>,
    pub next: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: (None, None),
            board: None,
            board_size: None,
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
            let (player_name, state) = self.get_current_player();
            println!("It is {}'s turn", player_name);

            let mut column = String::new();
            stdin().read_line(&mut column).unwrap();

            // Convert user input to indexed column
            let parsed = column.trim().parse::<usize>().unwrap();
            if parsed <= 0 {
                println!("{}", GameError::OutOfBound(self.board_size.unwrap()));
                continue;
            }
            match self.select(parsed - 1, state) {
                Ok(_) => {
                    let win = self.validate();
                    if win {
                        end = true;
                        println!("Player {} is the winner!", "todo")
                    } else {
                        self.switch();
                    }
                }
                Err(e) => {
                    println!("{e}");
                }
            }
            self.draw().unwrap();
        }
    }

    fn set_board(&mut self) {
        let mut set = false;
        while !set {
            let mut size = String::new();
            println!(
                "{}",
                "How large do you want your board to be?".to_string().blue()
            );
            stdin().read_line(&mut size).unwrap();

            let size = size.trim().parse::<usize>();
            if size.is_err() {
                println!("{}", GameError::BoardSizeLimit());
                continue;
            }

            let num = size.unwrap();
            if num < 6 || num > 12 {
                println!("{}", GameError::BoardSizeLimit());
                continue;
            }

            let mut grid: Vec<Vec<Tile>> = vec![];
            for _ in 0..num {
                grid.push(vec![Tile::new(); num]);
            }
            self.board = Some(grid);
            self.board_size = Some(num);
            set = true;
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
        let board_size = self.board_size.unwrap();
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

    fn switch(&mut self) {
        let next = if self.next == 0 { 1 } else { 0 };
        self.next = next;
    }

    fn get_current_player(&self) -> (&str, TileState) {
        if self.next == 0 {
            return (&self.players.0.as_ref().unwrap().name, TileState::P1);
        } else {
            return (&self.players.1.as_ref().unwrap().name, TileState::P2);
        };
    }

    fn select(&mut self, column: usize, state: TileState) -> Result<(), GameError> {
        let mutable_board = self.board.as_mut().unwrap();
        if column > self.board_size.unwrap() {
            return Err(GameError::OutOfBound(self.board_size.unwrap()));
        }

        for row in mutable_board.iter_mut().rev() {
            let t = row.get_mut(column);

            if t.is_some() {
                let val: &mut Tile = t.unwrap();
                if &val.state == &TileState::Empty {
                    val.set_state(state);
                    return Ok(());
                }
            }
        }
        return Err(GameError::MoveLimited());
    }

    fn validate_straight(&self, state: TileState) -> bool {
        let mut hoz_counter = 0;
        for row in self.board.as_ref().unwrap() {
            for tile in row {
                if tile.state == state {
                    hoz_counter += 1;
                } else {
                    hoz_counter = 0;
                }

                if hoz_counter == 4 {
                    return true;
                }
            }
        }

        let mut ver_counter = 0;
        for column in 0..self.board_size.unwrap() {
            for row in self.board.as_ref().unwrap() {
                let tile = row.get(column).unwrap();
                if tile.state == state {
                    ver_counter += 1;
                } else {
                    ver_counter = 0
                }

                if ver_counter == 4 {
                    return true;
                }
            }
        }
        return false;
    }

    fn validate_diagonal(&self, state: TileState) -> bool {
        let size = self.board_size.unwrap();
        for row in 0..(size - 3) {
            for column in 0..(size - 3) {
                let check = self.check_diagonal_win(true, state, row, column);
                if check {
                    return true;
                } 
            }
        }

        for row in 3..size {
            for column in 0..(size - 3) {
                let check = self.check_diagonal_win(false, state, row, column);
                if check {
                    return true;
                }
            }
        }

        return false;
    }

    fn validate(&self) -> bool {
        let (_, state) = &self.get_current_player();
        let straight_check = self.validate_straight(state.clone());
        let diagonal_check = self.validate_diagonal(state.clone());
        return straight_check || diagonal_check;
    }

    fn check_diagonal_win(
        &self,
        anti: bool,
        state: TileState,
        r_start: usize,
        c_start: usize,
    ) -> bool {
        let t1: TileState = self.get_tile_state(r_start, c_start);
        let t2: TileState;
        let t3: TileState;
        let t4: TileState;
        if anti {
            t2 = self.get_tile_state(r_start + 1, c_start + 1);
            t3 = self.get_tile_state(r_start + 2, c_start + 2);
            t4 = self.get_tile_state(r_start + 3, c_start + 3);
        } else {
            t2 = self.get_tile_state(r_start - 1, c_start + 1);
            t3 = self.get_tile_state(r_start - 2, c_start + 2);
            t4 = self.get_tile_state(r_start - 3, c_start + 3);
        }

        return [t1, t2, t3, t4].iter().all(|t| t == &state);
    }

    fn get_tile_state(&self, row: usize, column: usize) -> TileState {
        let board = self.board.as_ref().unwrap();
        let tile = match board.get(row) {
            Some(r) => r.get(column),
            None => return TileState::Empty,
        };

        if tile.is_none() {
            return TileState::Empty;
        }

        return tile.unwrap().state;
    }
}
