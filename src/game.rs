use std::collections::HashMap;
use std::io::empty;
use std::fmt;
use std::ops::Add;
use std::usize;

use crate::board::Board;
use crate::board::Square;
use crate::board::XPos;
use crate::board::YPos;

// type Result<T> = std::result::Result<T, PlacementError>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerType { 
    Human, Computer
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player { 
    X(PlayerType), 
    O(PlayerType)
}

impl Player { 
    pub fn to_string(&self) -> &str { 
        match self { 
            Self::X(_t) => "❌",
            Self::O(_t) => "⭕️"
        }
    }

    pub fn associated_square(&self) -> Square { 
        match self { 
            Self::X(_t) => Square::X,
            Self::O(_t) => Square::O,
        }
    }

    pub fn player_type(&self) -> &PlayerType { 
        match self { 
            Self::X(t) => t,
            Self::O(t) => t
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PlayerSet { 
    pub x: Player,
    pub o: Player
}

impl PlayerSet { 
    pub fn opposite_player(&self, player: &Player) -> Player { 
        match player { 
            Player::O(_t) => self.x,
            Player::X(_t) => self.o,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Game { 
    Uninitiated,
    InPlay {set: PlayerSet, turn: Player, board: Board},
    Tie(Board),
    Win(Player, Board)
}

#[derive(Debug, Clone)]
pub struct MoveError;
impl std::fmt::Display for MoveError { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no moves.  something is wrong")
    }
}

#[derive(Debug, Clone)]
pub struct PlacementError;
impl std::fmt::Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn pretty_print(game: &Game) { 
    match game { 
        Game::Uninitiated => { println!("uniniated"); },
        Game::Tie(board) => { board.pretty_print(); },
        Game::Win(_, board) => { board.pretty_print(); },
        Game::InPlay { board, .. } => { board.pretty_print(); }
    }
}

pub fn make_move(
    pos: &(usize, usize), 
    board: &Board, 
    player: &Player,
    set: &PlayerSet) -> Result<Game, PlacementError> { 

        let mut copy_board = board.clone();

        if copy_board.squares[pos.0][pos.1] != Square::Empty { 
            return Result::Err(PlacementError);
        }

        copy_board.squares[pos.0][pos.1] = player.associated_square();

        if is_win(&player, &copy_board) { 
            return Result::Ok(Game::Win(*player, copy_board));
        } else if is_tie(&copy_board) { 
            return Result::Ok(Game::Tie(copy_board));
        }

        let next_turn = set.opposite_player(&player);
        let updated_game = Game::InPlay { set: *set, turn: next_turn, board: copy_board };
        return Result::Ok(updated_game);
}

impl Board { 
    pub fn get_empty_squares(&self) -> Vec<(usize, usize)> { 
        let mut pos = Vec::new();

        for x in XPos::A ..= XPos::C { 
            for y in YPos::_1 ..= YPos::_3 {
                if self.squares[x][y] == Square::Empty { 
                    pos.push((x,y));
                }
            }
        }

        pos
    }

    pub fn get_positions_for(&self, player: &Player) -> Vec<(usize, usize)> { 
        let mut pos = Vec::new();

        for x in XPos::A ..= XPos::C { 
            for y in YPos::_1 ..= YPos::_3 {
                if self.squares[x][y] == player.associated_square() {
                    pos.push((x, y));
                }
            }
        }

        pos
    }
}

fn new_game(set: &PlayerSet) -> Game { 
    Game::InPlay { set: *set, turn: set.x, board: Board::default() }
}

fn is_tie(board: &Board) -> bool { 
    let empty = board.get_empty_squares();
    if empty.len() != 0 { return false; }
    let x_win = is_win(&Player::X(PlayerType::Human), board);
    let o_win = is_win(&Player::O(PlayerType::Human), board);
    !(x_win || o_win)
}

fn is_win(player: &Player, board: &Board) -> bool { 
    let mut row_ctr_ary: [u8; 3] = [0; 3];
    let mut col_ctr_ary: [u8; 3] = [0; 3];
    let mut l_diag_ctr: u8 = 0;
    let mut r_diag_ctr: u8 = 0;

    let positions = board.get_positions_for(player);

    for x in XPos::A ..= XPos::C { 
        for y in YPos::_1 ..= YPos::_3 { 
            if positions.contains(&(x, y)) { 
                row_ctr_ary[x] += 1;
                col_ctr_ary[y] += 1;

                if x == y { l_diag_ctr += 1; }
                if x == (2-y) { r_diag_ctr += 1; }
            }
        }
    }

    row_ctr_ary.contains(&3) || 
    col_ctr_ary.contains(&3) || 
    l_diag_ctr == 3 ||
    r_diag_ctr == 3
}

pub fn computer_move(
    turn: &Player, 
    set: &PlayerSet, 
    board: &Board) -> Result<Game, MoveError> {

        let mut chosen_pos: Option<(usize, usize)> = Option::None;
        let mut high_score = -2;
        let open_pos = board.get_empty_squares();

        if open_pos.len() == 9 { 
            // empty board, move to corner.
            let mut copy_board = board.clone();
            copy_board.squares[XPos::A][YPos::_1] = turn.associated_square();
            let game = Game::InPlay { set: *set, turn: set.opposite_player(turn), board: copy_board };
            return Result::Ok(game);
        }

        // println!("comp_move: starting board: {:?}", board.squares);
        // println!("comp_move: search for positions: {:?}", open_pos);

        for pos in open_pos { 
            let mut copy_board = board.clone();
            copy_board.squares[pos.0][pos.1] = turn.associated_square();
            // println!("comp_move: begin recursion for {} at pos: {:?}. board: {:?}", turn.to_string(), pos, copy_board.squares);
            let score = minimax(turn, turn, set, &copy_board, &0);
            if score == 1 {
                // println!("comp_move: {:?} is win for {}", pos, turn.to_string()); 
                chosen_pos = Option::Some(pos); 
                break;
            } else if score > high_score {
                // println!("comp_move: NEW high_score {} at {:?} for {}", score, pos, turn.to_string());
                high_score = score;
                chosen_pos = Option::Some(pos); 
            } else { 
                // println!("comp_move: NOT high_score {} at {:?} for {}", score, pos, turn.to_string());
            }
        }

        match chosen_pos {
            None => { Result::Err(MoveError) },
            Some(pos) => {
                // println!("comp_move: chose pos: {:?}", pos);
                let mut copy_board = board.clone();
                copy_board.squares[pos.0][pos.1] = turn.associated_square();

                if is_tie(&copy_board) { 
                    return Result::Ok(Game::Tie(copy_board));
                } else if is_win(turn, &copy_board) { 
                    return Result::Ok(Game::Win(*turn, copy_board));
                }

                let next_turn = set.opposite_player(turn);
                let game = Game::InPlay { set: *set, turn: next_turn, board: copy_board };
                Result::Ok(game)
            }
        }
}

fn minimax(
    turn: &Player,
    maximizing_player: &Player, 
    set: &PlayerSet, 
    board: &Board,
    depth: &usize) -> i32 { 

        let space = spacer(depth);

        if is_tie(board) { 
            // println!("{space}minmax_ recursion_end board: {:?} is tie. 0", board);
            return 0; 
        }
        if is_win(turn, board) { 
            let score: i32 = if turn == maximizing_player { 1 } else { -1 };
            // println!("{space}minmax_ recursion_end board: {:?} is win for {}. {}", board, turn.to_string(), score);
            return score; 
        }

        let minimizing_player = set.opposite_player(maximizing_player);
        let remaining_positions = board.get_empty_squares();
        let next_turn = set.opposite_player(turn);

        // println!("{space}minmax_ remaining_positions: {:?}.  turn: {}", remaining_positions, next_turn.to_string());

        let mut scores: Vec<i32> = Vec::new();

        for pos in remaining_positions {
            // println!("{space}minmax_ {} move to {:?}", next_turn.to_string(), pos);
            let mut copy_board = board.clone();
            copy_board.squares[pos.0][pos.1] = next_turn.associated_square();
            let next_depth = *depth + 1;
            let result = minimax(&next_turn, maximizing_player, set, &copy_board, &next_depth);

            scores.insert(0, result);

            if &next_turn == maximizing_player { 
                if result == 1 {
                    // println!("{space}minmax_ Found winning path for {} at {:?}. returning: {}", next_turn.to_string(), pos, result);
                    return 1; 
                }
            } else { 
                if result == -1 { 
                    // println!("{space}minmax_ Found winning path for {} at {:?}. returning: {}", next_turn.to_string(), pos, result);
                    return -1; 
                }
            }
        }

        if &next_turn == maximizing_player { 
            scores.sort();
            let result = scores.last().unwrap_or(&0);
            // println!("{space}SCORES: {:?}. turn: {}. return: {}", scores, next_turn.to_string(), result);
            return *result;
        } else { 
            scores.sort();
            let result = scores.first().unwrap_or(&0);
            // println!("{space}SCORES: {:?}. turn: {}. return: {}", scores, next_turn.to_string(), result);
            return *result;
        }

        return 0;
}

fn spacer(depth: &usize) -> String { 
    std::iter::repeat("  ").take(*depth).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::{board::{Board, XPos, YPos}, game::{Player, PlayerType}};

    use super::{is_win, make_move, PlayerSet, Game, is_tie, minimax, computer_move};

    fn player_x() -> &'static Player { 
        &Player::X(PlayerType::Human)
    }

    fn player_o() -> &'static Player { 
        &Player::O(PlayerType::Human)
    }

    fn player_set() -> &'static PlayerSet { 
        &PlayerSet { 
            x: Player::X(PlayerType::Human), 
            o: Player::O(PlayerType::Human) 
        }
    }

    #[test]
    fn test_make_move() {
        let almost_win_board = &Board::build_from_string(x_almost_win_build_string());
        let winning_move = &(XPos::C, YPos::_2);
        let win_game_result = make_move(
            winning_move, 
            almost_win_board, 
            player_x(), 
            player_set()
        );

        let assert_board = Board::build_from_string(x_win_build_string());
        let assert_game = Game::Win(*player_x(), assert_board);

        assert!(win_game_result.is_ok());
        assert_eq!(win_game_result.unwrap(), assert_game);

        let almost_tie_board = Board::build_from_string(almost_tie_build_string());
        let tie_move = &(XPos::A, YPos::_1);
        let tie_game_result = make_move(
            tie_move, 
            &almost_tie_board, 
            player_x(), 
            player_set()
        ); 

        let assert_board = Board::build_from_string(tie_build_string());
        let assert_game = Game::Tie(assert_board);

        assert!(tie_game_result.is_ok());
        assert_eq!(tie_game_result.unwrap(), assert_game);

        let empty_board = Board::build_from_string(empty_build_string());
        let x_move = &(XPos::A, YPos::_1);
        let move_result = make_move(
            x_move, 
            &empty_board, 
            player_x(),
            player_set()
        );

        let assert_board = Board::build_from_string(one_move_build_string());
        let assert_game = Game::InPlay { set: *player_set(), turn: *player_o(), board: assert_board };

        assert!(move_result.is_ok());
        assert_eq!(move_result.unwrap(), assert_game);

    }

    #[test]
    fn test_get_x_positions() {
        let test_board = Board::build_from_string(x_win_build_string());
        let x_pos = test_board.get_positions_for(player_x());
        assert!(!x_pos.is_empty());

        assert!(x_pos.contains(&(XPos::A,YPos::_1)));
        assert!(x_pos.contains(&(XPos::C,YPos::_1)));
        assert!(x_pos.contains(&(XPos::C,YPos::_2)));
        assert!(x_pos.contains(&(XPos::C,YPos::_3)));
        assert_eq!(x_pos.len(), 4);
    }

    #[test]
    fn test_get_empty() { 
        let test_board = Board::build_from_string(x_win_build_string());
        let empty_pos = test_board.get_empty_squares();
        assert!(!empty_pos.is_empty());

        assert!(empty_pos.contains(&(XPos::A, YPos::_2)));
        assert!(empty_pos.contains(&(XPos::B, YPos::_3)));

        assert_eq!(empty_pos.len(), 2);
    }

    #[test]
    fn test_win() { 
        let board = Board::build_from_string(empty_build_string());
        let empty_win = is_win(player_o(), &board);
        assert!(!empty_win);

        let board = Board::build_from_string(tie_build_string());
        let tie_win = is_win(player_o(), &board);
        assert!(!tie_win);

        let board = Board::build_from_string(l_diag_win_string());
        let o_d_win = is_win(player_o(), &board);
        assert!(o_d_win);
        let x_d_win = is_win(player_x(), &board);
        assert!(!x_d_win);
    }

    #[test]
    fn test_is_tie() { 
        let board = Board::build_from_string(tie_build_string());
        let tie = is_tie(&board);
        assert!(tie);

        let board = Board::build_from_string(x_win_build_string());
        let tie = is_tie(&board);
        assert!(!tie);

        let board = Board::build_from_string(empty_build_string());
        let tie = is_tie(&board);
        assert!(!tie);
    }
    
    #[test]
    fn test_min_max() { 
        let set = &PlayerSet { 
            x: Player::X(PlayerType::Computer), 
            o: Player::O(PlayerType::Computer) };
        let board = Board::build_from_string(x_minmax_setup_str());

        // FROM HERE
        let mut winning_pos: Option<(usize, usize)> = Option::None;
        let mut high_score = 0;
        let open_pos = board.get_empty_squares();

        for pos in open_pos { 
            println!("**** search for pos: {:?}", pos);
            let mut copy_board = board.clone();
            copy_board.squares[pos.0][pos.1] = set.x.associated_square();
            let score = minimax(&set.x, &set.x, set, &copy_board, &0);
            println!("**** score {} for pos: {:?}", score, pos);
            if score == 1 { 
                winning_pos = Option::Some(pos); 
                break;
            }
        }
        // TO HERE should not be in a test function

        assert!(winning_pos.is_some());
        assert_eq!(winning_pos.unwrap(), (XPos::B, YPos::_2));
    }

    #[test]
    fn test_computer_move_defend() {
        let set = &PlayerSet { 
            x: Player::X(PlayerType::Computer), 
            o: Player::O(PlayerType::Computer) };
        let board = Board::build_from_string(o_defend_setup_str());

        let result = computer_move(&set.o, set, &board);
        assert!(result.is_ok());
        let res_game = result.unwrap();

        let exp_board = Board::build_from_string(o_defend_exp_str());
        let exp_game = Game::InPlay { set: *set, turn: set.x, board: exp_board };

        assert_eq!(res_game, exp_game);
    }
    
    #[test]
    fn test_computer_move_win() {
        let set = &PlayerSet { 
            x: Player::X(PlayerType::Computer), 
            o: Player::O(PlayerType::Computer) };
        let board = Board::build_from_string(x_win_setup_str());
        let result = computer_move(&set.x, set, &board);
        assert!(result.is_ok());
        let res_game = result.unwrap();

        let exp_board = Board::build_from_string(x_win_exp_str());
        let exp_game = Game::Win(set.x, exp_board);

        assert_eq!(res_game, exp_game)
    }

    fn empty_build_string() -> &'static str { 
        "---\n---\n---"
    }

    fn one_move_build_string() -> &'static str { 
        "X--\n---\n---"
    }

    fn almost_tie_build_string() -> &'static str { 
        "-OX\nOOX\nXXO"
    }

    fn tie_build_string() -> &'static str { 
        "XOX\nOOX\nXXO"
    }

    fn x_almost_win_build_string() -> &'static str { 
        // A:123\B:123\C:123
        "X-O\nOO-\nX-X"
    }

    fn x_win_build_string() -> &'static str { 
        // A:123\B:123\C:123
        "X-O\nOO-\nXXX"
    }

    fn row_win_string() -> &'static str { 
        // A:123\B:123\C:123
        "X-O\nOO-\nXXX"
    }

    fn col_win_string() -> &'static str { 
        // A:123\B:123\C:123
        "O-O\nOXX\nOXX"
    }

    fn l_diag_win_string() -> &'static str { 
        // A:123\B:123\C:123
        "O-O\nXOX\nXXO"
    }

    fn r_diag_win_string() -> &'static str { 
        // A:123\B:123\C:123
        "O-X\nOX-\nXOX"
    }

    fn x_minmax_setup_str() -> &'static str { 
        "O-X\nO--\nXO-"
    }

    fn x_minmax_result_str() -> &'static str { 
        "O-X\nOX-\nXO-"
    }

    fn o_defend_setup_str() -> &'static str { 
        "-XO\nO--\nX-X"
    }

    fn o_defend_exp_str() -> &'static str { 
        "-XO\nO--\nXOX"
    }

    fn x_win_setup_str() -> &'static str { 
        "-XO\nOO-\nX-X"
    }

    fn x_win_exp_str() -> &'static str { 
        "-XO\nOO-\nXXX"
    }

}

// "do paced laps", toots pal decapod

//cargo test --package 'tic-tac-toe' --bin 'tic-tac-toe' -- 'game::tests::test_computer_move' --exact  --nocapture