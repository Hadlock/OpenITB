use chrono::Local;
use lazy_static::lazy_static;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

const COLUMN_REFERENCE: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
const EMPTY_SQUARE: char = ' ';
const TILE_WIDTH: usize = 60;
const BOARD_WIDTH: usize = 8 * TILE_WIDTH;
const BOARD_HEIGHT: usize = BOARD_WIDTH;
const DATA_DIR: &str = "src/chess_data";

pub static FSBLUE: Color = Color::new(0.10, 0.20, 0.30, 1.00);

lazy_static! {
    static ref TILES: Mutex<HashMap<&'static str, Texture2D>> = Mutex::new(HashMap::new());
}

struct Position {
    i: usize,
    j: usize,
}

impl Position {
    fn new(i: usize, j: usize) -> Self {
        Position { i, j }
    }
}

struct Model {
    board: [[char; 8]; 8],
    updated: bool,
    move_history: Vec<String>,
}

impl Model {
    fn new() -> Self {
        let white_pieces = "RNBQKBNR";
        let white_pawns = "P".repeat(8);
        let black_pieces = white_pieces.to_lowercase();
        let black_pawns = white_pawns.to_lowercase();

        let mut board = [[' '; 8]; 8];

        board[0] = black_pieces.chars().collect::<Vec<char>>().try_into().unwrap();
        board[1] = black_pawns.chars().collect::<Vec<char>>().try_into().unwrap();
        board[6] = white_pawns.chars().collect::<Vec<char>>().try_into().unwrap();
        board[7] = white_pieces.chars().collect::<Vec<char>>().try_into().unwrap();

        Model { board, updated: true, move_history: Vec::new() }
    }

    fn get_board(&self) -> &[[char; 8]; 8] {
        &self.board
    }

    fn set_board(&mut self, new_board: [[char; 8]; 8]) {
        self.board = new_board;
        self.updated = true;
    }

    fn move_piece(&mut self, start: Position, destination: Position) {
        // Error checking
        for c in [&start, &destination] {
            if c.i > 7 || c.j > 7 || c.i < 0 || c.j < 0 {
                return;
            }
        }
        if start.i == destination.i && start.j == destination.j {
            return;
        }
        if self.board[start.i][start.j] == EMPTY_SQUARE {
            return;
        }

        // Move the piece
        let piece = self.board[start.i][start.j];
        self.board[destination.i][destination.j] = piece;
        self.board[start.i][start.j] = EMPTY_SQUARE;
        self.updated = true;

        // Record the move
        let now = Local::now();
        let move_record = format!(
            "{} {}{}-{}{}",
            now.format("%m/%d/%y %H:%M:%S"),
            COLUMN_REFERENCE[start.j],
            8 - start.i,
            COLUMN_REFERENCE[destination.j],
            8 - destination.i
        );
        self.move_history.push(move_record);
        if self.move_history.len() > 3 {
            self.move_history.remove(0);
        }
    }

    fn reset_update_flag(&mut self) {
        self.updated = false;
    }
}

struct View;

impl View {
    fn new() -> Self {
        View
    }

    fn display(&self, board: &[[char; 8]; 8], debug_board: bool) {
        // Draw the empty board
        self.draw_empty_board(debug_board);
        // Draw the pieces on top of the board
        if !debug_board {
            self.draw_pieces(board);
        }
    }

    fn print_board(&self, board: &[[char; 8]; 8]) {
        let now = Local::now();
        println!("\n{}", now.format("%m/%d/%y %H:%M:%S"));
        println!(" : {:?}", COLUMN_REFERENCE);
        println!("{}", "-".repeat(50));
        for (i, row) in board.iter().enumerate() {
            let row_marker = 8 - i;
            println!("{}: {:?}", row_marker, row);
        }
        // Print the input prompt at the bottom
        print!("move (eg e2-e4): ");
        io::stdout().flush().unwrap();
    }

    fn draw_row(&self, y: f32, first_tile_white: bool, debug_board: bool) {
        let remainder = if first_tile_white { 1 } else { 0 };
        let tiles = TILES.lock().unwrap();
        for i in 0..8 {
            let x = i as f32 * TILE_WIDTH as f32;
            let tile = if i % 2 == remainder {
                tiles.get("black_tile").unwrap()
            } else {
                tiles.get("white_tile").unwrap()
            };
            draw_texture(tile, x, y, WHITE);

            if debug_board {
                let text_pos = (x + TILE_WIDTH as f32 / 2.0, y + TILE_WIDTH as f32 / 2.0);
                let line_end = (x + TILE_WIDTH as f32 / 4.0, y + TILE_WIDTH as f32 / 4.0);
                draw_line(x, y, line_end.0, line_end.1, 2.0, RED);
                draw_text(&format!("({}, {})", x, y), text_pos.0, text_pos.1, 20.0, RED);
            }
        }
    }

    fn draw_empty_board(&self, debug_board: bool) {
        for i in 0..8 {
            let y = i as f32 * TILE_WIDTH as f32;
            let first_tile_white = !(i % 2 == 0);
            self.draw_row(y, first_tile_white, debug_board);
        }
    }

    fn draw_pieces(&self, board: &[[char; 8]; 8]) {
        let tiles = TILES.lock().unwrap();
        for (i, row) in board.iter().enumerate() {
            for (j, &piece) in row.iter().enumerate() {
                if piece != EMPTY_SQUARE {
                    let x = j as f32 * TILE_WIDTH as f32;
                    let y = i as f32 * TILE_WIDTH as f32;
                    let piece_str = piece.to_string();
                    if let Some(texture) = tiles.get(piece_str.as_str()) {
                        draw_texture(texture, x, y, WHITE);
                    }
                }
            }
        }
    }

    fn draw_move_history(&self, move_history: &[String], input_prompt: &str) {
        for (i, record) in move_history.iter().enumerate() {
            draw_text(
                record,
                10.0,
                (screen_height() - 30.0 - ((move_history.len() - 1 - i) as f32 * 20.0)),
                20.0,
                FSBLUE,
            );
        }
        // Draw the input prompt at the bottom
        draw_text(
            input_prompt,
            10.0,
            screen_height() - 10.0,
            20.0,
            FSBLUE,
        );
    }
}


struct Controller {
    model: Model,
    view: View,
    click_list: Vec<Position>,
}

impl Controller {
    fn new() -> Self {
        Controller {
            model: Model::new(),
            view: View::new(),
            click_list: Vec::new(),
        }
    }

    fn handle_input(&mut self, input: &str) {
        let (start, destination) = self.parse_move(input);
        self.model.move_piece(start, destination);
    }

    fn parse_move(&self, move_str: &str) -> (Position, Position) {
        let parts: Vec<&str> = move_str.split('-').collect();
        if parts.len() != 2 {
            return (Position::new(0, 0), Position::new(0, 0)); // Invalid move format
        }

        let (s, d) = (parts[0], parts[1]);

        let start_i = 8 - s[1..2].parse::<usize>().unwrap();
        let start_j = COLUMN_REFERENCE.iter().position(|&r| r == &s[0..1]).unwrap();

        let dest_i = 8 - d[1..2].parse::<usize>().unwrap();
        let dest_j = COLUMN_REFERENCE.iter().position(|&r| r == &d[0..1]).unwrap();

        (
            Position::new(start_i, start_j),
            Position::new(dest_i, dest_j),
        )
    }

    fn handle_click(&mut self, x: f32, y: f32) {
        let row = (y / TILE_WIDTH as f32).floor() as usize;
        let col = (x / TILE_WIDTH as f32).floor() as usize;

        if row < 8 && col < 8 {
            let position = Position::new(row, col);
            self.click_list.push(position);

            if self.click_list.len() % 2 == 0 {
                let start = &self.click_list[self.click_list.len() - 2];
                let destination = &self.click_list[self.click_list.len() - 1];
                let move_str = format!(
                    "{}{}-{}{}",
                    COLUMN_REFERENCE[start.j], 8 - start.i,
                    COLUMN_REFERENCE[destination.j], 8 - destination.i
                );
                self.handle_input(&move_str);
                self.click_list.clear();
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_width: BOARD_WIDTH as i32,
        window_height: BOARD_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load textures asynchronously
    let black_tile = load_texture(&format!("{}/black_tile.png", DATA_DIR)).await.unwrap();
    let white_tile = load_texture(&format!("{}/white_tile.png", DATA_DIR)).await.unwrap();
    let mut tiles = TILES.lock().unwrap();
    tiles.insert("black_tile", black_tile);
    tiles.insert("white_tile", white_tile);

    // Load piece textures
    let piece_names = [
        ("B", "chess_b451.png"),
        ("b", "chess_b45.png"),
        ("k", "chess_k45.png"),
        ("K", "chess_k451.png"),
        ("n", "chess_n45.png"),
        ("N", "chess_n451.png"),
        ("p", "chess_p45.png"),
        ("P", "chess_p451.png"),
        ("q", "chess_q45.png"),
        ("Q", "chess_q451.png"),
        ("r", "chess_r45.png"),
        ("R", "chess_r451.png"),
    ];

    for (name, file) in &piece_names {
        let texture = load_texture(&format!("{}/{}", DATA_DIR, file)).await.unwrap();
        tiles.insert(name, texture);
    }

    drop(tiles); // Release the lock

    let mut controller = Controller::new();
    let mut input_buffer = String::new();

    // Set up a channel for console input
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            tx.send(input.trim().to_string()).unwrap();
        }
    });

    loop {
        clear_background(WHITE);

        // Display the board and pieces
        controller.view.display(controller.model.get_board(), false);

        // Draw move history and input prompt
        controller.view.draw_move_history(&controller.model.move_history, "move (eg e2-e4): ");

        // Handle macroquad window input
        handle_macroquad_input(&mut controller, &mut input_buffer).await;

        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            controller.handle_click(mouse_x, mouse_y);
        }

        // Handle console input
        match rx.try_recv() {
            Ok(input) => {
                if input == "q" {
                    break;
                }
                controller.handle_input(&input);
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => break,
        }

        // Print to console only if the model has been updated
        if controller.model.updated {
            controller.view.print_board(controller.model.get_board());
            controller.model.reset_update_flag();
        }

        next_frame().await;
    }
}

async fn handle_macroquad_input(controller: &mut Controller, input_buffer: &mut String) {
    if is_key_pressed(KeyCode::Enter) {
        controller.handle_input(input_buffer.trim());
        input_buffer.clear();
    } else if let Some(c) = get_char_pressed() {
        input_buffer.push(c);
    }
}