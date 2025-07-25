use crate::{Board, COLS, Cell, Player, ROWS};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct MoveResult {
    pub column: usize,
    pub moves_to_win: Option<u8>, // None if no forced win, Some(n) if win in n moves
}

#[derive(Debug, Clone, Copy)]
struct EvalResult {
    score: i32,
    moves_to_outcome: Option<u8>, // Moves until win/loss (None if draw or uncertain)
}

pub struct Solver {
    memo: HashMap<String, EvalResult>,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    pub fn find_best_move(&mut self, board: &Board, depth: u8) -> Option<MoveResult> {
        if board.is_game_over() {
            return None;
        }

        let valid_moves = board.get_valid_moves();
        if valid_moves.is_empty() {
            return None;
        }

        let mut best_move = valid_moves[0];
        let mut best_moves_to_win: Option<u8> = None;

        // Determine if AI should maximize or minimize based on current player
        // Red maximizes (seeks positive scores), Yellow minimizes (seeks negative scores)
        let ai_maximizes = board.current_player() == Player::Red;
        let mut best_score = if ai_maximizes { i32::MIN } else { i32::MAX };

        for &col in &valid_moves {
            let mut new_board = board.clone();
            new_board.make_move(col);
            // After making the move, it's the opponent's turn, so flip the maximizing flag
            let eval_result = self.minimax(&new_board, depth, i32::MIN, i32::MAX, !ai_maximizes);

            let is_better = if ai_maximizes {
                eval_result.score > best_score
                    || (eval_result.score == best_score
                        && self.is_faster_win(
                            eval_result.moves_to_outcome,
                            best_moves_to_win,
                            ai_maximizes,
                        ))
            } else {
                eval_result.score < best_score
                    || (eval_result.score == best_score
                        && self.is_faster_win(
                            eval_result.moves_to_outcome,
                            best_moves_to_win,
                            ai_maximizes,
                        ))
            };

            if is_better {
                best_score = eval_result.score;
                best_move = col;
                best_moves_to_win = eval_result.moves_to_outcome.map(|m| m + 1); // +1 for the move we're about to make
            }
        }

        Some(MoveResult {
            column: best_move,
            moves_to_win: best_moves_to_win,
        })
    }

    fn is_faster_win(
        &self,
        new_moves: Option<u8>,
        current_best: Option<u8>,
        ai_maximizes: bool,
    ) -> bool {
        match (new_moves, current_best) {
            (Some(new), Some(current)) => {
                if ai_maximizes {
                    new < current
                } else {
                    new > current
                }
            }
            (Some(_), None) => ai_maximizes,
            _ => false,
        }
    }

    fn minimax(
        &mut self,
        board: &Board,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
    ) -> EvalResult {
        if depth == 0 || board.is_game_over() {
            return self.evaluate_board_with_depth(board, depth);
        }

        let board_key = format!("{}_{}", self.board_to_string(board), depth);
        if let Some(&cached_result) = self.memo.get(&board_key) {
            return cached_result;
        }

        let valid_moves = board.get_valid_moves();

        if maximizing {
            let mut best_result = EvalResult {
                score: i32::MIN,
                moves_to_outcome: None,
            };

            for &col in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move(col);
                let eval_result = self.minimax(&new_board, depth - 1, alpha, beta, false);

                let is_better = eval_result.score > best_result.score
                    || (eval_result.score == best_result.score
                        && self.is_faster_win(
                            eval_result.moves_to_outcome,
                            best_result.moves_to_outcome,
                            true,
                        ));

                if is_better {
                    best_result = EvalResult {
                        score: eval_result.score,
                        moves_to_outcome: eval_result.moves_to_outcome.map(|m| m + 1),
                    };
                }

                alpha = alpha.max(eval_result.score);
                if beta <= alpha {
                    break; // Alpha-beta pruning
                }
            }
            self.memo.insert(board_key, best_result);
            best_result
        } else {
            let mut best_result = EvalResult {
                score: i32::MAX,
                moves_to_outcome: None,
            };

            for &col in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move(col);
                let eval_result = self.minimax(&new_board, depth - 1, alpha, beta, true);

                let is_better = eval_result.score < best_result.score
                    || (eval_result.score == best_result.score
                        && self.is_faster_win(
                            eval_result.moves_to_outcome,
                            best_result.moves_to_outcome,
                            false,
                        ));

                if is_better {
                    best_result = EvalResult {
                        score: eval_result.score,
                        moves_to_outcome: eval_result.moves_to_outcome.map(|m| m + 1),
                    };
                }

                beta = beta.min(eval_result.score);
                if beta <= alpha {
                    break; // Alpha-beta pruning
                }
            }
            self.memo.insert(board_key, best_result);
            best_result
        }
    }

    fn evaluate_board_with_depth(&self, board: &Board, _depth: u8) -> EvalResult {
        if let Some(winner) = board.winner() {
            return match winner {
                Player::Red => EvalResult {
                    score: 1000,
                    moves_to_outcome: Some(0),
                },
                Player::Yellow => EvalResult {
                    score: -1000,
                    moves_to_outcome: Some(0),
                },
            };
        }

        if board.is_game_over() {
            return EvalResult {
                score: 0,
                moves_to_outcome: Some(0),
            }; // Draw
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

        EvalResult {
            score,
            moves_to_outcome: None,
        }
    }

    fn evaluate_window(
        &self,
        board: &Board,
        row: usize,
        col: usize,
        delta_row: i32,
        delta_col: i32,
    ) -> i32 {
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
