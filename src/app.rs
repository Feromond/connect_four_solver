use crate::{Board, COLS, Cell, Player, ROWS, Solver};
use eframe::egui;
use std::time::{Duration, Instant};
use log::{debug, info};

pub struct ConnectFourApp {
    board: Board,
    solver: Solver,
    ai_player: Option<Player>,
    thinking: bool,
    game_mode: GameMode,
    ai_moves_to_win: Option<u8>, // Track if AI has a guaranteed win path
    ai_move_timer: Option<Instant>,
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
            ai_moves_to_win: None,
            ai_move_timer: None,
        }
    }
}

impl eframe::App for ConnectFourApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::Vec2::new(12.0, 8.0);
        style.spacing.item_spacing = egui::Vec2::new(8.0, 8.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(
                    egui::RichText::new("🔴 Connect Four Solver")
                        .size(28.0)
                        .strong(),
                );
                ui.add_space(15.0);

                match self.game_mode {
                    GameMode::Setup => {
                        self.show_setup_screen(ui);
                    }
                    GameMode::Playing => {
                        self.show_game_screen(ui, ctx);
                    }
                }
            });
        });

        self.process_ai_move_with_delay();
        // Only request repaint when needed to reduce CPU usage
        if self.thinking || self.ai_move_timer.is_some() || self.board.is_game_over() {
            ctx.request_repaint();
        }
    }
}

impl ConnectFourApp {
    fn process_ai_move_with_delay(&mut self) {
        if let Some(ai_player) = self.ai_player {
            if self.board.current_player() == ai_player
                && !self.board.is_game_over()
                && !self.thinking
            {
                if self.ai_move_timer.is_none() {
                    self.ai_move_timer = Some(Instant::now());
                }

                if let Some(timer) = self.ai_move_timer {
                    if timer.elapsed() >= Duration::from_millis(300) {
                        self.thinking = true;
                        self.ai_move_timer = None;

                        if let Some(move_result) = self.solver.find_best_move(&self.board, 9) {
                            info!(
                                "AI selects column {}{}",
                                move_result.column,
                                move_result
                                    .moves_to_win
                                    .map(|m| format!("; win in {m} moves"))
                                    .unwrap_or_default()
                            );
                            self.board.make_move(move_result.column);
                            self.ai_moves_to_win = move_result.moves_to_win;
                        }

                        self.thinking = false;
                    }
                }
            } else {
                self.ai_move_timer = None;
            }
        }
    }

    fn show_setup_screen(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(300.0);
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Choose who goes first:")
                        .size(18.0)
                        .strong(),
                );
                ui.add_space(15.0);

                if ui
                    .add_sized(
                        [250.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("🔴 Human goes first (Red)").size(16.0),
                        ),
                    )
                    .clicked()
                {
                    self.ai_player = Some(Player::Yellow);
                    self.game_mode = GameMode::Playing;
                    self.board.reset();
                    self.ai_moves_to_win = None;
                    self.ai_move_timer = None;
                }

                ui.add_space(10.0);

                if ui
                    .add_sized(
                        [250.0, 40.0],
                        egui::Button::new(egui::RichText::new("🤖 AI goes first (Red)").size(16.0)),
                    )
                    .clicked()
                {
                    self.ai_player = Some(Player::Red);
                    self.game_mode = GameMode::Playing;
                    self.board.reset();
                    self.ai_moves_to_win = None;
                    self.ai_move_timer = Some(Instant::now()); // Start timer for AI first move
                }

                ui.add_space(10.0);
            });
        });
    }

    fn show_game_screen(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.group(|ui| {
            ui.set_min_width(400.0);
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                if self.board.is_game_over() {
                    match self.board.winner() {
                        Some(winner) => {
                            let winner_text =
                                format!("🎉 {} WINS! 🎉", winner.to_string().to_uppercase());
                            let color = match winner {
                                Player::Red => egui::Color32::from_rgb(200, 50, 50),
                                Player::Yellow => egui::Color32::from_rgb(200, 150, 50),
                            };
                            ui.label(
                                egui::RichText::new(winner_text)
                                    .size(24.0)
                                    .strong()
                                    .color(color),
                            );
                        }
                        None => {
                            ui.label(
                                egui::RichText::new("🤝 IT'S A DRAW! 🤝")
                                    .size(24.0)
                                    .strong()
                                    .color(egui::Color32::DARK_BLUE),
                            );
                        }
                    }
                } else {
                    let current_player = self.board.current_player();
                    let (emoji, color) = match current_player {
                        Player::Red => ("🔴", egui::Color32::from_rgb(180, 60, 60)),
                        Player::Yellow => ("🟡", egui::Color32::from_rgb(180, 140, 60)),
                    };
                    let status_text = if let Some(ai_player) = self.ai_player {
                        if current_player == ai_player {
                            format!("{} AI's Turn ({})", emoji, current_player.to_string())
                        } else {
                            format!("{} Your Turn ({})", emoji, current_player.to_string())
                        }
                    } else {
                        format!("{} Current Player: {}", emoji, current_player.to_string())
                    };

                    ui.label(
                        egui::RichText::new(status_text)
                            .size(18.0)
                            .strong()
                            .color(color),
                    );

                    if let Some(ai_player) = self.ai_player {
                        if self.board.current_player() == ai_player
                            && (self.thinking || self.ai_move_timer.is_some())
                        {
                            ui.add_space(5.0);
                            ui.label(
                                egui::RichText::new("🤔 AI is thinking...")
                                    .size(14.0)
                                    .color(egui::Color32::GRAY),
                            );
                        }
                    }

                    if let Some(ai_player) = self.ai_player {
                        if let Some(moves_to_win) = self.ai_moves_to_win {
                            if self.board.current_player() == ai_player {
                                // AI has calculated a win and it's AI's turn
                                ui.add_space(8.0);
                                let win_text = format!(
                                    "🧠 AI has found a winning path! Victory in {moves_to_win} moves"
                                );
                                ui.label(
                                    egui::RichText::new(win_text)
                                        .size(16.0)
                                        .strong()
                                        .color(egui::Color32::from_rgb(0, 150, 0)),
                                );
                            } else {
                                // AI has calculated a win but it's human's turn
                                ui.add_space(8.0);
                                let win_text = format!(
                                    "🎯 AI will win in {moves_to_win} moves (barring mistakes)"
                                );
                                ui.label(
                                    egui::RichText::new(win_text)
                                        .size(15.0)
                                        .color(egui::Color32::from_rgb(100, 100, 100)),
                                );
                            }
                        }
                    }
                }
                ui.add_space(5.0);
            });
        });

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 350.0) / 2.0); // Center the board

            let cell_size = 50.0;
            let board_width = COLS as f32 * cell_size;
            let board_height = ROWS as f32 * cell_size;

            let (rect, response) = ui.allocate_exact_size(
                egui::Vec2::new(board_width, board_height),
                egui::Sense::click(),
            );
            let painter = ui.painter();

            painter.rect_filled(rect, 8.0, egui::Color32::from_rgb(41, 98, 255));

            let inner_rect = rect.shrink(2.0);
            painter.rect_filled(inner_rect, 6.0, egui::Color32::from_rgb(35, 85, 220));

            for row in 0..ROWS {
                for col in 0..COLS {
                    let cell_rect = egui::Rect::from_min_size(
                        rect.min + egui::Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                        egui::Vec2::splat(cell_size),
                    );

                    let (color, stroke) = match self.board.get_cell(row, col) {
                        Cell::Empty => (
                            egui::Color32::WHITE,
                            egui::Stroke::new(2.0, egui::Color32::LIGHT_GRAY),
                        ),
                        Cell::Occupied(Player::Red) => (
                            egui::Color32::from_rgb(220, 50, 50),
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(180, 40, 40)),
                        ),
                        Cell::Occupied(Player::Yellow) => (
                            egui::Color32::from_rgb(255, 215, 50),
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 160, 40)),
                        ),
                    };

                    let radius = cell_size * 0.35;
                    painter.circle_filled(cell_rect.center(), radius, color);
                    painter.circle_stroke(cell_rect.center(), radius, stroke);
                }
            }

            if response.clicked() {
                if let Some(ai_player) = self.ai_player {
                    if self.board.current_player() != ai_player
                        && !self.board.is_game_over()
                        && !self.thinking
                        && self.ai_move_timer.is_none()
                    {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let relative_pos = pos - rect.min;
                            let col = (relative_pos.x / cell_size) as usize;

                            if col < COLS && self.board.is_valid_move(col) {
                                debug!("Human plays column {}", col);
                                self.board.make_move(col);
                                self.ai_moves_to_win = None;
                                self.ai_move_timer = Some(Instant::now());
                            }
                        }
                    }
                }
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 300.0) / 2.0); // Center the buttons

            if ui
                .add_sized(
                    [140.0, 35.0],
                    egui::Button::new(egui::RichText::new("🆕 New Game").size(14.0)),
                )
                .clicked()
            {
                self.game_mode = GameMode::Setup;
                self.board.reset();
                self.ai_player = None;
                self.ai_moves_to_win = None;
                self.ai_move_timer = None;
            }

            ui.add_space(20.0);

            if ui
                .add_sized(
                    [140.0, 35.0],
                    egui::Button::new(egui::RichText::new("🔄 Reset Board").size(14.0)),
                )
                .clicked()
            {
                self.board.reset();
                self.ai_moves_to_win = None;
                self.ai_move_timer = None;
            }
        });

        if self.board.is_game_over() {
            self.show_game_over_overlay(ui, ctx);
        }
    }

    fn show_game_over_overlay(&self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.add_space(15.0);

        ui.group(|ui| {
            ui.set_min_width(350.0);
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);

                match self.board.winner() {
                    Some(winner) => {
                        let message = format!("Game Over - {} is the winner!", winner.to_string());
                        ui.label(
                            egui::RichText::new(message)
                                .size(16.0)
                                .color(egui::Color32::DARK_GREEN),
                        );
                    }
                    None => {
                        ui.label(
                            egui::RichText::new("Game Over - It's a draw!")
                                .size(16.0)
                                .color(egui::Color32::DARK_BLUE),
                        );
                    }
                }

                ui.label(
                    egui::RichText::new("Click 'New Game' to play again")
                        .size(12.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(10.0);
            });
        });
    }
}
