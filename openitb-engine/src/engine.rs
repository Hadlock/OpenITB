use crate::board::{Board, Move, Position, Player, PieceType, Unit};
use anyhow::Result;
use std::collections::HashSet;

/// Engine that calculates legal moves and evaluates positions
/// Inspired by UCI (Universal Chess Interface) protocol
pub struct Engine {
    board: Board,
}

impl Engine {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    /// Calculate all legal moves for a piece at the given position
    pub fn get_legal_moves(&self, pos: Position) -> Vec<Position> {
        let piece = match self.board.get_piece(pos) {
            Some(piece) => piece,
            None => return Vec::new(),
        };

        match piece.piece_type {
            PieceType::Mech | PieceType::Leaper => self.calculate_movement_range(pos, 3),
        }
    }

    /// Calculate movement range for a piece (up to max_moves squares)
    /// Respects terrain movement costs and obstacles
    fn calculate_movement_range(&self, start: Position, max_moves: u8) -> Vec<Position> {
        let mut reachable = HashSet::new();
        let mut to_visit = vec![(start, 0u8)]; // (position, moves_used)
        let mut visited = HashSet::new();

        while let Some((current_pos, moves_used)) = to_visit.pop() {
            if visited.contains(&current_pos) {
                continue;
            }
            visited.insert(current_pos);

            // Don't include starting position in legal moves
            if current_pos != start {
                reachable.insert(current_pos);
            }

            // Try all adjacent positions
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_file = current_pos.file as i8 + dx;
                let new_rank = current_pos.rank as i8 + dy;

                if new_file < 0 || new_file >= 8 || new_rank < 0 || new_rank >= 8 {
                    continue;
                }

                let new_pos = Position::new(new_file as u8, new_rank as u8).unwrap();
                
                if visited.contains(&new_pos) {
                    continue;
                }

                // Check if we can move to this position
                let terrain = self.board.get_terrain(new_pos);
                if !terrain.is_passable() {
                    continue;
                }

                // Check if position is occupied by another piece
                if self.board.get_piece(new_pos).is_some() {
                    continue;
                }

                let movement_cost = terrain.movement_cost();
                let new_moves_used = moves_used + movement_cost;

                if new_moves_used <= max_moves {
                    to_visit.push((new_pos, new_moves_used));
                }
            }
        }

        reachable.into_iter().collect()
    }

    /// Make a move on the board
    pub fn make_move(&mut self, mov: Move) -> Result<()> {
        // Validate the move is legal
        let legal_moves = self.get_legal_moves(mov.from);
        if !legal_moves.contains(&mov.to) {
            anyhow::bail!("Illegal move: {}", mov.to_notation());
        }

        // Execute the move
        self.board.move_piece(mov.from, mov.to)
            .map_err(|e| anyhow::anyhow!("Move failed: {}", e))?;

        Ok(())
    }

    /// Get current board state
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Get mutable board reference
    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Reset moves for all units of a given player
    pub fn reset_player_moves(&mut self, player: Player) {
        for unit in self.board.pieces.values_mut() {
            if unit.player == player {
                unit.reset_moves();
            }
        }
    }

    /// Use a move for the unit at the given position
    pub fn use_unit_move(&mut self, pos: Position) -> bool {
        if let Some(unit) = self.board.pieces.get_mut(&pos) {
            unit.use_move()
        } else {
            false
        }
    }

    /// Get all pieces for a given player
    pub fn get_player_pieces(&self, player: Player) -> Vec<(Position, Unit)> {
        self.board
            .pieces
            .iter()
            .filter(|(_, unit)| unit.player == player)
            .map(|(&pos, unit)| (pos, unit.clone()))
            .collect()
    }

    /// Simple AI: get a random legal move for the computer player
    pub fn get_computer_move(&self) -> Option<Move> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let computer_pieces = self.get_player_pieces(Player::Computer);
        
        for (pos, _piece) in computer_pieces {
            let legal_moves = self.get_legal_moves(pos);
            if !legal_moves.is_empty() {
                // Simple pseudo-random selection based on position
                let mut hasher = DefaultHasher::new();
                pos.hash(&mut hasher);
                let hash = hasher.finish();
                let index = (hash as usize) % legal_moves.len();
                
                return Some(Move::new(pos, legal_moves[index]));
            }
        }
        
        None
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        // Game is over if one side has no pieces
        let human_pieces = self.get_player_pieces(Player::Human);
        let computer_pieces = self.get_player_pieces(Player::Computer);
        
        human_pieces.is_empty() || computer_pieces.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Terrain, PieceType};

    #[test]
    fn test_legal_moves_basic() {
        let test_map = "G G G G G G G G\n\
                        G G G G G G G G\n\
                        G G G G G G G G\n\
                        G G G P G G G G\n\
                        G G G G G G G G\n\
                        G G G G G G G G\n\
                        G G G G G G G G\n\
                        G G G G G G G G";
        
        let board = Board::from_map_string(test_map);
        let engine = Engine::new(board);
        
        let mech_pos = Position::from_chess_notation("d4").unwrap();
        let legal_moves = engine.get_legal_moves(mech_pos);
        
        // Should be able to move up to 3 spaces in any direction
        assert!(!legal_moves.is_empty());
        assert!(legal_moves.contains(&Position::from_chess_notation("d1").unwrap()));
        assert!(legal_moves.contains(&Position::from_chess_notation("d7").unwrap()));
        assert!(legal_moves.contains(&Position::from_chess_notation("a4").unwrap()));
        assert!(legal_moves.contains(&Position::from_chess_notation("g4").unwrap()));
    }
}