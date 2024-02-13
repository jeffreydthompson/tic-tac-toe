#[non_exhaustive]
pub struct XPos;
impl XPos {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const C: usize = 2;

    fn letter_from(num: usize) -> &'static str { 
        match num { 
            0 => "A",
            1 => "B",
            _ => "C"
        }
    }
}

#[non_exhaustive]
pub struct YPos;
impl YPos {
    pub const _1: usize = 0;
    pub const _2: usize = 1;
    pub const _3: usize = 2;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Square { 
    X, O, Empty
}

impl Square { 
    pub fn to_string(&self) -> &str { 
        match self { 
            Self::X => "❌",
            Self::O => "⭕️",
            Self::Empty => "⬜️"
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board { 
    pub squares: [[Square; 3]; 3]
}

impl Default for Board { 
    fn default() -> Self { 
        Board { squares: [[Square::Empty; 3]; 3] }
    }
}

impl Board { 
    pub fn pretty_print(&self) { 
        for x in XPos::A ..= XPos::C { 
            let letter = XPos::letter_from(x);
            let squares = self.squares[x].iter()
            .map(|f| f.to_string())
            .collect::<String>();
            println!("{} {}", letter, squares);
        }

        println!("  1️⃣ 2️⃣ 3️⃣");
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::board::{Square, Board, XPos, YPos};

    #[derive(Debug)]
    pub struct BoardBuildError;
    #[derive(Debug)]
    pub struct SquareBuildError;

    impl FromStr for Square { 
        type Err = SquareBuildError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s { 
                "X" => { Ok(Self::X) },
                "O" => { Ok(Self::O) },
                "-" => { Ok(Self::Empty) },
                _ => { Err(SquareBuildError) }
            }
        }
    }

    impl FromStr for Board { 
        type Err = BoardBuildError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.split("\n").count() != 3 { return Err(BoardBuildError); }

            let build_string = s.split("\n");
            let mut board = Board { squares: [[Square::Empty; 3]; 3] };

            for (y, s) in build_string.into_iter().enumerate() {
                if s.len() != 3 { return Result::Err(BoardBuildError); }
                for (x, c) in s.chars().enumerate() {
                    let string = c.to_string(); 
                    let str = string.as_str();
                    let Ok(sq) = Square::from_str(str) else { return Err(BoardBuildError); };
                    board.squares[y][x] = sq;
                }
            }

            Ok(board)
        }
    }

    #[test]
    fn test_square_to_string() { 
        let x = Square::X;
        assert_eq!(x.to_string(), "❌");
        let o = Square::O;
        assert_eq!(o.to_string(), "⭕️");
        let empty = Square::Empty;
        assert_eq!(empty.to_string(), "⬜️");
    }

    #[test]
    fn test_board_default() { 
        let board = Board::default();
        let one = board.squares[0][0];
        assert_eq!(one.to_string(), "⬜️");
        let len_y = board.squares.len();
        let len_x = board.squares[0].len();
        assert_eq!(len_y, 3);
        assert_eq!(len_x, 3);
    }

    #[test]
    fn test_building_strings() { 

        let build_string = empty_build_string().split("\n");        

        {
            let len = &build_string.clone().count();
            assert_eq!(len, &3);
        }

        for(_a, b) in build_string.into_iter().enumerate() { 
            let v: Vec<char> = b.chars().collect();

            for vval in v { 
                let derive_str = vval.to_string();
                let str = derive_str.as_str();
                let square = Square::from_str(str).unwrap();
                assert_eq!(square.to_string(), "⬜️");
            }
        }
        
        let board = Board::from_str(empty_build_string());
        assert!(board.is_ok());
        assert_eq!(board.unwrap().squares.len(), 3);

        let x_win_board = Board::from_str(x_win_build_string()).unwrap();
        assert_eq!(x_win_board.squares[XPos::A][YPos::_1].to_string(), "❌");
        assert_eq!(x_win_board.squares[XPos::C][YPos::_3].to_string(), "❌");

        let board = Board::from_str(empty_build_string());
        assert!(board.is_ok());
        assert_eq!(board.unwrap().squares.len(), 3);

        let board_from_str = Board::from_str(x_win_build_string());
        assert!(board_from_str.is_ok());
    }

    fn empty_build_string() -> &'static str { 
        "---\n---\n---"
    }

    fn x_win_build_string() -> &'static str { 
        "X-O\nOO-\nXXX"
    }

}