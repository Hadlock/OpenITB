use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chess-like coordinate system (a1-h8)
/// Represents position on 8x8 board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub file: u8, // 0-7 (a-h)
    pub rank: u8, // 0-7 (1-8)
}

impl Position {
    pub fn new(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Position { file, rank })
        } else {
            None
        }
    }

    /// Create position from chess notation (e.g., "a1", "h8")
    pub fn from_chess_notation(notation: &str) -> Option<Self> {
        if notation.len() != 2 {
            return None;
        }
        let chars: Vec<char> = notation.chars().collect();
        let file = (chars[0] as u8).checked_sub(b'a')?;
        let rank = (chars[1] as u8).checked_sub(b'1')?;
        Self::new(file, rank)
    }

    /// Convert to chess notation
    pub fn to_chess_notation(&self) -> String {
        format!("{}{}", (b'a' + self.file) as char, (b'1' + self.rank) as char)
    }
}

/// Terrain types on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Terrain {
    Grass,          // G - passable
    SingleBuilding, // S - passable
    DoubleBuilding, // D - passable
    Forest,         // T - passable but costs extra movement
    Mountain,       // M - impassable
    Water,          // W - passable but costs extra movement
    MonsterIngress, // I - passable, spawns monsters
    PowerGenerator, // P - special objective
    Landmine,       // L - trap
    Rocket,         // R - weapon
}

impl Terrain {
    /// Movement cost for traversing this terrain
    pub fn movement_cost(&self) -> u8 {
        match self {
            Terrain::Grass | Terrain::SingleBuilding | Terrain::DoubleBuilding
            | Terrain::MonsterIngress | Terrain::PowerGenerator
            | Terrain::Landmine | Terrain::Rocket => 1,
            Terrain::Forest | Terrain::Water => 2, // Extra movement cost
            Terrain::Mountain => u8::MAX,           // Impassable
        }
    }

    /// Whether this terrain blocks movement
    pub fn is_passable(&self) -> bool {
        !matches!(self, Terrain::Mountain)
    }
}

/// Game pieces - basic unit types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Mech,   // P - player controlled
    Leaper, // H - enemy
}

/// Weapon types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weapon {
    None,
    PulseCannon,
    RocketLauncher,
    FlameCanon,
    LaserRifle,
    MissilePod,
    // Add more weapons as needed
}

/// Attack patterns for units
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackPattern {
    None,
    Rook,           // Straight lines, unlimited range
    LimitedRook(u8), // Straight lines, limited range
    Pawn,           // Forward attack only
    Knight,         // L-shaped pattern
    King,           // Adjacent squares
    Queen,          // Rook + Bishop
    Bishop,         // Diagonal lines
    RangedBallistic(u8), // Future: artillery with range and arc
}

impl AttackPattern {
    /// Get all possible attack positions from a given position
    pub fn get_attack_positions(&self, from: Position, max_range: Option<u8>) -> Vec<Position> {
        let range = max_range.unwrap_or(7); // Default to board edge
        let mut positions = Vec::new();

        match self {
            AttackPattern::None => {},
            AttackPattern::LimitedRook(pattern_range) => {
                let actual_range = range.min(*pattern_range);
                positions.extend(self.get_rook_positions(from, actual_range));
            },
            AttackPattern::Rook => {
                positions.extend(self.get_rook_positions(from, range));
            },
            AttackPattern::King => {
                positions.extend(self.get_king_positions(from));
            },
            AttackPattern::Knight => {
                positions.extend(self.get_knight_positions(from));
            },
            AttackPattern::Pawn => {
                positions.extend(self.get_pawn_positions(from));
            },
            AttackPattern::Queen => {
                positions.extend(self.get_rook_positions(from, range));
                positions.extend(self.get_bishop_positions(from, range));
            },
            AttackPattern::Bishop => {
                positions.extend(self.get_bishop_positions(from, range));
            },
            AttackPattern::RangedBallistic(artillery_range) => {
                // TODO: Implement ballistic arc calculations
                positions.extend(self.get_rook_positions(from, *artillery_range));
            },
        }

        positions
    }

    fn get_rook_positions(&self, from: Position, range: u8) -> Vec<Position> {
        let mut positions = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // Up, Down, Right, Left

        for (dx, dy) in directions {
            for i in 1..=range {
                let new_file = from.file as i8 + dx * i as i8;
                let new_rank = from.rank as i8 + dy * i as i8;
                
                if let Some(pos) = Position::new(new_file as u8, new_rank as u8) {
                    positions.push(pos);
                } else {
                    break; // Out of bounds, stop in this direction
                }
            }
        }
        positions
    }

    fn get_bishop_positions(&self, from: Position, range: u8) -> Vec<Position> {
        let mut positions = Vec::new();
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]; // Diagonals

        for (dx, dy) in directions {
            for i in 1..=range {
                let new_file = from.file as i8 + dx * i as i8;
                let new_rank = from.rank as i8 + dy * i as i8;
                
                if let Some(pos) = Position::new(new_file as u8, new_rank as u8) {
                    positions.push(pos);
                } else {
                    break;
                }
            }
        }
        positions
    }

    fn get_king_positions(&self, from: Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for (dx, dy) in directions {
            let new_file = from.file as i8 + dx;
            let new_rank = from.rank as i8 + dy;
            
            if let Some(pos) = Position::new(new_file as u8, new_rank as u8) {
                positions.push(pos);
            }
        }
        positions
    }

    fn get_knight_positions(&self, from: Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let moves = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

        for (dx, dy) in moves {
            let new_file = from.file as i8 + dx;
            let new_rank = from.rank as i8 + dy;
            
            if let Some(pos) = Position::new(new_file as u8, new_rank as u8) {
                positions.push(pos);
            }
        }
        positions
    }

    fn get_pawn_positions(&self, from: Position) -> Vec<Position> {
        let mut positions = Vec::new();
        
        // Pawn attacks diagonally forward (assuming white pawns move "up" the board)
        let forward_attacks = [(1, 1), (-1, 1)];
        
        for (dx, dy) in forward_attacks {
            let new_file = from.file as i8 + dx;
            let new_rank = from.rank as i8 + dy;
            
            if let Some(pos) = Position::new(new_file as u8, new_rank as u8) {
                positions.push(pos);
            }
        }
        positions
    }
}

/// Powerup types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Powerup {
    None,
    ShieldGenerator,
    ExtraMove,
    DamageBoost,
    RangeExtender,
    // Add more powerups as needed
}

/// Special effects that can affect units
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialEffect {
    Poisoned,
    Stunned,
    Burning,
    Frozen,
    // Add more effects as needed
}

/// Complete unit definition with all attributes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unit {
    pub piece_type: PieceType,
    pub player: Player,
    pub pilot_name: String,
    pub primary_weapon: Weapon,
    pub secondary_weapon: Weapon,
    pub special_weapon: Weapon,
    pub powerup1: Powerup,
    pub powerup2: Powerup,
    pub powerup3: Powerup,
    pub max_action_tokens: u8,
    pub action_tokens_left: u8,
    pub max_hit_points: u8,
    pub current_hit_points: u8,
    pub attack_pattern: AttackPattern,
    pub attack_damage: u8,
    pub special_effects: Vec<SpecialEffect>,
}

impl Unit {
    /// Create a new mech unit with default values
    pub fn new_mech(pilot_name: String) -> Self {
        Self {
            piece_type: PieceType::Mech,
            player: Player::Human,
            pilot_name,
            primary_weapon: Weapon::PulseCannon,
            secondary_weapon: Weapon::None,
            special_weapon: Weapon::None,
            powerup1: Powerup::None,
            powerup2: Powerup::None,
            powerup3: Powerup::None,
            max_action_tokens: 3, // Mechs get 3 action tokens
            action_tokens_left: 3,
            max_hit_points: 3,
            current_hit_points: 3,
            attack_pattern: AttackPattern::LimitedRook(1), // Can attack adjacent squares in rook pattern
            attack_damage: 1,
            special_effects: Vec::new(),
        }
    }

    /// Create a new leaper unit with default values
    pub fn new_leaper(pilot_name: String) -> Self {
        Self {
            piece_type: PieceType::Leaper,
            player: Player::Computer,
            pilot_name,
            primary_weapon: Weapon::LaserRifle,
            secondary_weapon: Weapon::None,
            special_weapon: Weapon::None,
            powerup1: Powerup::None,
            powerup2: Powerup::None,
            powerup3: Powerup::None,
            max_action_tokens: 2, // Leapers get 2 action tokens
            action_tokens_left: 2,
            max_hit_points: 2,
            current_hit_points: 2,
            attack_pattern: AttackPattern::LimitedRook(1), // Can leap to attack adjacent rook squares
            attack_damage: 1,
            special_effects: Vec::new(),
        }
    }

    /// Reset action tokens for a new turn
    pub fn reset_action_tokens(&mut self) {
        self.action_tokens_left = self.max_action_tokens;
        
        // Apply powerup effects
        if self.powerup1 == Powerup::ExtraMove || 
           self.powerup2 == Powerup::ExtraMove || 
           self.powerup3 == Powerup::ExtraMove {
            self.action_tokens_left += 1;
        }
    }


    /// Reset action tokens for a new turn
    pub fn reset_turn(&mut self) {
        self.reset_action_tokens();
    }

    /// Use one action token for movement or attack
    pub fn use_action_token(&mut self) -> bool {
        if self.action_tokens_left > 0 {
            self.action_tokens_left -= 1;
            true
        } else {
            false
        }
    }

    /// Check if unit can perform any action (move or attack)
    pub fn can_act(&mut self) -> bool {
        self.action_tokens_left > 0 && 
        !self.special_effects.contains(&SpecialEffect::Stunned) &&
        !self.special_effects.contains(&SpecialEffect::Frozen)
    }

    /// Check if unit can still move (legacy method for compatibility)
    pub fn can_move(&self) -> bool {
        self.action_tokens_left > 0 && 
        !self.special_effects.contains(&SpecialEffect::Stunned) &&
        !self.special_effects.contains(&SpecialEffect::Frozen)
    }

    /// Check if unit can still attack (legacy method for compatibility)
    pub fn can_attack(&self) -> bool {
        self.action_tokens_left > 0 &&
        !self.special_effects.contains(&SpecialEffect::Stunned) &&
        !self.special_effects.contains(&SpecialEffect::Frozen)
    }

    /// Take damage and return true if unit is destroyed
    pub fn take_damage(&mut self, damage: u8) -> bool {
        self.current_hit_points = self.current_hit_points.saturating_sub(damage);
        !self.is_alive()
    }

    /// Get all possible attack targets from current position
    pub fn get_attack_targets(&self, from: Position, board: &Board) -> Vec<Position> {
        let possible_positions = self.attack_pattern.get_attack_positions(from, None);
        
        // Filter to only include positions with enemy units
        possible_positions.into_iter()
            .filter(|&pos| {
                if let Some(target_unit) = board.get_piece(pos) {
                    target_unit.player != self.player && target_unit.is_alive()
                } else {
                    false
                }
            })
            .collect()
    }

    /// Check if unit is alive
    pub fn is_alive(&self) -> bool {
        self.current_hit_points > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Human,
    Computer,
}

/// Game board with multiple layers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    /// Terrain layer - the base map
    pub terrain: [[Terrain; 8]; 8],
    /// Units layer - current unit positions
    pub pieces: HashMap<Position, Unit>,
    /// Effects layer - temporary effects like poison gas
    pub effects: HashMap<Position, Vec<Effect>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    PoisonGas,
    Radiation,
    Fire,
}

impl Board {
    /// Create a new board from a test map string
    pub fn from_map_string(map_str: &str) -> Self {
        let mut terrain = [[Terrain::Grass; 8]; 8];
        let mut pieces = HashMap::new();

        for (row, line) in map_str.lines().enumerate() {
            if row >= 8 {
                break;
            }
            for (col, cell) in line.split_whitespace().enumerate() {
                if col >= 8 {
                    break;
                }
                
                // Direct mapping: row 0 of string -> rank 7, row 7 of string -> rank 0
                // col 0 of string -> file 0, col 7 of string -> file 7
                let pos = Position::new(col as u8, (7 - row) as u8).unwrap();
                
                match cell {
                    "G" => terrain[row][col] = Terrain::Grass,
                    "S" => terrain[row][col] = Terrain::SingleBuilding,
                    "D" => terrain[row][col] = Terrain::DoubleBuilding,
                    "T" => terrain[row][col] = Terrain::Forest,
                    "M" => terrain[row][col] = Terrain::Mountain,
                    "W" => terrain[row][col] = Terrain::Water,
                    "I" => terrain[row][col] = Terrain::MonsterIngress,
                    "P" => {
                        terrain[row][col] = Terrain::Grass;
                        pieces.insert(pos, Unit::new_mech(format!("Pilot-{}", pos.to_chess_notation())));
                    },
                    "H" => {
                        terrain[row][col] = Terrain::Grass;
                        pieces.insert(pos, Unit::new_leaper(format!("Enemy-{}", pos.to_chess_notation())));
                    },
                    _ => terrain[row][col] = Terrain::Grass,
                }
            }
        }

        Self {
            terrain,
            pieces,
            effects: HashMap::new(),
        }
    }

    /// Get terrain at position
    pub fn get_terrain(&self, pos: Position) -> Terrain {
        // Convert chess position to array indices
        // rank 7 (8th rank) -> row 0, rank 0 (1st rank) -> row 7
        let row = (7 - pos.rank) as usize;
        let col = pos.file as usize;
        self.terrain[row][col]
    }

    /// Get unit at position
    pub fn get_piece(&self, pos: Position) -> Option<&Unit> {
        self.pieces.get(&pos)
    }

    /// Get unit at position (alias for consistency)
    pub fn get_unit(&self, pos: Position) -> Option<&Unit> {
        self.pieces.get(&pos)
    }

    /// Move a piece from one position to another
    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        if let Some(piece) = self.pieces.remove(&from) {
            // Check if destination is valid
            if !self.get_terrain(to).is_passable() {
                return Err("Destination is not passable");
            }
            if self.pieces.contains_key(&to) {
                return Err("Destination is occupied");
            }
            
            self.pieces.insert(to, piece);
            Ok(())
        } else {
            Err("No piece at source position")
        }
    }
}

/// Move representation for UCI-like protocol
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    /// Create move from chess notation (e.g., "a1b2")
    pub fn from_notation(notation: &str) -> Option<Self> {
        if notation.len() != 4 {
            return None;
        }
        let from = Position::from_chess_notation(&notation[0..2])?;
        let to = Position::from_chess_notation(&notation[2..4])?;
        Some(Self::new(from, to))
    }

    /// Convert to chess notation
    pub fn to_notation(&self) -> String {
        format!("{}{}", self.from.to_chess_notation(), self.to.to_chess_notation())
    }
}

/// Game state for state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    PlayerTurn,
    ComputerTurn,
    ComputerThinking,
    ComputerAnimating,
    GameOver,
}

/// Result of an attack action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttackResult {
    pub damage_dealt: u8,
    pub target_destroyed: bool,
    pub target_position: Position,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_chess_notation() {
        let pos = Position::from_chess_notation("a1").unwrap();
        assert_eq!(pos.file, 0);
        assert_eq!(pos.rank, 0);
        assert_eq!(pos.to_chess_notation(), "a1");

        let pos = Position::from_chess_notation("h8").unwrap();
        assert_eq!(pos.file, 7);
        assert_eq!(pos.rank, 7);
        assert_eq!(pos.to_chess_notation(), "h8");
    }

    #[test]
    fn test_move_notation() {
        let mov = Move::from_notation("a1b2").unwrap();
        assert_eq!(mov.from.to_chess_notation(), "a1");
        assert_eq!(mov.to.to_chess_notation(), "b2");
        assert_eq!(mov.to_notation(), "a1b2");
    }
}