use crate::types::enums::{Stone, BoardSize};

pub struct GameState {
    pub board_size: BoardSize,
    pub board: Vec<Vec<Stone>>,
    pub current_player: Stone,
    pub winner: Option<Stone>,
}

impl GameState {
    pub fn new(board_size: BoardSize) -> Self {
        Self {
            board_size,
            board: vec![vec![Stone::Empty; board_size as usize]; board_size as usize],
            current_player: Stone::Black,
            winner: None,
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> bool {
        if self.winner.is_some() || self.board[row][col] != Stone::Empty {
            return false;
        }

        self.board[row][col] = self.current_player.clone();
        
        if self.check_winner(row, col) {
            self.winner = Some(self.current_player.clone());
        } else {
            self.current_player = match self.current_player {
                Stone::Black => Stone::White,
                Stone::White => Stone::Black,
                Stone::Empty => unreachable!(),
            };
        }
        true
    }

    pub fn check_winner(&self, row: usize, col: usize) -> bool {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
        let stone = &self.board[row][col];

        for (dx, dy) in directions.iter() {
            let mut count = 1;
            
            // Check positive direction
            let mut r = row as i32;
            let mut c = col as i32;
            for _ in 0..4 {
                r += dx;
                c += dy;
                if r < 0 || r >= self.board_size as i32 || c < 0 || c >= self.board_size as i32 {
                    break;
                }
                if &self.board[r as usize][c as usize] != stone {
                    break;
                }
                count += 1;
            }

            // Check negative direction
            let mut r = row as i32;
            let mut c = col as i32;
            for _ in 0..4 {
                r -= dx;
                c -= dy;
                if r < 0 || r >= self.board_size as i32 || c < 0 || c >= self.board_size as i32 {
                    break;
                }
                if &self.board[r as usize][c as usize] != stone {
                    break;
                }
                count += 1;
            }

            if count >= 5 {
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        self.board = vec![vec![Stone::Empty; self.board_size as usize]; self.board_size as usize];
        self.current_player = Stone::Black;
        self.winner = None;
    }
}
