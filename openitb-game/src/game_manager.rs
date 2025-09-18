use openitb_engine::{Engine, Board, GameState, Player, Position, Move};
use anyhow::Result;
use std::time::Instant;

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
    /// Current cursor position (last clicked tile) - useful for gamepad support
    cursor_position: Option<Position>,
    /// Whether we're in TUI debug mode
    debug_mode: bool,
    /// Whether to show coordinate labels on the board
    show_coordinates: bool,
    /// Computer animation state
    animation_timer: Option<Instant>,
    animation_phase: ComputerAnimationPhase,
    animation_moves: Vec<Move>,
    current_animation_move: usize,
    animation_selected_piece: Option<Position>,
    animation_legal_moves: Vec<Position>,
}

#[derive(Debug, Clone, PartialEq)]
enum ComputerAnimationPhase {
    None,
    InitialPause,
    ShowingSelectedPiece,
    ShowingLegalMoves,
    ExecutingMove,
    DelayBetweenMoves,
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
            cursor_position: None,
            debug_mode: false,
            show_coordinates: false,
            animation_timer: None,
            animation_phase: ComputerAnimationPhase::None,
            animation_moves: Vec::new(),
            current_animation_move: 0,
            animation_selected_piece: None,
            animation_legal_moves: Vec::new(),
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

    /// Toggle display of coordinate labels
    pub fn toggle_coordinates(&mut self) {
        self.show_coordinates = !self.show_coordinates;
    }

    /// Get animation selected piece for rendering red highlight
    pub fn get_animation_selected_piece(&self) -> Option<Position> {
        if self.game_state == GameState::ComputerAnimating {
            self.animation_selected_piece
        } else {
            None
        }
    }

    /// Get animation legal moves for rendering purple highlights  
    pub fn get_animation_legal_moves(&self) -> Vec<Position> {
        if self.game_state == GameState::ComputerAnimating {
            self.animation_legal_moves.clone()
        } else {
            Vec::new()
        }
    }

    /// Check if coordinate display is enabled
    pub fn show_coordinates(&self) -> bool {
        self.show_coordinates
    }

    /// Get current cursor position
    pub fn get_cursor_position(&self) -> Option<Position> {
        self.cursor_position
    }

    /// Handle piece selection from UI
    /// Returns true if selection changed
    pub fn select_piece(&mut self, pos: Position) -> bool {
        // Always update cursor position when clicking on any tile
        self.cursor_position = Some(pos);
        
        // Only allow selection during player's turn
        if self.game_state != GameState::PlayerTurn {
            return true; // Cursor moved, return true to indicate state change
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
        
        // Get all computer pieces and their best moves
        let computer_pieces = self.engine.get_player_pieces(Player::Computer);
        let mut computer_moves = Vec::new();
        
        for (pos, _piece) in computer_pieces {
            if let Some(computer_move) = self.simple_ai_move(pos) {
                computer_moves.push(computer_move);
            }
        }

        if computer_moves.is_empty() {
            // No moves available - end turn
            self.game_state = GameState::GameOver;
            return true;
        }

        // Start computer animation sequence
        self.animation_moves = computer_moves;
        self.current_animation_move = 0;
        self.animation_phase = ComputerAnimationPhase::InitialPause;
        self.animation_timer = Some(Instant::now());
        self.game_state = GameState::ComputerAnimating;
        
        println!("Starting computer animation with {} moves", self.animation_moves.len());
        true
    }

    /// Update computer animation state machine
    pub fn update_computer_animation(&mut self) -> bool {
        if self.game_state != GameState::ComputerAnimating {
            return false;
        }

        let Some(timer_start) = self.animation_timer else {
            return false;
        };

        let elapsed = timer_start.elapsed().as_secs_f32();

        match self.animation_phase {
            ComputerAnimationPhase::InitialPause => {
                if elapsed >= 0.5 {
                    // Move to next phase - show selected piece
                    if self.current_animation_move < self.animation_moves.len() {
                        let current_move = &self.animation_moves[self.current_animation_move];
                        self.animation_selected_piece = Some(current_move.from);
                        
                        // Get legal moves for this piece
                        self.animation_legal_moves = self.engine.get_legal_moves(current_move.from);
                        
                        self.animation_phase = ComputerAnimationPhase::ShowingSelectedPiece;
                        self.animation_timer = Some(Instant::now());
                        
                        println!("Animation: Showing selected piece at {}", current_move.from.to_chess_notation());
                    }
                }
            }
            ComputerAnimationPhase::ShowingSelectedPiece => {
                if elapsed >= 0.3 {
                    // Move to showing legal moves
                    self.animation_phase = ComputerAnimationPhase::ShowingLegalMoves;
                    self.animation_timer = Some(Instant::now());
                    
                    println!("Animation: Showing {} legal moves", self.animation_legal_moves.len());
                }
            }
            ComputerAnimationPhase::ShowingLegalMoves => {
                if elapsed >= 1.2 {
                    // Execute the move
                    let current_move = self.animation_moves[self.current_animation_move].clone();
                    let move_notation = current_move.to_notation();
                    
                    match self.engine.make_move(current_move) {
                        Ok(()) => {
                            println!("Computer executed move: {}", move_notation);
                            
                            // Clear animation highlights
                            self.animation_selected_piece = None;
                            self.animation_legal_moves.clear();
                            
                            self.animation_phase = ComputerAnimationPhase::ExecutingMove;
                            self.animation_timer = Some(Instant::now());
                        }
                        Err(e) => {
                            println!("Computer move failed: {}", e);
                            // Skip this move
                            self.current_animation_move += 1;
                            self.animation_phase = ComputerAnimationPhase::DelayBetweenMoves;
                            self.animation_timer = Some(Instant::now());
                        }
                    }
                }
            }
            ComputerAnimationPhase::ExecutingMove => {
                if elapsed >= 0.1 {
                    // Move to delay between moves or finish
                    self.current_animation_move += 1;
                    
                    if self.current_animation_move >= self.animation_moves.len() {
                        // All moves completed
                        self.finish_computer_animation();
                        return true;
                    } else {
                        // Prepare for next move
                        self.animation_phase = ComputerAnimationPhase::DelayBetweenMoves;
                        self.animation_timer = Some(Instant::now());
                    }
                }
            }
            ComputerAnimationPhase::DelayBetweenMoves => {
                if elapsed >= 0.33 {
                    // Start next move
                    if self.current_animation_move < self.animation_moves.len() {
                        let current_move = &self.animation_moves[self.current_animation_move];
                        self.animation_selected_piece = Some(current_move.from);
                        self.animation_legal_moves = self.engine.get_legal_moves(current_move.from);
                        
                        self.animation_phase = ComputerAnimationPhase::ShowingSelectedPiece;
                        self.animation_timer = Some(Instant::now());
                    }
                }
            }
            ComputerAnimationPhase::None => {
                // Should not be here
                self.finish_computer_animation();
                return true;
            }
        }

        false
    }

    /// Finish computer animation and return to player turn
    fn finish_computer_animation(&mut self) {
        println!("Computer animation finished");
        
        // Clear animation state
        self.animation_timer = None;
        self.animation_phase = ComputerAnimationPhase::None;
        self.animation_moves.clear();
        self.current_animation_move = 0;
        self.animation_selected_piece = None;
        self.animation_legal_moves.clear();
        
        // Check game state
        if self.engine.is_game_over() {
            self.game_state = GameState::GameOver;
            println!("Game Over!");
        } else {
            self.game_state = GameState::PlayerTurn;
            println!("Switching to player turn");
        }
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
            GameState::ComputerThinking => "Computer is thinking...".to_string(),
            GameState::ComputerAnimating => "Computer is making moves...".to_string(),
            GameState::GameOver => "Game Over!".to_string(),
        }
    }
}