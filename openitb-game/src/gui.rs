use macroquad::prelude::*;
use openitb_engine::{Board, Position, Player, PieceType, Terrain};
use std::collections::HashMap;

/// Isometric tile renderer for the game
pub struct IsometricRenderer {
    /// Tile textures loaded from assets
    tile_textures: HashMap<String, Texture2D>,
    /// Tile dimensions
    tile_width: f32,
    tile_height: f32,
    /// Board offset for centering
    board_offset: Vec2,
    /// Currently selected piece position
    selected_piece: Option<Position>,
    /// Legal moves for selected piece
    legal_moves: Vec<Position>,
    /// Current cursor position (last clicked tile)
    cursor_position: Option<Position>,
    /// Scale factor for rendering
    scale: f32,
}

impl IsometricRenderer {
    pub async fn new() -> Self {
        let mut tile_textures = HashMap::new();
        let scale = 1.0;

        // Load tile textures - fallback to PNG if GIF not supported
        let texture_map = [
            ("ground", "assets/tiles/ground.png"),
            ("double", "assets/tiles/double.png"),
            ("trees", "assets/tiles/trees.png"),
            ("mountain", "assets/tiles/mountain.png"),
            ("water", "assets/tiles/water.png"),
            ("mech", "assets/tiles/mech.png"),
            ("leaper", "assets/tiles/leaper.png"),
            ("black_tile", "assets/tiles/black_tile.png"),
            ("white_tile", "assets/tiles/white_tile.png"),
            ("green_highlight", "assets/tiles/green_highlight.png"),
            ("yellow_cursor", "assets/tiles/yellow_cursor.png"),
        ];

        for (name, path) in texture_map {
            match load_texture(path).await {
                Ok(texture) => {
                    // Set texture filter to nearest for pixel-perfect rendering
                    texture.set_filter(FilterMode::Nearest);
                    tile_textures.insert(name.to_string(), texture);
                    println!("Loaded texture: {} from {}", name, path);
                }
                Err(e) => {
                    println!("Failed to load texture {}: {:?}", path, e);
                    // Create a fallback colored texture based on tile type
                    let color = match name {
                        "ground" => [34, 139, 34, 255],    // Forest green
                        "double" => [128, 128, 128, 255],  // Gray
                        "trees" => [0, 100, 0, 255],       // Dark green
                        "mountain" => [139, 69, 19, 255],  // Brown
                        "water" => [0, 0, 255, 255],       // Blue
                        "mech" => [0, 191, 255, 255],      // Deep sky blue
                        "leaper" => [255, 69, 0, 255],     // Red orange
                        "green_highlight" => [0, 255, 0, 100], // Semi-transparent green
                        "yellow_cursor" => [255, 255, 0, 150], // Semi-transparent yellow
                        _ => [255, 0, 255, 255],           // Magenta fallback
                    };
                    let texture_data = vec![color; 120 * 60]; // 120x60 pixels
                    let mut texture_bytes = Vec::with_capacity(120 * 60 * 4);
                    for pixel in texture_data {
                        texture_bytes.extend_from_slice(&pixel);
                    }
                    let fallback_texture = Texture2D::from_rgba8(120, 60, &texture_bytes);
                    fallback_texture.set_filter(FilterMode::Nearest);
                    tile_textures.insert(name.to_string(), fallback_texture);
                }
            }
        }

        // Standard isometric tile dimensions (as specified in CLAUDE.md)
        let tile_width = 120.0 * scale;
        let tile_height = 60.0 * scale;

        // Calculate board offset to center the 8x8 board
        let screen_width = screen_width();
        let screen_height = screen_height();
        let board_pixel_width = tile_width * 8.0;
        let board_pixel_height = tile_height * 8.0;
        
        let board_offset = Vec2::new(
            (screen_width - board_pixel_width) * 0.5,
            (screen_height - board_pixel_height) * 0.5,
        );

        Self {
            tile_textures,
            tile_width,
            tile_height,
            board_offset,
            selected_piece: None,
            legal_moves: Vec::new(),
            cursor_position: None,
            scale,
        }
    }

    /// Convert board coordinates to screen position (isometric projection)
    fn board_to_screen(&self, pos: Position) -> Vec2 {
        // Standard isometric projection for a diamond-shaped board
        // For proper orientation to match the expected output:
        let board_x = pos.file as f32;
        let board_y = pos.rank as f32;
        
        // Isometric transformation: diamond pattern
        let screen_x = (board_x - board_y) * self.tile_width * 0.5;
        let screen_y = (board_x + board_y) * self.tile_height * 0.5;
        
        Vec2::new(
            self.board_offset.x + screen_x + self.tile_width * 4.0, // Center horizontally
            self.board_offset.y + screen_y + self.tile_height,      // Center vertically
        )
    }

    /// Convert screen position to board coordinates with proper diamond-shaped hit detection
    fn screen_to_board(&self, screen_pos: Vec2) -> Option<Position> {
        println!("Raw screen click: ({:.1}, {:.1})", screen_pos.x, screen_pos.y);
        
        // First, get the approximate tile using rectangular coordinate system
        let relative_x = screen_pos.x - self.board_offset.x - self.tile_width * 4.0;
        let relative_y = screen_pos.y - self.board_offset.y - self.tile_height;
        
        let u = relative_x / (self.tile_width * 0.5);
        let v = relative_y / (self.tile_height * 0.5);
        
        let board_x = (u + v) * 0.5;
        let board_y = (v - u) * 0.5;
        
        // Apply offset correction
        let corrected_x = board_x - 0.5;
        let corrected_y = board_y - 0.0;
        
        println!("Approximate board coordinates: ({:.2}, {:.2})", corrected_x, corrected_y);
        
        // Get the candidate tile and its neighbors
        let base_file = corrected_x.floor() as i32;
        let base_rank = corrected_y.floor() as i32;
        
        // Check the candidate tile and its neighbors for diamond-shaped hit detection
        let candidates = vec![
            (base_file, base_rank),
            (base_file + 1, base_rank),
            (base_file, base_rank + 1),
            (base_file + 1, base_rank + 1),
        ];
        
        println!("Checking candidates: {:?}", candidates);
        
        // Find the best match using diamond-shaped distance
        let mut best_match: Option<(i32, i32)> = None;
        let mut min_distance = f32::INFINITY;
        
        for &(file, rank) in &candidates {
            if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
                let tile_center = self.board_to_screen(Position::new(file as u8, rank as u8).unwrap());
                let tile_center_x = tile_center.x + self.tile_width * 0.5;
                let tile_center_y = tile_center.y + self.tile_height * 0.5;
                
                // Check if click is within diamond bounds using isometric distance
                let dx = (screen_pos.x - tile_center_x) / (self.tile_width * 0.5);
                let dy = (screen_pos.y - tile_center_y) / (self.tile_height * 0.5);
                
                // Diamond-shaped boundary check: |dx| + |dy| <= 1
                let diamond_distance = dx.abs() + dy.abs();
                
                println!("Tile ({}, {}): center=({:.1}, {:.1}), diamond_distance={:.2}", 
                    file, rank, tile_center_x, tile_center_y, diamond_distance);
                
                if diamond_distance <= 1.0 && diamond_distance < min_distance {
                    min_distance = diamond_distance;
                    best_match = Some((file, rank));
                }
            }
        }
        
        if let Some((file, rank)) = best_match {
            println!("Selected: file={}, rank={} (diamond_distance={:.2})", file, rank, min_distance);
            let pos = Position::new(file as u8, rank as u8).unwrap();
            let back_to_screen = self.board_to_screen(pos);
            println!("Round-trip: {} -> ({:.1}, {:.1})", pos.to_chess_notation(), back_to_screen.x, back_to_screen.y);
            Some(pos)
        } else {
            println!("Click outside any diamond tile bounds");
            None
        }
    }

    /// Set selected piece, legal moves, and cursor position
    pub fn set_selection(&mut self, piece_pos: Option<Position>, legal_moves: Vec<Position>, cursor_pos: Option<Position>) {
        self.selected_piece = piece_pos;
        self.legal_moves = legal_moves;
        self.cursor_position = cursor_pos;
    }

    /// Handle mouse input and return clicked position
    pub fn handle_mouse_input(&self) -> Option<Position> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
            self.screen_to_board(mouse_pos)
        } else {
            None
        }
    }

    /// Render the entire board
    pub fn render(&self, board: &Board) {
        clear_background(Color::new(0.2, 0.3, 0.2, 1.0)); // Dark green background

        // Render in layers: terrain -> pieces -> highlights (on top)
        self.render_terrain_layer(board);
        self.render_piece_layer(board);
        self.render_highlight_layer(); // Draw highlights on top of everything
        self.render_ui_overlay(board);
    }

    fn render_terrain_layer(&self, board: &Board) {
        // Render from back to front for proper isometric depth sorting
        // Back = high rank + high file, Front = low rank + low file
        for rank in (0..8).rev() {
            for file in (0..8).rev() {
                let pos = Position::new(file, rank).unwrap();
                let screen_pos = self.board_to_screen(pos);
                let terrain = board.get_terrain(pos);
                
                // Get the appropriate texture for this terrain
                let texture_name = match terrain {
                    Terrain::Grass => "ground",
                    Terrain::SingleBuilding | Terrain::DoubleBuilding => "double",
                    Terrain::Forest => "trees",
                    Terrain::Mountain => "mountain",
                    Terrain::Water => "water",
                    Terrain::MonsterIngress => "ground", // Special handling later
                    Terrain::PowerGenerator => "ground",
                    Terrain::Landmine => "ground",
                    Terrain::Rocket => "ground",
                };

                if let Some(texture) = self.tile_textures.get(texture_name) {
                    draw_texture_ex(
                        texture,
                        screen_pos.x,
                        screen_pos.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(self.tile_width, self.tile_height)),
                            ..Default::default()
                        },
                    );
                } else {
                    // Only draw fallback rectangles for non-grass terrain
                    // Grass is the base layer and shouldn't cover other terrain
                    match terrain {
                        Terrain::Grass => {
                            // Don't draw anything for grass - let the background show through
                        }
                        Terrain::Forest => {
                            draw_rectangle(screen_pos.x, screen_pos.y, self.tile_width, self.tile_height, DARKGREEN);
                        }
                        Terrain::Mountain => {
                            draw_rectangle(screen_pos.x, screen_pos.y, self.tile_width, self.tile_height, BROWN);
                        }
                        Terrain::Water => {
                            draw_rectangle(screen_pos.x, screen_pos.y, self.tile_width, self.tile_height, BLUE);
                        }
                        _ => {
                            draw_rectangle(screen_pos.x, screen_pos.y, self.tile_width, self.tile_height, GRAY);
                        }
                    }
                }

                // Special terrain indicators
                if matches!(terrain, Terrain::MonsterIngress) {
                    draw_circle(
                        screen_pos.x + self.tile_width * 0.5,
                        screen_pos.y + self.tile_height * 0.5,
                        10.0,
                        RED,
                    );
                }
            }
        }
    }

    fn render_highlight_layer(&self) {
        // Highlight selected piece
        if let Some(selected_pos) = self.selected_piece {
            let screen_pos = self.board_to_screen(selected_pos);
            if let Some(texture) = self.tile_textures.get("yellow_cursor") {
                draw_texture_ex(
                    texture,
                    screen_pos.x,
                    screen_pos.y,
                    Color::new(1.0, 1.0, 0.0, 0.8), // Semi-transparent yellow
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(self.tile_width, self.tile_height)),
                        ..Default::default()
                    },
                );
            } else {
                // Fallback: yellow rectangle
                draw_rectangle(
                    screen_pos.x,
                    screen_pos.y,
                    self.tile_width,
                    self.tile_height,
                    Color::new(1.0, 1.0, 0.0, 0.5),
                );
            }
        }

        // Highlight legal moves with green
        for &move_pos in &self.legal_moves {
            let screen_pos = self.board_to_screen(move_pos);
            if let Some(texture) = self.tile_textures.get("green_highlight") {
                draw_texture_ex(
                    texture,
                    screen_pos.x,
                    screen_pos.y,
                    Color::new(0.0, 1.0, 0.0, 0.9), // Bright green, more opaque
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(self.tile_width, self.tile_height)),
                        ..Default::default()
                    },
                );
            } else {
                // Fallback: bright green rectangle
                draw_rectangle(
                    screen_pos.x,
                    screen_pos.y,
                    self.tile_width,
                    self.tile_height,
                    Color::new(0.0, 1.0, 0.0, 0.7), // Bright green
                );
            }
        }

        // Render cursor if present
        if let Some(cursor_pos) = self.cursor_position {
            let screen_pos = self.board_to_screen(cursor_pos);
            if let Some(texture) = self.tile_textures.get("yellow_cursor") {
                draw_texture_ex(
                    texture,
                    screen_pos.x,
                    screen_pos.y,
                    Color::new(1.0, 1.0, 0.0, 0.8), // Semi-transparent yellow
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(self.tile_width, self.tile_height)),
                        ..Default::default()
                    },
                );
            } else {
                // Fallback: yellow rectangle with border
                draw_rectangle_lines(
                    screen_pos.x,
                    screen_pos.y,
                    self.tile_width,
                    self.tile_height,
                    3.0,
                    Color::new(1.0, 1.0, 0.0, 0.8),
                );
            }
        }
    }

    fn render_piece_layer(&self, board: &Board) {
        // Render pieces from back to front for proper depth sorting
        for rank in (0..8).rev() {
            for file in (0..8).rev() {
                let pos = Position::new(file, rank).unwrap();
                if let Some(piece) = board.get_piece(pos) {
                    let screen_pos = self.board_to_screen(pos);
                    
                    let texture_name = match piece.piece_type {
                        PieceType::Mech => "mech",
                        PieceType::Leaper => "leaper",
                    };

                    // Color based on player
                    let tint = match piece.player {
                        Player::Human => Color::new(0.7, 0.7, 1.0, 1.0),    // Light blue tint for human
                        Player::Computer => Color::new(1.0, 0.7, 0.7, 1.0), // Light red tint for computer
                    };

                    if let Some(texture) = self.tile_textures.get(texture_name) {
                        draw_texture_ex(
                            texture,
                            screen_pos.x,
                            screen_pos.y,
                            tint,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(self.tile_width, self.tile_height)),
                                ..Default::default()
                            },
                        );
                    } else {
                        // Fallback: colored circle
                        let color = match (piece.piece_type, piece.player) {
                            (PieceType::Mech, Player::Human) => BLUE,
                            (PieceType::Mech, Player::Computer) => RED,
                            (PieceType::Leaper, Player::Human) => SKYBLUE,
                            (PieceType::Leaper, Player::Computer) => PINK,
                        };
                        draw_circle(
                            screen_pos.x + self.tile_width * 0.5,
                            screen_pos.y + self.tile_height * 0.5,
                            15.0,
                            color,
                        );
                    }
                }
            }
        }
    }

    fn render_ui_overlay(&self, board: &Board) {
        // Simple UI info at the top
        let info_text = if let Some(selected_pos) = self.selected_piece {
            if let Some(piece) = board.get_piece(selected_pos) {
                format!(
                    "Selected: {} {:?} at {} | Legal moves: {}",
                    match piece.player {
                        Player::Human => "Player",
                        Player::Computer => "Computer",
                    },
                    piece.piece_type,
                    selected_pos.to_chess_notation(),
                    self.legal_moves.len()
                )
            } else {
                format!("Selected: {}", selected_pos.to_chess_notation())
            }
        } else {
            "Click on a piece to select it".to_string()
        };

        draw_text(&info_text, 10.0, 30.0, 20.0, WHITE);
        
        // Instructions
        draw_text("Left click: Select piece", 10.0, screen_height() - 60.0, 16.0, LIGHTGRAY);
        draw_text("ESC: Quit", 10.0, screen_height() - 40.0, 16.0, LIGHTGRAY);
        draw_text("Space: Switch to TUI debug view", 10.0, screen_height() - 20.0, 16.0, LIGHTGRAY);
    }
}