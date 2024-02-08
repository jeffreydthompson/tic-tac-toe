use std::collections::HashMap;

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
    use crate::board::{Square, Board, XPos, YPos};

    impl Square { 
        pub fn from_string(string: &str) -> Self { 
            match string { 
                "X" => { Square::X },
                "O" => { Square::O },
                _ => { Square::Empty },
            }
        }
    }

    impl Board {

        pub fn build_from_string(input_str: &str) -> Self {
            let build_string = input_str.split("\n");

            let mut board = Board { squares: [[Square::Empty; 3]; 3] };

            for (y, str) in build_string.into_iter().enumerate() { 
                for (x, ch) in str.chars().enumerate() { 
                    let cast_str = ch.to_string();
                    board.squares[y][x] = Square::from_string(&cast_str);
                }
            }

            board
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

        for(a, b) in build_string.into_iter().enumerate() { 
            // println!("index: {a}, str: {b}");
            let v: Vec<char> = b.chars().collect();

            for vval in v { 
                let square = Square::from_string("{vval}");
                assert_eq!(square.to_string(), "⬜️");
            }
        }
        
        let board = Board::build_from_string(empty_build_string());
        assert_eq!(board.squares.len(), 3);

        let x_win_board = Board::build_from_string(x_win_build_string());
        assert_eq!(x_win_board.squares[XPos::A][YPos::_1].to_string(), "❌");
        assert_eq!(x_win_board.squares[XPos::C][YPos::_3].to_string(), "❌");
    }

    // #[test]
    fn test_odd_char_casting() { 
        let odd_char_str = "❌⬜️⬜️\n⬜️⬜️⬜️\n⬜️⬜️⭕️".split("\n");
        for (a, b) in odd_char_str.into_iter().enumerate() { 
            let v_char: Vec<char> = b.chars().collect();
            let len = v_char.len();
            println!("v_char len: {}", len);

            for f_ch in v_char {
                println!("ch to cast: {}", f_ch);
                let cast_ch = f_ch.to_string();
                match char::from_u32(65039) { 
                    Some(ch) => { 
                        if ch == f_ch { continue; }
                     }
                    None => { break; }
                }
                // if cast_ch != "❌" && cast_ch != "⭕️" && cast_ch != "⬜️" { continue; }
                println!("cast ch: {}", cast_ch);

                let mut byte_ary: [u16; 4] = [0; 4];
                let ary = f_ch.encode_utf16(&mut byte_ary);
                for byte in ary { 
                    println!("byte: {}", byte);
                }
            }
        }
    }

    fn empty_build_string() -> &'static str { 
        "---\n---\n---"
    }

    fn x_win_build_string() -> &'static str { 
        "X-O\nOO-\nXXX"
    }

}