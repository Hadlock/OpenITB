use crate::board::{Board, Move, Position, Player, PieceType, Unit, AttackResult};
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

    /// Calculate the minimum number of moves required to go from one position to another
    /// This uses Manhattan distance since pieces move orthogonally
    pub fn calculate_move_distance(&self, from: Position, to: Position) -> u8 {
        let dx = (to.file as i8 - from.file as i8).abs() as u8;
        let dy = (to.rank as i8 - from.rank as i8).abs() as u8;
        dx + dy // Manhattan distance for orthogonal movement
    }

    /// Make a move on the board
    pub fn make_move(&mut self, mov: Move) -> Result<()> {
        // Validate the move is legal
        let legal_moves = self.get_legal_moves(mov.from);
        if !legal_moves.contains(&mov.to) {
            anyhow::bail!("Illegal move: {}", mov.to_notation());
        }

        // Calculate move distance (action tokens required)
        let move_distance = self.calculate_move_distance(mov.from, mov.to);
        
        // Check if unit has enough action tokens
        if let Some(unit) = self.board.pieces.get(&mov.from) {
            if unit.action_tokens_left < move_distance {
                anyhow::bail!("Unit needs {} action tokens but only has {}", move_distance, unit.action_tokens_left);
            }
        } else {
            anyhow::bail!("No unit at source position");
        }

        // Consume the required action tokens
        for _ in 0..move_distance {
            if !self.use_unit_action_token(mov.from) {
                anyhow::bail!("Failed to consume action token");
            }
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

    /// Reset action tokens for all units of a given player
    pub fn reset_player_action_tokens(&mut self, player: Player) {
        for unit in self.board.pieces.values_mut() {
            if unit.player == player {
                unit.reset_action_tokens();
            }
        }
    }


    /// Reset action tokens for all units of a given player
    pub fn reset_player_turn(&mut self, player: Player) {
        for unit in self.board.pieces.values_mut() {
            if unit.player == player {
                unit.reset_turn();
            }
        }
    }

    /// Use an action token for the unit at the given position
    pub fn use_unit_action_token(&mut self, pos: Position) -> bool {
        if let Some(unit) = self.board.pieces.get_mut(&pos) {
            unit.use_action_token()
        } else {
            false
        }
    }

    /// Get all possible attack targets for a unit at the given position
    pub fn get_attack_targets(&self, pos: Position) -> Vec<Position> {
        if let Some(unit) = self.board.pieces.get(&pos) {
            if unit.can_attack() {
                unit.get_attack_targets(pos, &self.board)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    /// Execute an attack from one position to another
    pub fn execute_attack(&mut self, attacker_pos: Position, target_pos: Position) -> Result<AttackResult> {
        // Validate attacker exists and can attack
        let attacker_damage = {
            let attacker = self.board.pieces.get(&attacker_pos)
                .ok_or_else(|| anyhow::anyhow!("No unit at attacker position"))?;
            
            if !attacker.can_attack() {
                return Err(anyhow::anyhow!("Attacker cannot attack"));
            }

            let valid_targets = attacker.get_attack_targets(attacker_pos, &self.board);
            if !valid_targets.contains(&target_pos) {
                return Err(anyhow::anyhow!("Invalid attack target"));
            }

            attacker.attack_damage
        };

        // Use attacker's action token for the attack
        if !self.use_unit_action_token(attacker_pos) {
            return Err(anyhow::anyhow!("Failed to use action token for attack"));
        }

        // Apply damage to target
        let target_destroyed = if let Some(target) = self.board.pieces.get_mut(&target_pos) {
            target.take_damage(attacker_damage)
        } else {
            return Err(anyhow::anyhow!("No target at position"));
        };

        // Remove destroyed units
        if target_destroyed {
            self.board.pieces.remove(&target_pos);
        }

        Ok(AttackResult {
            damage_dealt: attacker_damage,
            target_destroyed,
            target_position: target_pos,
        })
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

    /// Get all positions for pieces of a given player
    pub fn get_player_positions(&self, player: Player) -> Vec<Position> {
        self.board
            .pieces
            .iter()
            .filter(|(_, unit)| unit.player == player)
            .map(|(&pos, _)| pos)
            .collect()
    }

    /// Get unit at a given position
    pub fn get_unit(&self, pos: Position) -> Option<&Unit> {
        self.board.pieces.get(&pos)
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
    // Testing utilities no longer needed

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