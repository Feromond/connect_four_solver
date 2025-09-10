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

    #[cfg(test)]
    fn clear_cache(&mut self) {
        self.memo.clear();
    }

    pub fn find_best_move(&mut self, board: &Board, depth: u8) -> Option<MoveResult> {
        if board.is_game_over() {
            return None;
        }

        let mut valid_moves = board.get_valid_moves();
        if valid_moves.is_empty() {
            return None;
        }

        // Immediate winning move
        if let Some(winning_col) = self.find_immediate_win(board) {
            return Some(MoveResult {
                column: winning_col,
                moves_to_win: Some(1),
            });
        }

        // Prefer center-first move ordering to improve pruning and play strength
        self.order_moves_center_out(&mut valid_moves);

        let mut best_move = valid_moves[0];
        let mut best_moves_to_win: Option<u8> = None;

        // Determine if AI should maximize or minimize based on current player
        // Red maximizes (seeks positive scores), Yellow minimizes (seeks negative scores)
        let ai_maximizes = board.current_player() == Player::Red;
        let mut best_score = if ai_maximizes { i32::MIN } else { i32::MAX };

        // Root-level losing move avoidance: avoid moves that immediately allow opponent to win
        // Prefer among non-losing moves if any exist
        let mut non_losing_candidates: Vec<usize> = Vec::new();

        for &col in &valid_moves {
            let mut new_board = board.clone();
            new_board.make_move(col);
            // After making the move, it's the opponent's turn, so flip the maximizing flag
            let opponent_has_mate_in_1 = self.find_immediate_win(&new_board).is_some();
            if !opponent_has_mate_in_1 {
                non_losing_candidates.push(col);
            }
        }

        let search_space: Vec<usize> = if !non_losing_candidates.is_empty() {
            non_losing_candidates
        } else {
            valid_moves.clone()
        };

        for &col in &search_space {
            let mut new_board = board.clone();
            new_board.make_move(col);
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

        let mut valid_moves = board.get_valid_moves();
        self.order_moves_center_out(&mut valid_moves);

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

        // Heuristic: potential windows + center column control
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

        // Center column preference
        let center_col = COLS / 2;
        let mut red_center = 0;
        let mut yellow_center = 0;
        for row in 0..ROWS {
            match board.get_cell(row, center_col) {
                Cell::Occupied(Player::Red) => red_center += 1,
                Cell::Occupied(Player::Yellow) => yellow_center += 1,
                _ => {}
            }
        }
        score += 3 * red_center - 3 * yellow_center;

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

    fn find_immediate_win(&self, board: &Board) -> Option<usize> {
        let me = board.current_player();
        for col in board.get_valid_moves() {
            let mut nb = board.clone();
            nb.make_move(col);
            if nb.is_game_over() && nb.winner() == Some(me) {
                return Some(col);
            }
        }
        None
    }

    fn order_moves_center_out(&self, moves: &mut [usize]) {
        let center = COLS as i32 / 2;
        moves.sort_by_key(|&c| (c as i32 - center).abs());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_finds_winning_move_in_one() {
        let mut board = Board::new();
        // Red to move, three in a row horizontally at bottom row, winning at col 3
        // Columns indexed 0..6; place at (row 5, cols 0..2) alternating correctly
        board.make_move(0); // R
        board.make_move(0); // Y
        board.make_move(1); // R
        board.make_move(1); // Y
        board.make_move(2); // R

        let mut solver = Solver::new();
        solver.clear_cache();
        let mv = solver
            .find_best_move(&board, 4)
            .expect("should have a move");
        assert_eq!(mv.column, 3);
    }
}
