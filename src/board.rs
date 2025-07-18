use crate::{Player, ROWS, COLS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: [[Cell; COLS]; ROWS],
    current_player: Player,
    game_over: bool,
    winner: Option<Player>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; COLS]; ROWS],
            current_player: Player::Red,
            game_over: false,
            winner: None,
        }
    }

    pub fn make_move(&mut self, col: usize) -> bool {
        if self.game_over || col >= COLS {
            return false;
        }

        // Find the lowest empty row in the column
        for row in (0..ROWS).rev() {
            if self.grid[row][col] == Cell::Empty {
                self.grid[row][col] = Cell::Occupied(self.current_player);
                
                // Check for win
                if self.check_win(row, col) {
                    self.game_over = true;
                    self.winner = Some(self.current_player);
                } else if self.is_board_full() {
                    self.game_over = true;
                    self.winner = None; // Draw
                } else {
                    self.current_player = self.current_player.opposite();
                }
                return true;
            }
        }
        false // Column is full
    }

    pub fn is_valid_move(&self, col: usize) -> bool {
        if self.game_over || col >= COLS {
            return false;
        }
        self.grid[0][col] == Cell::Empty
    }

    pub fn get_valid_moves(&self) -> Vec<usize> {
        (0..COLS).filter(|&col| self.is_valid_move(col)).collect()
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        let player = match self.grid[row][col] {
            Cell::Occupied(p) => p,
            Cell::Empty => return false,
        };

        // Check horizontal
        if self.count_direction(row, col, 0, 1, player) + 
           self.count_direction(row, col, 0, -1, player) + 1 >= 4 {
            return true;
        }

        // Check vertical
        if self.count_direction(row, col, 1, 0, player) + 
           self.count_direction(row, col, -1, 0, player) + 1 >= 4 {
            return true;
        }

        // Check diagonal (/)
        if self.count_direction(row, col, 1, 1, player) + 
           self.count_direction(row, col, -1, -1, player) + 1 >= 4 {
            return true;
        }

        // Check diagonal (\)
        if self.count_direction(row, col, 1, -1, player) + 
           self.count_direction(row, col, -1, 1, player) + 1 >= 4 {
            return true;
        }

        false
    }

    fn count_direction(&self, row: usize, col: usize, delta_row: i32, delta_col: i32, player: Player) -> usize {
        let mut count = 0;
        let mut r = row as i32 + delta_row;
        let mut c = col as i32 + delta_col;

        while r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 {
            if let Cell::Occupied(p) = self.grid[r as usize][c as usize] {
                if p == player {
                    count += 1;
                    r += delta_row;
                    c += delta_col;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        count
    }

    fn is_board_full(&self) -> bool {
        self.grid[0].iter().all(|&cell| cell != Cell::Empty)
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.grid[row][col]
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn reset(&mut self) {
        self.grid = [[Cell::Empty; COLS]; ROWS];
        self.current_player = Player::Red;
        self.game_over = false;
        self.winner = None;
    }
} 