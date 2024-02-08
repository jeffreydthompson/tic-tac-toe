use core::num;
use std::{io, num::ParseIntError};

use crate::game::{Game, PlayerSet, Player, PlayerType};
use crate::board::{Board, XPos, YPos};

mod board;
mod game;

fn main() {

    let mut game = init_game();
    
    let mut is_playing = true;
    while is_playing { 
        
        match game { 
            Game::Uninitiated => { println!("wait.. this shouldn't happen"); },
            Game::Tie(board) => { 
                println!("Game is tied");
                board.pretty_print(); 
            },
            Game::Win(winner, board) => { 
                println!("{} Wins!", winner.to_string());
                board.pretty_print();
            },
            Game::InPlay { set, turn, board } => { 
                match turn.player_type() { 
                    PlayerType::Human => { game = player_input(set, turn, board); },
                    PlayerType::Computer => { 
                        let computer_move = game::computer_move(&turn, &set, &board);
                        match computer_move { 
                            Ok(updated_game) => { game = updated_game; },
                            Err(error) => { panic!("THIS shouldn't happpen!!"); }
                        }
                    }
                }
                
                continue;
            }
        }

        is_playing = false;
    }

    println!("Thanks for playing!");
}

fn player_input(set: PlayerSet, turn: Player, board: Board) -> Game { 

    while true { 
        
        board.pretty_print();
        println!("{}, please enter move A1 thru C3:", turn.to_string());

        let mut in_buffer = String::new();

        io::stdin()
        .read_line(&mut in_buffer)
        .expect("failed to read");

        in_buffer.to_ascii_uppercase();
        let letter = &in_buffer[..1];
        let number = &in_buffer[1..];
        let mut x_pos: usize = 0;
        let mut y_pos: usize = 0;

        match letter { 
            "A" => { x_pos = XPos::A; },
            "B" => { x_pos = XPos::B; },
            "C" => { x_pos = XPos::C; },
            _ => { 
                println!("Letter. Please enter Letter (A-C) & Number (1-3) format. ie: A1, C2, etc");
                continue; 
            }
        }

        match number.trim() { 
            "1" => { y_pos = YPos::_1; },
            "2" => { y_pos = YPos::_2; },
            "3" => { y_pos = YPos::_3; },
            _ => { 
                println!("Number. Please enter Letter (A-C) & Number (1-3) format. ie: A1, C2, etc");
                continue; 
            }
        }

        let copy_board = board.clone();
        let pos = &(x_pos, y_pos);
        
        let move_result = game::make_move(pos, &copy_board, &turn, &set);

        match move_result { 
            Ok(game) => {
                return game; 
            },
            Err(error) => { 
                println!("Can't move there. Please choose another move.");
                continue;
            }
        }
    }

    Game::Uninitiated
}

fn init_game() -> Game { 
    while true {
        println!("Tic Tac Toe.  Enter an option: (X goes first)");
        println!("1. X: Human, O: Computer");
        println!("2: X: Human, O: Human");
        println!("3: X: Computer, O: Human");

        let mut in_buffer = String::new();

        io::stdin()
        .read_line(&mut in_buffer)
        .expect("failed to read");

        let option: Result<i32, ParseIntError> = in_buffer
        .trim()
        .parse();

        if option.is_err() { 
            println!("Please enter a number");
            continue; 
        }

        let num_input: i32 = option.unwrap();
        
        let mut x_type = PlayerType::Human;
        let mut o_type = PlayerType::Human;

        match num_input {
            1 => { o_type = PlayerType::Computer; },
            2 => { },
            3 => { x_type = PlayerType::Computer; }
            _ => { 
                println!("invalid input, try again");
                continue;
            }
        }

        let x = Player::X(x_type);
        let o = Player::O(o_type);
        let player_set = PlayerSet { x: x, o: o };
        let board = Board::default();
        return Game::InPlay { set: player_set, turn: player_set.x, board: board }
    }

    Game::Uninitiated
}