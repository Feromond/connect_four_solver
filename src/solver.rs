use std::collections::HashMap;
use crate::{Board, Player, Cell, ROWS, COLS};

pub struct Solver {
    memo: HashMap<String, i32>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    pub fn find_best_move(&mut self, board: &Board, depth: u8) -> Option<usize> {
        if board.is_game_over() {
            return None;
        }

        let valid_moves = board.get_valid_moves();
        if valid_moves.is_empty() {
            return None;
        }

        let mut best_move = valid_moves[0];
        let mut best_score = i32::MIN;

        for &col in &valid_moves {
            let mut new_board = board.clone();
            new_board.make_move(col);
            let score = self.minimax(&new_board, depth, i32::MIN, i32::MAX, false);
            
            if score > best_score {
                best_score = score;
                best_move = col;
            }
        }

        Some(best_move)
    }

    fn minimax(&mut self, board: &Board, depth: u8, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
        if depth == 0 || board.is_game_over() {
            return self.evaluate_board(board);
        }

        let board_key = self.board_to_string(board);
        if let Some(&cached_score) = self.memo.get(&board_key) {
            return cached_score;
        }

        let valid_moves = board.get_valid_moves();
        
        if maximizing {
            let mut max_eval = i32::MIN;
            for &col in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move(col);
                let eval = self.minimax(&new_board, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break; // Alpha-beta pruning
                }
            }
            self.memo.insert(board_key, max_eval);
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &col in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move(col);
                let eval = self.minimax(&new_board, depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break; // Alpha-beta pruning
                }
            }
            self.memo.insert(board_key, min_eval);
            min_eval
        }
    }

    fn evaluate_board(&self, board: &Board) -> i32 {
        if let Some(winner) = board.winner() {
            return match winner {
                Player::Red => 1000,
                Player::Yellow => -1000,
            };
        }

        if board.is_game_over() {
            return 0; // Draw
        }

        // Simple heuristic: evaluate based on potential winning positions
        let mut score = 0;
        
        // Evaluate all possible 4-in-a-row positions
        for row in 0..ROWS {
            for col in 0..COLS {
                score += self.evaluate_window(board, row, col, 0, 1); // Horizontal
                score += self.evaluate_window(board, row, col, 1, 0); // Vertical
                score += self.evaluate_window(board, row, col, 1, 1); // Diagonal /
                score += self.evaluate_window(board, row, col, 1, -1); // Diagonal \
            }
        }

        score
    }

    fn evaluate_window(&self, board: &Board, row: usize, col: usize, delta_row: i32, delta_col: i32) -> i32 {
        let mut red_count = 0;
        let mut yellow_count = 0;
        let mut empty_count = 0;

        for i in 0..4 {
            let r = row as i32 + i * delta_row;
            let c = col as i32 + i * delta_col;

            if r < 0 || r >= ROWS as i32 || c < 0 || c >= COLS as i32 {
                return 0; // Out of bounds
            }

            match board.get_cell(r as usize, c as usize) {
                Cell::Occupied(Player::Red) => red_count += 1,
                Cell::Occupied(Player::Yellow) => yellow_count += 1,
                Cell::Empty => empty_count += 1,
            }
        }

        if red_count > 0 && yellow_count > 0 {
            return 0; // Mixed window
        }

        if red_count == 4 {
            return 1000;
        } else if yellow_count == 4 {
            return -1000;
        } else if red_count == 3 && empty_count == 1 {
            return 10;
        } else if yellow_count == 3 && empty_count == 1 {
            return -10;
        } else if red_count == 2 && empty_count == 2 {
            return 2;
        } else if yellow_count == 2 && empty_count == 2 {
            return -2;
        }

        0
    }

    fn board_to_string(&self, board: &Board) -> String {
        let mut result = String::new();
        for row in 0..ROWS {
            for col in 0..COLS {
                match board.get_cell(row, col) {
                    Cell::Empty => result.push('0'),
                    Cell::Occupied(Player::Red) => result.push('1'),
                    Cell::Occupied(Player::Yellow) => result.push('2'),
                }
            }
        }
        result.push(match board.current_player() {
            Player::Red => '1',
            Player::Yellow => '2',
        });
        result
    }
} 