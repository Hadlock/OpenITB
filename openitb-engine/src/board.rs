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

/// Game pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Mech,   // P - player controlled
    Leaper, // H - enemy
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: Player,
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
    /// Pieces layer - current piece positions
    pub pieces: HashMap<Position, Piece>,
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
                        pieces.insert(pos, Piece {
                            piece_type: PieceType::Mech,
                            player: Player::Human,
                        });
                    },
                    "H" => {
                        terrain[row][col] = Terrain::Grass;
                        pieces.insert(pos, Piece {
                            piece_type: PieceType::Leaper,
                            player: Player::Computer,
                        });
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

    /// Get piece at position
    pub fn get_piece(&self, pos: Position) -> Option<&Piece> {
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