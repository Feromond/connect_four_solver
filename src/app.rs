use eframe::egui;
use crate::{Board, Solver, Player, Cell, ROWS, COLS};

pub struct ConnectFourApp {
    board: Board,
    solver: Solver,
    ai_player: Option<Player>,
    thinking: bool,
    game_mode: GameMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    Setup,
    Playing,
}

impl Default for ConnectFourApp {
    fn default() -> Self {
        Self {
            board: Board::new(),
            solver: Solver::new(),
            ai_player: None,
            thinking: false,
            game_mode: GameMode::Setup,
        }
    }
}

impl eframe::App for ConnectFourApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Connect Four Solver");

            match self.game_mode {
                GameMode::Setup => {
                    ui.separator();
                    ui.label("Choose who goes first:");
                    
                    ui.horizontal(|ui| {
                        if ui.button("Human goes first (Red)").clicked() {
                            self.ai_player = Some(Player::Yellow);
                            self.game_mode = GameMode::Playing;
                            self.board.reset();
                        }
                        
                        if ui.button("AI goes first (Red)").clicked() {
                            self.ai_player = Some(Player::Red);
                            self.game_mode = GameMode::Playing;
                            self.board.reset();
                        }
                    });
                }
                GameMode::Playing => {
                    // Game status
                    ui.horizontal(|ui| {
                        ui.label(format!("Current Player: {}", self.board.current_player().to_string()));
                        
                        if let Some(ai_player) = self.ai_player {
                            if self.board.current_player() == ai_player && !self.board.is_game_over() {
                                ui.label("AI is thinking...");
                            }
                        }
                    });

                    if self.board.is_game_over() {
                        match self.board.winner() {
                            Some(winner) => ui.label(format!("{} wins!", winner.to_string())),
                            None => ui.label("It's a draw!"),
                        };
                    }

                    ui.separator();

                    // Game board
                    let cell_size = 50.0;
                    let board_width = COLS as f32 * cell_size;
                    let board_height = ROWS as f32 * cell_size;

                    let (rect, response) = ui.allocate_exact_size(
                        egui::Vec2::new(board_width, board_height),
                        egui::Sense::click()
                    );

                    // Draw the board
                    let painter = ui.painter();
                    
                    // Background
                    painter.rect_filled(rect, 5.0, egui::Color32::BLUE);

                    // Draw cells
                    for row in 0..ROWS {
                        for col in 0..COLS {
                            let cell_rect = egui::Rect::from_min_size(
                                rect.min + egui::Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                                egui::Vec2::splat(cell_size)
                            );

                            let color = match self.board.get_cell(row, col) {
                                Cell::Empty => egui::Color32::WHITE,
                                Cell::Occupied(Player::Red) => egui::Color32::RED,
                                Cell::Occupied(Player::Yellow) => egui::Color32::YELLOW,
                            };

                            painter.circle_filled(
                                cell_rect.center(),
                                cell_size * 0.4,
                                color
                            );
                        }
                    }

                    // Handle clicks for human moves
                    if response.clicked() {
                        if let Some(ai_player) = self.ai_player {
                            if self.board.current_player() != ai_player && !self.board.is_game_over() {
                                if let Some(pos) = response.interact_pointer_pos() {
                                    let relative_pos = pos - rect.min;
                                    let col = (relative_pos.x / cell_size) as usize;
                                    
                                    if col < COLS && self.board.is_valid_move(col) {
                                        self.board.make_move(col);
                                    }
                                }
                            }
                        }
                    }

                    ui.separator();

                    // Control buttons
                    ui.horizontal(|ui| {
                        if ui.button("New Game").clicked() {
                            self.game_mode = GameMode::Setup;
                            self.board.reset();
                            self.ai_player = None;
                        }

                        if ui.button("Reset Board").clicked() {
                            self.board.reset();
                        }
                    });
                }
            }
        });

        // AI move logic
        if let Some(ai_player) = self.ai_player {
            if self.board.current_player() == ai_player && !self.board.is_game_over() && !self.thinking {
                self.thinking = true;
                
                // Find best move in background
                if let Some(best_col) = self.solver.find_best_move(&self.board, 7) {
                    self.board.make_move(best_col);
                }
                
                self.thinking = false;
            }
        }

        // Request repaint for smooth AI moves
        ctx.request_repaint();
    }
} 