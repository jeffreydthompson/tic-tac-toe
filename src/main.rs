use std::{io, num::ParseIntError};

use crate::game::{Game, PlayerSet, Player, PlayerType};
use crate::board::{Board, XPos, YPos};

mod board;
mod game;

fn main() {
    play_game();
    println!("Thanks for playing!");
}

fn play_game() { 
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
                            Err(error) => { panic!("THIS shouldn't happpen!! {:?}", error); }
                        }
                    }
                }
                
                continue;
            }
        }

        is_playing = false;
    }
}

fn player_input(set: PlayerSet, turn: Player, board: Board) -> Game { 

    let mut output_game = Game::Uninitiated;
    let mut is_awaiting_input = true;

    while is_awaiting_input { 
        
        board.pretty_print();
        println!("{}, please enter move A1 thru C3:", turn.to_string());

        let mut in_buffer = String::new();

        io::stdin()
        .read_line(&mut in_buffer)
        .expect("failed to read");

        in_buffer = in_buffer.to_ascii_uppercase();
        let letter = &in_buffer[..1];
        let number = &in_buffer[1..];
        let x_pos: usize;
        let y_pos: usize;

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
            Ok(result_game) => {
                output_game = result_game;
                is_awaiting_input = false; 
            },
            Err(error) => { 
                println!("Can't move there. Please choose another move. {:?}", error);
                continue;
            }
        }
    }

    output_game
}

fn init_game() -> Game {
    let mut game = Game::Uninitiated;
    let mut is_init = true;

    while is_init {
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

        game = Game::InPlay { set: player_set, turn: player_set.x, board: board };
        is_init = false;
    }

    game
}