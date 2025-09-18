![openitb logo](https://github.com/hadlock/openitb/blob/master/static/openitb-logo-sm.png)

# OpenITB - Into the Breach Clone

A Rust implementation of an Into the Breach-style tactical game, featuring a modular architecture inspired by chess engine protocols.

## Architecture

The project follows a three-layer architecture:

### 1. **Engine Module** (`openitb-engine`)
- Handles all game logic and move validation
- Uses a UCI-like protocol for communication
- Implements 8x8 board with chess-like coordinate system (a1-h8)
- Supports multi-layered game state (terrain, pieces, effects)

### 2. **TUI Debug Interface** (`src/tui.rs`)
- Terminal-based interface using ratatui
- Toggleable layer visualization for debugging
- Real-time game state display
- Keyboard controls for layer management

### 3. **GUI Interface** (`src/gui.rs`)
- Visual interface using macroquad
- Isometric tile rendering (120x60px tiles)
- Mouse-based piece selection and movement
- Real-time legal move highlighting

## Game Features

### Current Implementation (MVP/POC)
- ✅ 8x8 isometric board with multiple terrain types
- ✅ Player mechs and computer leapers
- ✅ Legal move calculation (up to 3 spaces with terrain costs)
- ✅ Turn-based gameplay with state machine management
- ✅ Real-time GUI with mouse controls
- ✅ Debug TUI for development
- ✅ Simple AI opponent

### Game Rules
- **Mechs** (Player): Move up to 3 spaces, cannot move through mountains or occupied tiles
- **Leapers** (Computer): Same movement rules as mechs
- **Terrain Effects**: 
  - Trees and water cost 2 movement points per tile
  - Mountains are impassable
  - Other terrain costs 1 movement point

## Controls

### GUI Mode (Default)
- **Left Click**: Select piece / Make move
- **Space**: Toggle debug overlay
- **ESC**: Quit

### TUI Debug Mode
- **T**: Toggle terrain layer
- **P**: Toggle pieces layer  
- **E**: Toggle effects layer
- **L**: Toggle legal moves layer
- **C**: Toggle coordinates
- **Q**: Quit

## Running the Game

### GUI Mode (Recommended)
```bash
cargo run
```

### TUI Debug Mode
```bash
cargo run -- --tui
```

### Build Only
```bash
cargo build --release
```

## Project Structure

```
OpenITB/
├── openitb-engine/          # Game logic engine
│   ├── src/
│   │   ├── board.rs         # Board representation and moves
│   │   ├── engine.rs        # Game logic and AI
│   │   └── lib.rs          # Public API
│   └── Cargo.toml
├── openitb-game/           # Main game application  
│   ├── src/
│   │   ├── gui.rs          # Macroquad graphics
│   │   ├── tui.rs          # Ratatui terminal interface
│   │   ├── game_manager.rs # State management
│   │   └── main.rs         # Application entry point
│   └── Cargo.toml
├── assets/tiles/           # Isometric tile graphics
└── Cargo.toml             # Workspace configuration
```

## Development Notes

- The engine uses a protocol similar to UCI (Universal Chess Interface)
- State management follows a finite state machine pattern
- All components communicate through structured data types
- The codebase is designed to be familiar to chess engine developers

## Asset Requirements

Place 120x60px isometric tile images in `assets/tiles/`:
- `ground.gif` - Basic grass terrain
- `double.png` - Buildings
- `trees.png` - Forest tiles
- `mountain.png` - Impassable terrain
- `water.png` - Water terrain
- `mech.png` - Player pieces
- `leaper.png` - Enemy pieces
- `green_highlight.gif` - Legal move indicators
- `yellow_cursor.gif` - Selection highlight

Missing assets will fall back to colored rectangles.

## Future Enhancements

- More piece types and abilities
- Advanced AI with minimax evaluation
- Multiplayer support
- Save/load game states
- Campaign mode with scenarios
- Sound effects and animations
- Asset loading improvements

Open source turn based strategy game based on Into The Breach

Mantafish strongly based off of sunfish chess engine. GUI supports traditional three-quarter overhead view with layered tiles and mouse click using tkinter gui.

Screenshot:

![openitb screenshot](https://github.com/hadlock/openitb/blob/master/static/openitb-screenshot-sm.png)

# Installing tkinter

* Ubuntu: `sudo apt-get install python3-tk`
* RHEL/Centos: `sudo yum install python3-tkinter`
* Windows: https://tkdocs.com/tutorial/install.html

# TODO

* cursor icon
* controller support, D pad, ABXY, R, L, XR, XL, start, select
* place characters
* transparent move overlay icon
* randomly place enemies
* state machine