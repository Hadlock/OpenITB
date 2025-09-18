# OpenITB - Into the Breach Clone

## Project Overview

This is a Rust-based clone of Into the Breach, implementing a turn-based tactical game with isometric graphics. The project follows chess engine architecture patterns for familiarity among chess programmers.

## Architecture

The game is split into three main components:

### 1. Engine Module (`openitb-engine/`)
- **Location**: `openitb-engine/src/`
- **Purpose**: Core game logic and move validation
- **Protocol**: UCI-like interface for communication with GUI/TUI
- **Key Files**:
  - `board.rs`: 8x8 board with chess notation (a1-h8), Position struct, Move struct
  - `game.rs`: Game state management, turn logic, piece movement
  - `lib.rs`: Public API and engine interface

### 2. GUI Module (`openitb-game/src/gui.rs`)
- **Framework**: Macroquad 0.4
- **Rendering**: Isometric tiles (120x60px) with transparency
- **Key Features**:
  - Mouse input handling with coordinate transformation
  - Layered rendering (terrain → highlights → pieces → UI)
  - Real-time legal move visualization
- **Critical Implementation Notes**:
  - **Coordinate System**: Uses chess notation internally (1-8 ranks, a-h files)
  - **Screen-to-Board Transform**: Custom isometric math with 0.5 file offset correction
  - **Tile Assets**: Located in `assets/tiles/`, diamond-shaped PNG files

### 3. TUI Module (`openitb-game/src/tui.rs`)
- **Framework**: Ratatui
- **Purpose**: Debug interface with toggleable layers
- **Features**: Multi-layer visualization, real-time game state display

## Chess-Specific Implementation Details

### Coordinate Systems
- **Internal Representation**: 0-based indexing (file: 0-7, rank: 0-7)
- **Chess Notation**: 1-based display (a1-h8) via `Position::to_chess_notation()`
- **GUI Coordinate Transform**: 
  ```rust
  // Critical: File calculation needs 0.5 offset correction
  let corrected_x = board_x - 0.5; // Fixes A8 → B8 offset bug
  let file = corrected_x.round() as i32;
  ```

### UCI-Like Protocol
- Move format: `Position::from` → `Position::to` (e.g., "a1b2")
- Engine communication via structured commands
- State machine pattern for turn management

## Game Mechanics

### Pieces
- **Mechs (Human)**: Blue-tinted, up to 3 move range
- **Leapers (Computer)**: Red-tinted, up to 3 move range

### Movement Rules
- **Blocked Terrain**: Mountains, occupied tiles
- **Movement Costs**: Trees and water reduce available moves by 1
- **Range**: Maximum 3 tiles per turn

### Turn Structure
1. **Player Turn**: Select mech → highlight legal moves → click destination
2. **Computer Turn**: AI automatically moves all leapers
3. **Repeat**: State machine handles turn transitions

## Technical Challenges Solved

### Isometric Coordinate Transformation
**Problem**: Mouse clicks were offset by one tile (clicking A8 → detected as B8)

**Root Cause**: Mismatch between isometric math and chess coordinate expectations

**Solution**: 
```rust
fn screen_to_board(&self, screen_pos: Vec2) -> Option<Position> {
    // ... isometric inverse transformation ...
    let corrected_x = board_x - 0.5; // Critical offset correction
    let corrected_y = board_y - 0.0; // Y was already correct
    let file = corrected_x.round() as i32;
    let rank = corrected_y.round() as i32;
}
```

### Coordinate Label Display
**Implementation**: Labels drawn outside board boundaries for debugging
- File letters (a-h): Above and below board edges
- Rank numbers (1-8): Left and right of board edges
- Toggle with 'C' key (when coordinate display is implemented)

## Asset Management

### Tile Requirements
- **Size**: 120x60 pixels
- **Format**: PNG with transparency
- **Shape**: Diamond/rhombus for isometric view
- **Location**: `assets/tiles/`

### Key Assets
- `ground.png`, `trees.png`, `mountain.png`, `water.png`: Terrain
- `mech.png`, `leaper.png`: Game pieces  
- `yellow_cursor.png`, `green_highlight.png`: UI overlays

## Development Workflow

### Building & Running
```bash
cargo build          # Compile project
cargo run            # Start GUI mode
cargo test           # Run unit tests
```

### Debug Features
- **Console Output**: Detailed coordinate transformation logging
- **TUI Mode**: Press Space to switch to debug view (when implemented)
- **Coordinate Display**: Press 'C' to toggle coordinate labels

## State Management

### Game States
```rust
enum GameState {
    PlayerTurn,     // Waiting for player input
    ComputerTurn,   // AI processing moves
    GameOver,       // End state
}
```

### Turn Logic
- **Player Phase**: Select piece → show legal moves → execute move
- **Computer Phase**: Process all AI moves sequentially
- **State Transitions**: Handled by GameManager

## Future Development

### Planned Features
- [ ] Coordinate display toggle (C key)
- [ ] Sound system integration
- [ ] Animation system for moves
- [ ] Advanced AI with difficulty levels
- [ ] Campaign/scenario system

### Code Structure Improvements
- [ ] Reduce coordinate system warnings
- [ ] Implement proper error handling
- [ ] Add comprehensive test coverage
- [ ] Performance optimization for large boards

## Known Issues

### Compiler Warnings (Non-Critical)
- `TuiMode` variant never constructed
- `scale` field never read
- Some methods never used (future features)

### Architecture Notes
- Engine follows chess programming conventions
- Clear separation between logic (engine) and presentation (GUI/TUI)
- State machine pattern ensures clean turn management
- Isometric rendering requires careful coordinate transformation

## For New Contributors

1. **Start with Engine**: Understand `Position`, `Move`, and `Board` structs
2. **GUI Debugging**: Use coordinate display and console output
3. **Coordinate Systems**: Remember 0-based internal vs 1-based display
4. **Testing**: Click corner tiles (A1, H1, H8, A8) to verify coordinate accuracy
5. **Chess Knowledge**: UCI protocol and chess engine patterns are used throughout

This codebase is designed to be familiar to chess engine developers while implementing tactical turn-based gameplay mechanics.