use egui_macroquad::egui::{self, Color32, RichText, Ui, Vec2};
use openitb_engine::{Player, Position, Unit, PieceType, GameState};
use crate::GameManager;

/// Main egui panels for unit information and debug data
pub struct EguiPanels {
    show_panels: bool,
}

impl EguiPanels {
    pub fn new() -> Self {
        Self {
            show_panels: true,
        }
    }

    /// Render all egui panels
    pub fn render(&mut self, ctx: &egui::Context, game_manager: &GameManager) {
        if !self.show_panels {
            return;
        }

        self.render_mech_panel(ctx, game_manager);
        self.render_leaper_panel(ctx, game_manager);
        self.render_debug_panel(ctx, game_manager);
    }

    /// Toggle panel visibility
    pub fn toggle_visibility(&mut self) {
        self.show_panels = !self.show_panels;
    }

    /// Render mech units panel
    fn render_mech_panel(&mut self, ctx: &egui::Context, game_manager: &GameManager) {
        egui::Window::new("Mechs")
            .default_pos([10.0, 10.0])
            .default_size([200.0, 150.0])
            .resizable(false)
            .show(ctx, |ui| {
                let human_pieces = game_manager.get_engine().get_player_pieces(Player::Human);
                let mechs: Vec<_> = human_pieces
                    .iter()
                    .filter(|(_, unit)| unit.piece_type == PieceType::Mech)
                    .collect();

                if mechs.is_empty() {
                    ui.label("No mechs remaining");
                    return;
                }

                for (pos, unit) in mechs {
                    self.render_unit_info(ui, *pos, unit);
                }
            });
    }

    /// Render leaper units panel
    fn render_leaper_panel(&mut self, ctx: &egui::Context, game_manager: &GameManager) {
        egui::Window::new("Leapers")
            .default_pos([10.0, 180.0])
            .default_size([200.0, 150.0])
            .resizable(false)
            .show(ctx, |ui| {
                let computer_pieces = game_manager.get_engine().get_player_pieces(Player::Computer);
                let leapers: Vec<_> = computer_pieces
                    .iter()
                    .filter(|(_, unit)| unit.piece_type == PieceType::Leaper)
                    .collect();

                if leapers.is_empty() {
                    ui.label("No leapers remaining");
                    return;
                }

                for (pos, unit) in leapers {
                    self.render_unit_info(ui, *pos, unit);
                }
            });
    }

    /// Render debug information panel
    fn render_debug_panel(&mut self, ctx: &egui::Context, game_manager: &GameManager) {
        egui::Window::new("Debug Info")
            .default_pos([10.0, 350.0])
            .default_size([200.0, 200.0])
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!("Turn: {}", match game_manager.get_game_state() {
                    GameState::PlayerTurn => "Player",
                    GameState::ComputerTurn => "Computer",
                    GameState::ComputerThinking => "Computer (Thinking)",
                    GameState::ComputerAnimating => "Computer (Animating)",
                    GameState::GameOver => "Game Over",
                }));

                if let Some(selected) = game_manager.get_selected_piece() {
                    ui.label(format!("Selected: {}", selected.to_chess_notation()));
                } else {
                    ui.label("Selected: None");
                }

                let legal_moves = game_manager.get_legal_moves();
                ui.label(format!("Legal moves: {}", legal_moves.len()));

                if let Some(cursor) = game_manager.get_cursor_position() {
                    ui.label(format!("Cursor: {}", cursor.to_chess_notation()));
                }

                ui.separator();

                // Game statistics
                let human_pieces = game_manager.get_engine().get_player_pieces(Player::Human);
                let computer_pieces = game_manager.get_engine().get_player_pieces(Player::Computer);
                
                ui.label(format!("Human units: {}", human_pieces.len()));
                ui.label(format!("Computer units: {}", computer_pieces.len()));

                // Total action tokens remaining for human player
                let human_actions: u32 = human_pieces
                    .iter()
                    .map(|(_, unit)| unit.action_tokens_left as u32)
                    .sum();
                ui.label(format!("Human actions left: {}", human_actions));

                ui.separator();
                ui.label("Controls:");
                ui.label("• Click: Select/Move");
                ui.label("• Space: Toggle Debug");
                ui.label("• Tab: Toggle Panels");
            });
    }

    /// Render individual unit information
    fn render_unit_info(&self, ui: &mut Ui, pos: Position, unit: &Unit) {
        ui.horizontal(|ui| {
            // 16x16 pixel placeholder portrait (colored rectangle)
            let portrait_size = Vec2::new(16.0, 16.0);
            let portrait_color = match unit.piece_type {
                PieceType::Mech => Color32::from_rgb(0, 100, 200), // Blue for mechs
                PieceType::Leaper => Color32::from_rgb(200, 100, 0), // Orange for leapers
            };

            let (rect, _response) = ui.allocate_exact_size(portrait_size, egui::Sense::hover());
            ui.painter().rect_filled(rect, 2.0, portrait_color);

            ui.vertical(|ui| {
                // Position
                ui.label(format!("@{}", pos.to_chess_notation()));
                
                // HP - red if critical (1/n)
                let hp_text = format!("{}/{}", unit.current_hit_points, unit.max_hit_points);
                let hp_color = if unit.current_hit_points == 1 && unit.max_hit_points > 1 {
                    Color32::from_rgb(200, 50, 50) // Red for critical health
                } else {
                    Color32::from_gray(200) // Normal color
                };
                ui.label(RichText::new(hp_text).color(hp_color));

                // Action tokens
                ui.label(format!("Actions: {}/{}", unit.action_tokens_left, unit.max_action_tokens));

                // Primary weapon
                ui.label(format!("Weapon: {:?}", unit.primary_weapon));

                // Status effects (if any)
                if !unit.special_effects.is_empty() {
                    ui.label(format!("Effects: {:?}", unit.special_effects));
                }
            });
        });
        ui.separator();
    }
}