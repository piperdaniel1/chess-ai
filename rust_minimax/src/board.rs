use std::fmt::Error;

pub struct Board { 
    // Cells are represented as a 2D array of u8
    // 0,0 is the top left corner (a8)
    cells: [[u8; 8]; 8],

    // The current turn
    // True = white, False = black
    turn: bool,

    // En passant target square, if any
    en_passant: Option<Square>,

    // Castling rights
    // 0 = white king side, 1 = white queen side
    // 2 = black king side, 3 = black queen side
    castling: [bool; 4],

    // Halfmove clock
    // The number of halfmoves since the last capture or pawn advance
    halfmove_clock: u8,

    // Fullmove number
    // The number of the full move. 
    // It starts at 1, and is incremented after Black's move
    fullmove_number: u8,
}

struct Square { row: u8, col: u8 }
struct Move { from: Square, to: Square }

impl Move {
    fn new(from: Square, to: Square) -> Move {
        Move { from, to }
    }

    fn new_from_string(s: &str) -> Move {
        let from_square = Square::new_from_string(&s[0..2]);
        let to_square = Square::new_from_string(&s[2..4]);

        Move::new(from_square, to_square)
    }

    fn get_move_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.from.get_square_string());
        s.push_str(&self.to.get_square_string());
        s
    }
}

impl Square {
    fn new(row: u8, col: u8) -> Square {
        Square { row, col }
    }

    fn get_square_string(&self) -> String {
        let mut s = String::new();
        s.push((self.col + 97) as char);
        s.push((self.row + 49) as char);
        s
    }

    fn new_from_string(s: &str) -> Square {
        if s.len() != 2 {
            panic!("Invalid square string");
        }

        let mut chars = s.chars();
        let x = chars.next().unwrap() as u8 - 97;

        if x > 7 {
            panic!("Invalid square string");
        }

        let y = chars.next().unwrap() as u8 - 49;

        if y > 7 {
            panic!("Invalid square string");
        }

        Square { row: y, col: x }
    }
}

impl Board {
    pub fn new() -> Board {
        Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap()
    }

    pub fn new_empty() -> Board {
        Board {
            cells: [[0; 8]; 8],
            turn: true,
            en_passant: None,
            castling: [true, true, true, true],
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn new_from_fen(fen: &str) -> Result<Board, Error> {
        let mut board = Board::new_empty();
        let mut row = 0;
        let mut col = 0;

        let fen_parts: Vec<&str> = fen.split(' ').collect();

        // Put the pieces on the board
        for c in fen_parts[0].chars() {
            // This should not happen
            if c == ' ' {
                break;
            }

            // If we encounter a slash, we move to the next row
            if c == '/' {
                row += 1;
                col = 0;
                continue;
            }

            // If we encounter a number, we skip that many cells
            if c.is_digit(10) {
                col += c.to_digit(10).unwrap() as u8;
                continue;
            }

            if row > 7 || col > 7 {
                return Err(Error);
            }

            // If we encounter a letter, we put the piece on the board
            board.cells[row as usize][col as usize] = match Board::get_piece_from_char(c) {
                Ok(piece) => piece,
                _ => return Err(Error),
            };

            // Move to the next cell
            col += 1;
        }

        // Set the turn
        board.turn = match fen_parts[1] {
            "w" => true,
            "b" => false,
            _ => return Err(Error),
        };

        // Set the castling rights
        for c in fen_parts[2].chars() {
            match c {
                'K' => board.castling[0] = true,
                'Q' => board.castling[1] = true,
                'k' => board.castling[2] = true,
                'q' => board.castling[3] = true,
                _ => return Err(Error),
            }
        }

        // Set en passant target square
        if fen_parts[3] != "-" {
            board.en_passant = Some(Square::new_from_string(fen_parts[3]));
        }

        // Set the halfmove clock
        board.halfmove_clock = match fen_parts[4].parse() {
            Ok(n) => n,
            Err(_) => return Err(Error),
        };

        // Set the fullmove number
        board.fullmove_number = match fen_parts[5].parse() {
            Ok(n) => n,
            Err(_) => return Err(Error),
        };

        Ok(board)
    }
    
    // Setup the board to the starting position
    pub fn clear(&mut self) {
        self.cells = [[0; 8]; 8];
        self.turn = true;
        self.en_passant = None;
        self.castling = [true, true, true, true];
        self.halfmove_clock = 0;
        self.fullmove_number = 1;
    }

    // Import from fen string
    pub fn import_from_fen(&mut self, fen: &str) -> Result<(), Error> {
        self.clear();

        let mut row = 0;
        let mut col = 0;

        let fen_parts: Vec<&str> = fen.split(' ').collect();

        // Put the pieces on the board
        for c in fen_parts[0].chars() {
            // This should not happen
            if c == ' ' {
                break;
            }

            // If we encounter a slash, we move to the next row
            if c == '/' {
                row += 1;
                col = 0;
                continue;
            }

            // If we encounter a number, we skip that many cells
            if c.is_digit(10) {
                col += c.to_digit(10).unwrap() as u8;
                continue;
            }

            if row > 7 || col > 7 {
                return Err(Error);
            }

            // If we encounter a letter, we put the piece on the board
            self.cells[row as usize][col as usize] = match Board::get_piece_from_char(c) {
                Ok(piece) => piece,
                _ => return Err(Error),
            };

            // Move to the next cell
            col += 1;
        }

        // Set the turn
        self.turn = match fen_parts[1] {
            "w" => true,
            "b" => false,
            _ => return Err(Error),
        };

        // Set the castling rights
        for c in fen_parts[2].chars() {
            match c {
                'K' => self.castling[0] = true,
                'Q' => self.castling[1] = true,
                'k' => self.castling[2] = true,
                'q' => self.castling[3] = true,
                _ => return Err(Error),
            }
        }

        // Set en passant target square
        if fen_parts[3] != "-" {
            self.en_passant = Some(Square::new_from_string(fen_parts[3]));
        }

        // Set the halfmove clock
        self.halfmove_clock = match fen_parts[4].parse() {
            Ok(n) => n,
            Err(_) => return Err(Error),
        };

        // Set the fullmove number
        self.fullmove_number = match fen_parts[5].parse() {
            Ok(n) => n,
            Err(_) => return Err(Error),
        };

        Ok(())
    }

    fn get_piece_from_char(c: char) -> Result<u8, Error> {
        match c {
            'p' => Ok(12 as u8),
            'n' => Ok(11 as u8),
            'b' => Ok(10 as u8),
            'r' => Ok(9 as u8),
            'q' => Ok(8 as u8),
            'k' => Ok(7 as u8),
            'P' => Ok(6 as u8),
            'N' => Ok(5 as u8),
            'B' => Ok(4 as u8),
            'R' => Ok(3 as u8),
            'Q' => Ok(2 as u8),
            'K' => Ok(1 as u8),
            _ => Err(Error),
        }
    }

    pub fn setup(&mut self) {
        // Black major pieces
        self.cells[0][0] = 9;
        self.cells[0][1] = 11;
        self.cells[0][2] = 10;
        self.cells[0][3] = 8;
        self.cells[0][4] = 7;
        self.cells[0][5] = 10;
        self.cells[0][6] = 11;
        self.cells[0][7] = 9;

        // Black pawns
        self.cells[1][0] = 12;
        self.cells[1][1] = 12;
        self.cells[1][2] = 12;
        self.cells[1][3] = 12;
        self.cells[1][4] = 12;
        self.cells[1][5] = 12;
        self.cells[1][6] = 12;
        self.cells[1][7] = 12;

        // White major pieces
        self.cells[7][0] = 3;
        self.cells[7][1] = 5;
        self.cells[7][2] = 4;
        self.cells[7][3] = 2;
        self.cells[7][4] = 1;
        self.cells[7][5] = 4;
        self.cells[7][6] = 5;
        self.cells[7][7] = 3;

        // White pawns
        self.cells[6][0] = 6;
        self.cells[6][1] = 6;
        self.cells[6][2] = 6;
        self.cells[6][3] = 6;
        self.cells[6][4] = 6;
        self.cells[6][5] = 6;
        self.cells[6][6] = 6;
        self.cells[6][7] = 6;

        // Empty cells
        for i in 2..6 {
            for j in 0..8 {
                self.cells[i][j] = 0;
            }
        }

        // Assign turn
        self.turn = true;
    }

    pub fn print(&self) {
        println!("    A   B   C   D   E   F   G   H");
        println!("  +---+---+---+---+---+---+---+---+");
        let mut row_ind = 0;
        for row in self.cells.iter() {
            row_ind += 1;

            print!("{} |", 9-row_ind);
            for cell in row.iter() {
                print!(" ");

                match cell {
                    0 => print!(" "),
                    1 => print!("♚"),
                    2 => print!("♛"),
                    3 => print!("♜"),
                    4 => print!("♝"),
                    5 => print!("♞"),
                    6 => print!("♟"),
                    7 => print!("♔"),
                    8 => print!("♕"),
                    9 => print!("♖"),
                    10 => print!("♗"),
                    11 => print!("♘"),
                    12 => print!("♙"),
                    _ => print!("?"),
                }

                print!(" ");
                print!("|");
            }

            println!("");
            println!("  +---+---+---+---+---+---+---+---+");
        }

        println!("Turn: {}", if self.turn { "White" } else { "Black" });
        println!("Castling rights: {}", self.get_castling_rights());
        println!("En passant target square: {}", self.get_en_passant_square());
        println!("Halfmove clock: {}, Fullmove clock: {}", self.halfmove_clock, self.fullmove_number);
    }

    fn get_en_passant_square(&self) -> String {
        match &self.en_passant {
            Some(square) => square.get_square_string(),
            None => "-".to_string(),
        }
    }

    fn get_castling_rights(&self) -> String {
        let mut castling_rights = String::new();

        if self.castling[0] {
            castling_rights.push('K');
        }

        if self.castling[1] {
            castling_rights.push('Q');
        }

        if self.castling[2] {
            castling_rights.push('k');
        }

        if self.castling[3] {
            castling_rights.push('q');
        }

        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        castling_rights
    }

    pub fn fen(&self) -> String {
        let mut fen = String::new();

        // Piece placement
        for row in self.cells.iter() {
            let mut empty_count = 0;
            for cell in row.iter() {
                if *cell == 0 {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }

                    match cell {
                        1 => fen.push('K'),
                        2 => fen.push('Q'),
                        3 => fen.push('R'),
                        4 => fen.push('B'),
                        5 => fen.push('N'),
                        6 => fen.push('P'),
                        7 => fen.push('k'),
                        8 => fen.push('q'),
                        9 => fen.push('r'),
                        10 => fen.push('b'),
                        11 => fen.push('n'),
                        12 => fen.push('p'),
                        _ => panic!("Invalid piece"),
                    }
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

            fen.push('/');
        }

        fen.pop();

        // Active color
        fen.push(' ');
        if self.turn {
            fen.push('w');
        } else {
            fen.push('b');
        }

        // Castling availability
        fen.push(' ');
        if self.castling[0] {
            fen.push('K');
        }
        if self.castling[1] {
            fen.push('Q');
        }
        if self.castling[2] {
            fen.push('k');
        }
        if self.castling[3] {
            fen.push('q');
        }
        if !self.castling[0] && !self.castling[1] && !self.castling[2] && !self.castling[3] {
            fen.push('-');
        }

        // En passant target square
        fen.push(' ');
        if let Some(square) = &self.en_passant {
            fen.push_str(&square.get_square_string());
        } else {
            fen.push('-');
        }

        // Halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // Fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen() {
        // Tests creating a board using a FEN string
        let test_str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let board = Board::new_from_fen(test_str);

        let mut board = match board {
            Ok(b) => b,
            Err(_) => panic!("Error, invalid FEN provided."),
        };

        assert_eq!(board.fen(), test_str);

        // Tests changing an already existing board using a FEN string
        let test_str2 = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        board.import_from_fen(test_str2).unwrap();

        assert_eq!(board.fen(), test_str2);

        // Tests creating a new board with the default position
        let board = Board::new();
        assert_eq!(board.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn test_clear() {
        let mut board = Board::new();
        board.clear();
        assert_eq!(board.fen(), "8/8/8/8/8/8/8/8 w KQkq - 0 1");
    }
}
