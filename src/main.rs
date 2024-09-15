use std::io::{self, Write};

struct Position {
    i: usize,
    j: usize,
}

impl Position {
    fn new(i: usize, j: usize) -> Self {
        Position { i, j }
    }
}

struct ChessBoard {
    board: [[char; 8]; 8],
}

impl ChessBoard {
    fn new() -> Self {
        let board = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'], // Black pieces
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'], // Black pawns
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Empty row
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Empty row
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Empty row
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Empty row
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'], // White pawns
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'], // White pieces
        ];
        ChessBoard { board }
    }

    fn get_board(&self) -> &[[char; 8]; 8] {
        &self.board
    }

    fn set_board(&mut self, new_board: [[char; 8]; 8]) {
        self.board = new_board;
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
        if self.board[start.i][start.j] == ' ' {
            return;
        }

        // Move the piece
        let piece = self.board[start.i][start.j];
        self.board[destination.i][destination.j] = piece;
        self.board[start.i][start.j] = ' ';
    }
}

struct View;

impl View {
    fn new() -> Self {
        View
    }

    fn display(&self, board: &[[char; 8]; 8]) {
        let column_reference = ["a", "b", "c", "d", "e", "f", "g", "h"];
        println!(" : {:?}", column_reference);
        println!("{}", "-".repeat(50));
        for (i, row) in board.iter().enumerate() {
            let row_marker = 8 - i;
            println!("{}: {:?}", row_marker, row);
        }
    }
}

struct Controller {
    model: ChessBoard,
    view: View,
}

impl Controller {
    fn new() -> Self {
        Controller {
            model: ChessBoard::new(),
            view: View::new(),
        }
    }

    fn run(&mut self) {
        loop {
            self.view.display(self.model.get_board());
            print!("move (eg e2-e4): ");
            io::stdout().flush().unwrap();

            let mut move_input = String::new();
            io::stdin().read_line(&mut move_input).unwrap();
            let move_input = move_input.trim().to_lowercase();

            if move_input == "q" {
                break;
            }
            if move_input.is_empty() {
                continue;
            }

            let (start, destination) = self.parse_move(&move_input);
            self.model.move_piece(start, destination);
        }
    }

    fn parse_move(&self, move_str: &str) -> (Position, Position) {
        let column_reference = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let parts: Vec<&str> = move_str.split('-').collect();
        if parts.len() != 2 {
            return (Position::new(0, 0), Position::new(0, 0)); // Invalid move format
        }

        let (s, d) = (parts[0], parts[1]);

        let start_i = 8 - s[1..2].parse::<usize>().unwrap();
        let start_j = column_reference.iter().position(|&r| r == &s[0..1]).unwrap();

        let dest_i = 8 - d[1..2].parse::<usize>().unwrap();
        let dest_j = column_reference.iter().position(|&r| r == &d[0..1]).unwrap();

        (
            Position::new(start_i, start_j),
            Position::new(dest_i, dest_j),
        )
    }
}

fn main() {
    let mut controller = Controller::new();
    controller.run();
}