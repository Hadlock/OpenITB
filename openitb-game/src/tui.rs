use openitb_engine::{Board, Position, Player, PieceType, Terrain};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashSet;
use std::io;

pub struct DebugTui {
    /// Currently selected position for debugging
    pub selected_position: Option<Position>,
    /// Legal moves for selected piece
    pub legal_moves: Vec<Position>,
    /// Layer visibility toggles
    pub show_terrain: bool,
    pub show_pieces: bool,
    pub show_effects: bool,
    pub show_legal_moves: bool,
    pub show_coordinates: bool,
}

impl Default for DebugTui {
    fn default() -> Self {
        Self {
            selected_position: None,
            legal_moves: Vec::new(),
            show_terrain: true,
            show_pieces: true,
            show_effects: true,
            show_legal_moves: true,
            show_coordinates: true,
        }
    }
}

impl DebugTui {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set legal moves for display
    pub fn set_legal_moves(&mut self, moves: Vec<Position>) {
        self.legal_moves = moves;
    }

    /// Set selected position
    pub fn set_selected_position(&mut self, pos: Option<Position>) {
        self.selected_position = pos;
    }

    /// Toggle layer visibility
    pub fn toggle_terrain(&mut self) { self.show_terrain = !self.show_terrain; }
    pub fn toggle_pieces(&mut self) { self.show_pieces = !self.show_pieces; }
    pub fn toggle_effects(&mut self) { self.show_effects = !self.show_effects; }
    pub fn toggle_legal_moves(&mut self) { self.show_legal_moves = !self.show_legal_moves; }
    pub fn toggle_coordinates(&mut self) { self.show_coordinates = !self.show_coordinates; }

    /// Render the TUI
    pub fn render(&self, frame: &mut Frame, board: &Board, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(20), // Board area
                Constraint::Length(8), // Info panel
            ])
            .split(area);

        // Render the board
        self.render_board(frame, board, chunks[0]);
        
        // Render info panel
        self.render_info_panel(frame, chunks[1]);
    }

    fn render_board(&self, frame: &mut Frame, board: &Board, area: Rect) {
        let legal_move_set: HashSet<Position> = self.legal_moves.iter().copied().collect();

        // Create board representation
        let mut board_lines = Vec::new();
        
        // Header with file labels (a-h)
        if self.show_coordinates {
            let header = "   a b c d e f g h";
            board_lines.push(Line::from(header));
        }

        // Render each rank (8 to 1, top to bottom)
        for rank in (0..8).rev() {
            let mut line_spans = Vec::new();
            
            if self.show_coordinates {
                // Rank label
                line_spans.push(Span::styled(
                    format!("{} ", rank + 1),
                    Style::default().fg(Color::Gray),
                ));
            }

            for file in 0..8 {
                let pos = Position::new(file, rank).unwrap();
                
                // Determine what to display at this position
                let mut cell_char = ' ';
                let mut cell_color = Color::Gray;
                let mut bg_color = None;

                // Base terrain
                if self.show_terrain {
                    let terrain = board.get_terrain(pos);
                    cell_char = match terrain {
                        Terrain::Grass => '·',
                        Terrain::SingleBuilding => 'S',
                        Terrain::DoubleBuilding => 'D',
                        Terrain::Forest => 'T',
                        Terrain::Mountain => 'M',
                        Terrain::Water => 'W',
                        Terrain::MonsterIngress => 'I',
                        Terrain::PowerGenerator => 'P',
                        Terrain::Landmine => 'L',
                        Terrain::Rocket => 'R',
                    };
                    cell_color = match terrain {
                        Terrain::Grass => Color::Green,
                        Terrain::SingleBuilding | Terrain::DoubleBuilding => Color::Gray,
                        Terrain::Forest => Color::Green,
                        Terrain::Mountain => Color::Yellow,
                        Terrain::Water => Color::Blue,
                        Terrain::MonsterIngress => Color::Red,
                        Terrain::PowerGenerator => Color::Cyan,
                        Terrain::Landmine => Color::Magenta,
                        Terrain::Rocket => Color::White,
                    };
                }

                // Pieces override terrain display
                if self.show_pieces {
                    if let Some(piece) = board.get_piece(pos) {
                        match piece.piece_type {
                            PieceType::Mech => {
                                cell_char = 'M';
                                cell_color = match piece.player {
                                    Player::Human => Color::Cyan,
                                    Player::Computer => Color::Red,
                                };
                            }
                            PieceType::Leaper => {
                                cell_char = 'L';
                                cell_color = match piece.player {
                                    Player::Human => Color::Cyan,
                                    Player::Computer => Color::Red,
                                };
                            }
                        }
                    }
                }

                // Highlight selected position
                if Some(pos) == self.selected_position {
                    bg_color = Some(Color::Yellow);
                }

                // Highlight legal moves
                if self.show_legal_moves && legal_move_set.contains(&pos) {
                    bg_color = Some(Color::Green);
                }

                let style = Style::default()
                    .fg(cell_color)
                    .bg(bg_color.unwrap_or(Color::Reset));

                line_spans.push(Span::styled(format!("{} ", cell_char), style));
            }

            if self.show_coordinates {
                line_spans.push(Span::styled(
                    format!(" {}", rank + 1),
                    Style::default().fg(Color::Gray),
                ));
            }

            board_lines.push(Line::from(line_spans));
        }

        // Footer with file labels (a-h)
        if self.show_coordinates {
            let footer = "   a b c d e f g h";
            board_lines.push(Line::from(footer));
        }

        let board_paragraph = Paragraph::new(board_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Game Board (TUI Debug View)"));

        frame.render_widget(board_paragraph, area);
    }

    fn render_info_panel(&self, frame: &mut Frame, area: Rect) {
        let mut info_lines = vec![
            Line::from("=== DEBUG INFO ==="),
            Line::from(format!("Terrain: {} | Pieces: {} | Effects: {} | Legal Moves: {} | Coords: {}",
                if self.show_terrain { "ON" } else { "OFF" },
                if self.show_pieces { "ON" } else { "OFF" },
                if self.show_effects { "ON" } else { "OFF" },
                if self.show_legal_moves { "ON" } else { "OFF" },
                if self.show_coordinates { "ON" } else { "OFF" },
            )),
        ];

        if let Some(pos) = self.selected_position {
            info_lines.push(Line::from(format!("Selected: {}", pos.to_chess_notation())));
            info_lines.push(Line::from(format!("Legal moves: {}", self.legal_moves.len())));
        } else {
            info_lines.push(Line::from("No piece selected"));
        }

        info_lines.push(Line::from(""));
        info_lines.push(Line::from("Controls: T=terrain, P=pieces, E=effects, L=legal moves, C=coords, Q=quit"));

        let info_paragraph = Paragraph::new(info_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Debug Controls"));

        frame.render_widget(info_paragraph, area);
    }
}

/// Run the TUI in standalone mode for debugging
pub fn run_debug_tui(board: Board) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut tui = DebugTui::new();
    
    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            tui.render(frame, &board, size);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('t') => tui.toggle_terrain(),
                KeyCode::Char('p') => tui.toggle_pieces(),
                KeyCode::Char('e') => tui.toggle_effects(),
                KeyCode::Char('l') => tui.toggle_legal_moves(),
                KeyCode::Char('c') => tui.toggle_coordinates(),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}