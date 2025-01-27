#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece {
    None,
    Red,
    Black,
}

pub fn make_blank_board() -> [[BoardPiece; 5]; 5] {
    return [[BoardPiece::None; 5]; 5];
} 

pub struct GameState {
    pub board: [[BoardPiece; 5]; 5],
}

impl GameState {
    pub fn jumble_board(&mut self) {
        self.board[1][0] = BoardPiece::Red;
        self.board[2][0] = BoardPiece::Black;
    }

    pub fn print_board(&self) {
        let mut label = String::new();
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == BoardPiece::None {
                    label = "-".to_string();
                }
                else if self.board[row][col] == BoardPiece::Red {
                    label = "R".to_string();
                }
                else if self.board[row][col] == BoardPiece::Black {
                    label = "B".to_string();
                }

                print!("{}", label);
            }
            println!();
        }
        println!();
    }
}