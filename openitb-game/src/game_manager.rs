use openitb_engine::{Engine, Board, GameState, Player, Position, Move};
use anyhow::Result;

/// Main game state manager that orchestrates between GUI, TUI, and Engine
/// Implements state machine pattern for turn management
pub struct GameManager {
    /// The game engine that handles logic and move validation
    engine: Engine,
    /// Current game state (whose turn, game over, etc.)
    game_state: GameState,
    /// Currently selected piece position (for UI)
    selected_piece: Option<Position>,
    /// Legal moves for currently selected piece
    legal_moves: Vec<Position>,
    /// Whether we're in TUI debug mode
    debug_mode: bool,
}

impl GameManager {
    /// Create new game manager with the test map
    pub fn new() -> Self {
        // Use the test map from CLAUDE.md (rotated 90° clockwise + horizontal flip for correct orientation)
        let test_map = "G G M M T T G G\n\
                        S G G T G G G T\n\
                        S G G G G W G G\n\
                        S G G G G G G G\n\
                        S G G W G I W G\n\
                        G G G G G T G I\n\
                        G T D D G G G G\n\
                        T T D M M M M T";

        let mut board = Board::from_map_string(test_map);
        
        // Add some test pieces to make it interesting
        // Add a few mechs for the player
        if let Some(pos) = Position::from_chess_notation("b2") {
            board.pieces.insert(pos, openitb_engine::Piece {
                piece_type: openitb_engine::PieceType::Mech,
                player: Player::Human,
            });
        }
        if let Some(pos) = Position::from_chess_notation("c3") {
            board.pieces.insert(pos, openitb_engine::Piece {
                piece_type: openitb_engine::PieceType::Mech,
                player: Player::Human,
            });
        }

        // Add some leapers for the computer
        if let Some(pos) = Position::from_chess_notation("f6") {
            board.pieces.insert(pos, openitb_engine::Piece {
                piece_type: openitb_engine::PieceType::Leaper,
                player: Player::Computer,
            });
        }
        if let Some(pos) = Position::from_chess_notation("h7") {
            board.pieces.insert(pos, openitb_engine::Piece {
                piece_type: openitb_engine::PieceType::Leaper,
                player: Player::Computer,
            });
        }

        let engine = Engine::new(board);

        Self {
            engine,
            game_state: GameState::PlayerTurn,
            selected_piece: None,
            legal_moves: Vec::new(),
            debug_mode: false,
        }
    }

    /// Get current board state
    pub fn get_board(&self) -> &Board {
        self.engine.get_board()
    }

    /// Get current game state
    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    /// Get currently selected piece
    pub fn get_selected_piece(&self) -> Option<Position> {
        self.selected_piece
    }

    /// Get legal moves for selected piece
    pub fn get_legal_moves(&self) -> &[Position] {
        &self.legal_moves
    }

    /// Toggle debug mode
    pub fn toggle_debug_mode(&mut self) {
        self.debug_mode = !self.debug_mode;
    }

    /// Check if in debug mode
    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }

    /// Handle piece selection from UI
    /// Returns true if selection changed
    pub fn select_piece(&mut self, pos: Position) -> bool {
        // Only allow selection during player's turn
        if self.game_state != GameState::PlayerTurn {
            return false;
        }

        // Check if there's a piece at this position
        if let Some(piece) = self.get_board().get_piece(pos) {
            // Only allow selecting human player pieces
            if piece.player == Player::Human {
                // If we're clicking on the already selected piece, deselect it
                if self.selected_piece == Some(pos) {
                    self.selected_piece = None;
                    self.legal_moves.clear();
                    return true;
                } else {
                    // Select the new piece and calculate legal moves
                    self.selected_piece = Some(pos);
                    self.legal_moves = self.engine.get_legal_moves(pos);
                    println!("Selected piece at {}, legal moves: {}", 
                        pos.to_chess_notation(), self.legal_moves.len());
                    return true;
                }
            }
        } else {
            // Clicked on empty square - try to move selected piece there
            if let Some(selected_pos) = self.selected_piece {
                if self.legal_moves.contains(&pos) {
                    // Attempt to make the move
                    if let Err(e) = self.make_move(selected_pos, pos) {
                        println!("Move failed: {}", e);
                    }
                    return true;
                }
            }
            
            // Clear selection if clicking on invalid square
            if self.selected_piece.is_some() {
                self.selected_piece = None;
                self.legal_moves.clear();
                return true;
            }
        }

        false
    }

    /// Make a move and update game state
    fn make_move(&mut self, from: Position, to: Position) -> Result<()> {
        let mov = Move::new(from, to);
        
        // Execute the move through the engine
        self.engine.make_move(mov)?;
        
        println!("Move executed: {} -> {}", from.to_chess_notation(), to.to_chess_notation());
        
        // Clear selection
        self.selected_piece = None;
        self.legal_moves.clear();
        
        // Check if game is over
        if self.engine.is_game_over() {
            self.game_state = GameState::GameOver;
            println!("Game Over!");
        } else {
            // Switch to computer turn
            self.game_state = GameState::ComputerTurn;
            println!("Switching to computer turn");
        }
        
        Ok(())
    }

    /// Process computer turn
    /// Returns true if the game state changed
    pub fn process_computer_turn(&mut self) -> bool {
        if self.game_state != GameState::ComputerTurn {
            return false;
        }

        println!("Processing computer turn...");
        
        // Get all computer pieces and try to move them
        let computer_pieces = self.engine.get_player_pieces(Player::Computer);
        let mut moves_made = 0;
        
        for (pos, _piece) in computer_pieces {
            if let Some(computer_move) = self.simple_ai_move(pos) {
                let move_notation = computer_move.to_notation();
                if let Err(e) = self.engine.make_move(computer_move) {
                    println!("Computer move failed: {}", e);
                } else {
                    println!("Computer moved: {}", move_notation);
                    moves_made += 1;
                }
            }
        }

        println!("Computer made {} moves", moves_made);

        // Check if game is over
        if self.engine.is_game_over() {
            self.game_state = GameState::GameOver;
            println!("Game Over!");
        } else {
            // Switch back to player turn
            self.game_state = GameState::PlayerTurn;
            println!("Switching to player turn");
        }

        true
    }

    /// Simple AI: find a legal move for the piece at the given position
    fn simple_ai_move(&self, pos: Position) -> Option<Move> {
        let legal_moves = self.engine.get_legal_moves(pos);
        if legal_moves.is_empty() {
            return None;
        }

        // Simple strategy: move towards human pieces or towards center
        let board = self.engine.get_board();
        let human_pieces: Vec<Position> = board.pieces.iter()
            .filter(|(_, piece)| piece.player == Player::Human)
            .map(|(&pos, _)| pos)
            .collect();

        if !human_pieces.is_empty() {
            // Find the move that gets closest to a human piece
            let mut best_move = None;
            let mut best_distance = f32::MAX;

            for &target_pos in &legal_moves {
                for &human_pos in &human_pieces {
                    let distance = ((target_pos.file as f32 - human_pos.file as f32).powi(2) + 
                                   (target_pos.rank as f32 - human_pos.rank as f32).powi(2)).sqrt();
                    if distance < best_distance {
                        best_distance = distance;
                        best_move = Some(Move::new(pos, target_pos));
                    }
                }
            }

            best_move
        } else {
            // No human pieces, just move randomly
            Some(Move::new(pos, legal_moves[0]))
        }
    }

    /// Update game logic (called each frame)
    pub fn update(&mut self) {
        // Handle computer turn with a simple timer-based approach
        if self.game_state == GameState::ComputerTurn {
            // In a real implementation, you might want to add a delay
            // For now, process immediately
            self.process_computer_turn();
        }
    }

    /// Get a status message for the UI
    pub fn get_status_message(&self) -> String {
        match self.game_state {
            GameState::PlayerTurn => {
                if self.selected_piece.is_some() {
                    "Player turn - Piece selected, click to move".to_string()
                } else {
                    "Player turn - Select a piece to move".to_string()
                }
            }
            GameState::ComputerTurn => "Computer is thinking...".to_string(),
            GameState::GameOver => "Game Over!".to_string(),
        }
    }
}