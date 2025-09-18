use macroquad::prelude::*;
use egui_macroquad;
use openitb_engine::GameState;

mod gui;
mod tui;
mod game_manager;

use gui::IsometricRenderer;
use game_manager::GameManager;

/// Main application state
enum AppState {
    /// Normal GUI mode with macroquad
    GuiMode,
    /// Debug TUI mode with ratatui (currently not switchable at runtime)
    TuiMode,
}

/// Macroquad window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "OpenITB - Into the Breach Clone".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting OpenITB - Into the Breach Clone");
    println!("=== Game Architecture ===");
    println!("- Engine: Handles game logic and move validation (UCI-like protocol)");
    println!("- TUI: Debug interface using ratatui (toggleable layers)");
    println!("- GUI: Visual interface using macroquad with isometric tiles");
    println!("- State Machine: Manages player/computer turns");
    println!("=========================");

    // Check if we should run in TUI-only debug mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--tui" {
        println!("Running in TUI debug mode");
        run_tui_mode().await?;
        return Ok(());
    }

    // Initialize game components
    println!("Initializing game manager...");
    let mut game_manager = GameManager::new();
    
    println!("Loading isometric renderer...");
    let mut renderer = IsometricRenderer::new().await;
    
    let app_state = AppState::GuiMode;
    
    println!("Starting main game loop...");
    
    // Main game loop
    loop {
        match app_state {
            AppState::GuiMode => {
                // Handle input
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
                
                // Toggle debug mode (but stay in GUI)
                if is_key_pressed(KeyCode::Space) {
                    game_manager.toggle_debug_mode();
                    println!("Debug mode: {}", if game_manager.is_debug_mode() { "ON" } else { "OFF" });
                }

                // Handle mouse input for piece selection
                if let Some(clicked_pos) = renderer.handle_mouse_input() {
                    println!("Clicked on board position: {}", clicked_pos.to_chess_notation());
                    game_manager.select_piece(clicked_pos);
                }

                // Update game logic
                game_manager.update();
                
                // Update computer animation if active
                game_manager.update_computer_animation();

                // Update renderer with current selection
                renderer.set_selection(
                    game_manager.get_selected_piece(),
                    game_manager.get_legal_moves().to_vec(),
                    game_manager.get_cursor_position()
                );
                
                // Update renderer with computer animation highlights
                renderer.set_animation_highlights(
                    game_manager.get_animation_selected_piece(),
                    game_manager.get_animation_legal_moves(),
                    game_manager.get_animation_attack_positions(),
                    game_manager.get_animation_attack_target()
                );

                // Render the game
                renderer.render(game_manager.get_board());

                // Draw status information
                draw_status_overlay(&game_manager);

                // Draw debug overlay if enabled
                if game_manager.is_debug_mode() {
                    draw_debug_overlay(&game_manager);
                }

                // Draw end turn dialog if needed
                draw_end_turn_dialog(&mut game_manager);
            }
            AppState::TuiMode => {
                // This branch is not currently reachable at runtime
                // TUI mode is only available via command line argument
                break;
            }
        }

        next_frame().await;
    }

    println!("Exiting OpenITB");
    Ok(())
}

/// Run the TUI-only debug mode
async fn run_tui_mode() -> Result<(), Box<dyn std::error::Error>> {
    let game_manager = GameManager::new();
    tui::run_debug_tui(game_manager.get_board().clone())?;
    Ok(())
}

/// Draw status overlay on the GUI
fn draw_status_overlay(game_manager: &GameManager) {
    let status_msg = game_manager.get_status_message();
    draw_text(&status_msg, 10.0, 60.0, 24.0, WHITE);

    // Show current turn
    let turn_msg = match game_manager.get_game_state() {
        GameState::PlayerTurn => "PLAYER TURN",
        GameState::ComputerTurn => "COMPUTER TURN",
        GameState::ComputerThinking => "COMPUTER THINKING",
        GameState::ComputerAnimating => "COMPUTER ANIMATING",
        GameState::GameOver => "GAME OVER",
    };
    
    let turn_color = match game_manager.get_game_state() {
        GameState::PlayerTurn => BLUE,
        GameState::ComputerTurn => RED,
        GameState::ComputerThinking => ORANGE,
        GameState::ComputerAnimating => PURPLE,
        GameState::GameOver => YELLOW,
    };
    
    draw_text(turn_msg, 10.0, 90.0, 20.0, turn_color);
}

/// Draw debug information overlay
fn draw_debug_overlay(game_manager: &GameManager) {
    let debug_y_start = 120.0;
    let line_height = 18.0;
    let mut y = debug_y_start;

    draw_text("=== DEBUG INFO ===", 10.0, y, 16.0, YELLOW);
    y += line_height;

    let board = game_manager.get_board();
    
    // Count pieces
    let human_pieces = board.pieces.iter()
        .filter(|(_, piece)| piece.player == openitb_engine::Player::Human)
        .count();
    let computer_pieces = board.pieces.iter()
        .filter(|(_, piece)| piece.player == openitb_engine::Player::Computer)
        .count();
    
    draw_text(&format!("Human pieces: {}", human_pieces), 10.0, y, 14.0, LIGHTGRAY);
    y += line_height;
    draw_text(&format!("Computer pieces: {}", computer_pieces), 10.0, y, 14.0, LIGHTGRAY);
    y += line_height;

    if let Some(selected) = game_manager.get_selected_piece() {
        draw_text(&format!("Selected: {}", selected.to_chess_notation()), 10.0, y, 14.0, LIGHTGRAY);
        y += line_height;
        draw_text(&format!("Legal moves: {}", game_manager.get_legal_moves().len()), 10.0, y, 14.0, LIGHTGRAY);
        y += line_height;

        // Show legal moves
        if !game_manager.get_legal_moves().is_empty() {
            let moves_str: Vec<String> = game_manager.get_legal_moves()
                .iter()
                .map(|pos| pos.to_chess_notation())
                .collect();
            let moves_text = moves_str.join(", ");
            draw_text(&format!("Moves: {}", moves_text), 10.0, y, 12.0, LIGHTGRAY);
        }
    }

    // Instructions
    y = screen_height() - 100.0;
    draw_text("Controls:", 10.0, y, 14.0, YELLOW);
    y += line_height;
    draw_text("Left click: Select/move piece", 10.0, y, 12.0, LIGHTGRAY);
    y += line_height;
    draw_text("Space: Toggle debug overlay", 10.0, y, 12.0, LIGHTGRAY);
    y += line_height;
    y += line_height;
    draw_text("ESC: Quit", 10.0, y, 12.0, LIGHTGRAY);
}

/// Draw end turn dialog when player has no legal moves
fn draw_end_turn_dialog(game_manager: &mut GameManager) {
    use egui_macroquad::egui;

    // Only show dialog during player turn when no legal moves remain
    if game_manager.get_game_state() == GameState::PlayerTurn && !game_manager.player_has_legal_moves() {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("No Legal Moves")
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label("No legal moves remain for your units.");
                        ui.add_space(10.0);
                        ui.label("End your turn?");
                        ui.add_space(15.0);
                        
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                game_manager.end_player_turn();
                            }
                            ui.add_space(20.0);
                            if ui.button("No").clicked() {
                                // Just close dialog, let player continue looking
                            }
                        });
                        ui.add_space(10.0);
                    });
                });
        });
        
        egui_macroquad::draw();
    }
}