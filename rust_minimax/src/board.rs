use std::fmt::Error;

const BLACK_TURN: bool = false;
const WHITE_TURN: bool = true;

const RANK_ONE: u8 = 7;
const RANK_TWO: u8 = 6;
const RANK_THREE: u8 = 5;
const RANK_FOUR: u8 = 4;
const RANK_FIVE: u8 = 3;
const RANK_SIX: u8 = 2;
const RANK_SEVEN: u8 = 1;
const RANK_EIGHT: u8 = 0;

const FILE_A: u8 = 0;
const FILE_B: u8 = 1;
const FILE_C: u8 = 2;
const FILE_D: u8 = 3;
const FILE_E: u8 = 4;
const FILE_F: u8 = 5;
const FILE_G: u8 = 6;
const FILE_H: u8 = 7;

const WHITE_KINGSIDE_CASTLE: usize = 0;
const WHITE_QUEENSIDE_CASTLE: usize = 1;
const BLACK_KINGSIDE_CASTLE: usize = 2;
const BLACK_QUEENSIDE_CASTLE: usize = 3;

const BLACK_PAWN: u8 = 12;
const BLACK_KNIGHT: u8 = 11;
const BLACK_BISHOP: u8 = 10;
const BLACK_ROOK: u8 = 9;
const BLACK_QUEEN: u8 = 8;
const BLACK_KING: u8 = 7;
const WHITE_PAWN: u8 = 6;
const WHITE_KNIGHT: u8 = 5;
const WHITE_BISHOP: u8 = 4;
const WHITE_ROOK: u8 = 3;
const WHITE_QUEEN: u8 = 2;
const WHITE_KING: u8 = 1;
const EMPTY_SQUARE: u8 = 0;
const OUT_OF_BOUNDS: u8 = 255;

const A8: Square = Square { row: 0, col: 0 };
const B8: Square = Square { row: 0, col: 1 };
const C8: Square = Square { row: 0, col: 2 };
const D8: Square = Square { row: 0, col: 3 };
const E8: Square = Square { row: 0, col: 4 };
const F8: Square = Square { row: 0, col: 5 };
const G8: Square = Square { row: 0, col: 6 };
const H8: Square = Square { row: 0, col: 7 };
const A7: Square = Square { row: 1, col: 0 };
const B7: Square = Square { row: 1, col: 1 };
const C7: Square = Square { row: 1, col: 2 };
const D7: Square = Square { row: 1, col: 3 };
const E7: Square = Square { row: 1, col: 4 };
const F7: Square = Square { row: 1, col: 5 };
const G7: Square = Square { row: 1, col: 6 };
const H7: Square = Square { row: 1, col: 7 };
const A6: Square = Square { row: 2, col: 0 };
const B6: Square = Square { row: 2, col: 1 };
const C6: Square = Square { row: 2, col: 2 };
const D6: Square = Square { row: 2, col: 3 };
const E6: Square = Square { row: 2, col: 4 };
const F6: Square = Square { row: 2, col: 5 };
const G6: Square = Square { row: 2, col: 6 };
const H6: Square = Square { row: 2, col: 7 };
const A5: Square = Square { row: 3, col: 0 };
const B5: Square = Square { row: 3, col: 1 };
const C5: Square = Square { row: 3, col: 2 };
const D5: Square = Square { row: 3, col: 3 };
const E5: Square = Square { row: 3, col: 4 };
const F5: Square = Square { row: 3, col: 5 };
const G5: Square = Square { row: 3, col: 6 };
const H5: Square = Square { row: 3, col: 7 };
const A4: Square = Square { row: 4, col: 0 };
const B4: Square = Square { row: 4, col: 1 };
const C4: Square = Square { row: 4, col: 2 };
const D4: Square = Square { row: 4, col: 3 };
const E4: Square = Square { row: 4, col: 4 };
const F4: Square = Square { row: 4, col: 5 };
const G4: Square = Square { row: 4, col: 6 };
const H4: Square = Square { row: 4, col: 7 };
const A3: Square = Square { row: 5, col: 0 };
const B3: Square = Square { row: 5, col: 1 };
const C3: Square = Square { row: 5, col: 2 };
const D3: Square = Square { row: 5, col: 3 };
const E3: Square = Square { row: 5, col: 4 };
const F3: Square = Square { row: 5, col: 5 };
const G3: Square = Square { row: 5, col: 6 };
const H3: Square = Square { row: 5, col: 7 };
const A2: Square = Square { row: 6, col: 0 };
const B2: Square = Square { row: 6, col: 1 };
const C2: Square = Square { row: 6, col: 2 };
const D2: Square = Square { row: 6, col: 3 };
const E2: Square = Square { row: 6, col: 4 };
const F2: Square = Square { row: 6, col: 5 };
const G2: Square = Square { row: 6, col: 6 };
const H2: Square = Square { row: 6, col: 7 };
const A1: Square = Square { row: 7, col: 0 };
const B1: Square = Square { row: 7, col: 1 };
const C1: Square = Square { row: 7, col: 2 };
const D1: Square = Square { row: 7, col: 3 };
const E1: Square = Square { row: 7, col: 4 };
const F1: Square = Square { row: 7, col: 5 };
const G1: Square = Square { row: 7, col: 6 };
const H1: Square = Square { row: 7, col: 7 };

pub struct Board { 
    // Cells are represented as a 2D array of u8
    // 0,0 is the top left corner (a8)
    cells: [[u8; 8]; 8],

    // The history of moves made
    history: Vec<PrevMove>,

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Square { row: u8, col: u8 }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move { from: Square, to: Square, promotion: Option<u8> }

// These PrevMove objects are used to keep track of the previous move
// with enough detail to undo it
struct PrevMove {
    // Stores the actual move
    inner_move: Move,

    // Stores the piece that was captured, if any
    captured_piece: Option<u8>,

    // Stores the en passant square that was present
    // before the move was made (if any)
    prev_en_passant: Option<Square>,

    // Stores the castling rights that were present
    // before the move was made
    prev_castling: [bool; 4],

    // Stores the halfmove clock that was present
    // before the move was made
    prev_halfmove_clock: u8,

    // Turn and full move number are not stored as they can be trivially
    // derived due to the nature of the game
}

fn get_piece_from_char(c: char) -> Result<u8, Error> {
    match c {
        'p' => Ok(BLACK_PAWN as u8),
        'n' => Ok(BLACK_KNIGHT as u8),
        'b' => Ok(BLACK_BISHOP as u8),
        'r' => Ok(BLACK_ROOK as u8),
        'q' => Ok(BLACK_QUEEN as u8),
        'k' => Ok(BLACK_KING as u8),
        'P' => Ok(WHITE_PAWN as u8),
        'N' => Ok(WHITE_KNIGHT as u8),
        'B' => Ok(WHITE_BISHOP as u8),
        'R' => Ok(WHITE_ROOK as u8),
        'Q' => Ok(WHITE_QUEEN as u8),
        'K' => Ok(WHITE_KING as u8),
        _ => Err(Error),
    }
}

impl Move {
    fn new(from: Square, to: Square, promotion: Option<u8>) -> Move {
        Move { from, to, promotion }
    }

    fn new_from_string(s: &str) -> Result<Move, Error> {
        let from_square = Square::new_from_string(&s[0..2]);
        let to_square = Square::new_from_string(&s[2..4]);
        let promotion = match s.len() {
            5 => {
                // The .unwrap() will never fail because we just checked the length
                // and made sure it was 5
                let promotion_piece = get_piece_from_char(s.chars().nth(4).unwrap());
                match promotion_piece {
                    Ok(p) => Some(p),
                    Err(e) => return Err(e),
                }
            },
            _ => None,
        };

        Ok(Move::new(from_square, to_square, promotion))
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
        s.push(((7-self.row) + 49) as char);
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

        let y = 7 - (chars.next().unwrap() as u8 - 49);

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
            cells: [[EMPTY_SQUARE; 8]; 8],
            history: Vec::new(),
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
            board.cells[row as usize][col as usize] = match get_piece_from_char(c) {
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
        self.cells = [[EMPTY_SQUARE; 8]; 8];
        self.history = Vec::new();
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

            // If we ever go off the board, return an error because
            // the fen string must have been invalid
            if row > 7 || col > 7 {
                return Err(Error);
            }

            // If we encounter a letter, we put the piece on the board
            self.cells[row as usize][col as usize] = match get_piece_from_char(c) {
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

    pub fn setup(&mut self) {
        // Black major pieces
        self.cells[0][0] = BLACK_ROOK;
        self.cells[0][1] = BLACK_KNIGHT;
        self.cells[0][2] = BLACK_BISHOP;
        self.cells[0][3] = BLACK_QUEEN;
        self.cells[0][4] = BLACK_KING;
        self.cells[0][5] = BLACK_BISHOP;
        self.cells[0][6] = BLACK_KNIGHT;
        self.cells[0][7] = BLACK_ROOK;

        // Black pawns
        self.cells[1][0] = BLACK_PAWN;
        self.cells[1][1] = BLACK_PAWN;
        self.cells[1][2] = BLACK_PAWN;
        self.cells[1][3] = BLACK_PAWN;
        self.cells[1][4] = BLACK_PAWN;
        self.cells[1][5] = BLACK_PAWN;
        self.cells[1][6] = BLACK_PAWN;
        self.cells[1][7] = BLACK_PAWN;

        // White major pieces
        self.cells[7][0] = WHITE_ROOK;
        self.cells[7][1] = WHITE_KNIGHT;
        self.cells[7][2] = WHITE_BISHOP;
        self.cells[7][3] = WHITE_QUEEN;
        self.cells[7][4] = WHITE_KING;
        self.cells[7][5] = WHITE_BISHOP;
        self.cells[7][6] = WHITE_KNIGHT;
        self.cells[7][7] = WHITE_ROOK;

        // White pawns
        self.cells[6][0] = WHITE_PAWN;
        self.cells[6][1] = WHITE_PAWN;
        self.cells[6][2] = WHITE_PAWN;
        self.cells[6][3] = WHITE_PAWN;
        self.cells[6][4] = WHITE_PAWN;
        self.cells[6][5] = WHITE_PAWN;
        self.cells[6][6] = WHITE_PAWN;
        self.cells[6][7] = WHITE_PAWN;

        // Empty cells
        for i in 2..6 {
            for j in 0..8 {
                self.cells[i][j] = EMPTY_SQUARE;
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

            print!("{} |", 9 - row_ind);
            for cell in row.iter() {
                print!(" ");

                match cell {
                    &EMPTY_SQUARE => print!(" "),
                    &WHITE_KING => print!("♚"),
                    &WHITE_QUEEN => print!("♛"),
                    &WHITE_ROOK => print!("♜"),
                    &WHITE_BISHOP => print!("♝"),
                    &WHITE_KNIGHT => print!("♞"),
                    &WHITE_PAWN => print!("♟"),
                    &BLACK_KING => print!("♔"),
                    &BLACK_QUEEN => print!("♕"),
                    &BLACK_ROOK => print!("♖"),
                    &BLACK_BISHOP => print!("♗"),
                    &BLACK_KNIGHT => print!("♘"),
                    &BLACK_PAWN => print!("♙"),
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

    fn set_square(&mut self, square: &Square, piece: u8) {
        self.cells[square.row as usize][square.col as usize] = piece;
    }

    fn get_square(&self, square: &Square) -> u8 {
        if square.row > 7 || square.col > 7 {
            return 255;
        }

        self.cells[square.row as usize][square.col as usize]
    }

    // This can be useful if you want to check a square
    // that may not actually exist
    fn get_square_ind(&self, row: i8, col: i8) -> u8 {
        if row < 0 || row > 7 || col < 0 || col > 7 {
            return 255;
        }

        self.cells[row as usize][col as usize]
    }

    fn execute_move(&mut self, mv: &Move) -> Result<PrevMove, Error> {
        // TODO check if move is legal

        let from_piece = match self.get_square(&mv.from) {
            EMPTY_SQUARE => return Err(Error),
            piece => piece,
        };

        let to_piece = match self.get_square(&mv.to) {
            EMPTY_SQUARE => None,
            piece => Some(piece),
        };

        // This struct does not generally mutate except if the move is a promotion
        // This cannot be efficiently determined before we create the struct, so
        // we create most of the struct here and then mutate it if necessary
        // It may also change if we capture with en passant, as that is not
        // determined until later as well
        let mut prev_move = PrevMove {
            inner_move: Move {
                from: mv.from,
                to: mv.to,
                promotion: None,
            },
            captured_piece: to_piece,
            prev_castling: self.castling.clone(),
            prev_en_passant: self.en_passant.clone(),
            prev_halfmove_clock: self.halfmove_clock,
        };

        // Set the square we are leaving to empty
        self.set_square(&mv.from, EMPTY_SQUARE);

        // Set the square we are moving to to the piece we are moving
        self.set_square(&mv.to, from_piece);

        //
        // HERE COME THE EDGE CASES
        //

        // If this was a castling move, move the rook as well
        if from_piece == WHITE_KING {
            // White king side castling
            if mv.from == E1 && mv.to == G1 {
                self.set_square(&H1, EMPTY_SQUARE);
                self.set_square(&F1, WHITE_ROOK);
            } else if mv.from == E1 && mv.to == C1 {
                self.set_square(&A1, EMPTY_SQUARE);
                self.set_square(&D1, WHITE_ROOK);
            }

            // Once the king has moved, castling is no longer possible
            self.castling[WHITE_KINGSIDE_CASTLE] = false;
            self.castling[WHITE_QUEENSIDE_CASTLE] = false;
        } else if from_piece == BLACK_KING {
            // Black king side castling
            if mv.from == E8 && mv.to == G8 {
                self.set_square(&H8, EMPTY_SQUARE);
                self.set_square(&F8, BLACK_ROOK);
            } else if mv.from == E8 && mv.to == C8 {
                self.set_square(&A8, EMPTY_SQUARE);
                self.set_square(&D8, BLACK_ROOK);
            }

            // Once the king has moved, castling is no longer possible
            self.castling[BLACK_KINGSIDE_CASTLE] = false;
            self.castling[BLACK_QUEENSIDE_CASTLE] = false;
        }

        // If this was a rook move, castling is no longer possible on that side
        if from_piece == WHITE_ROOK {
            if mv.from == A1 {
                self.castling[WHITE_QUEENSIDE_CASTLE] = false;
            } else if mv.from == H1 {
                self.castling[WHITE_KINGSIDE_CASTLE] = false;
            }
        } else if from_piece == BLACK_ROOK {
            if mv.from == A8 {
                self.castling[BLACK_QUEENSIDE_CASTLE] = false;
            } else if mv.from == H8 {
                self.castling[BLACK_KINGSIDE_CASTLE] = false;
            }
        }

        // If this was a pawn move, reset the halfmove clock
        if from_piece == WHITE_PAWN || from_piece == BLACK_PAWN {
            self.halfmove_clock = 0;
        // Otherwise, increment the halfmove clock
        } else {
            self.halfmove_clock += 1;
        }

        // If this was an en passant capture, remove the captured pawn
        // Also, set the captured piece field of the PrevMove struct
        // We specifically check for this before we check if we should update
        // the en passant square, as that check will clear the en passant square
        if self.en_passant.is_some() {
            // We know that the en passant square is not empty, so we can unwrap
            if from_piece == WHITE_PAWN && mv.to == self.en_passant.unwrap() {
                self.set_square(&Square {
                    row: RANK_FIVE,
                    col: mv.to.col,
                }, EMPTY_SQUARE);

                prev_move.captured_piece = Some(BLACK_PAWN);
            } else if from_piece == BLACK_PAWN && mv.to == self.en_passant.unwrap() {
                self.set_square(&Square {
                    row: RANK_FOUR,
                    col: mv.to.col,
                }, EMPTY_SQUARE);

                prev_move.captured_piece = Some(WHITE_PAWN);
            }
        }

        // If this was a two square pawn move, set the en passant target square
        if from_piece == WHITE_PAWN
            && mv.from.row == RANK_TWO && mv.to.row == RANK_FOUR {

            self.en_passant = Some(Square {
                row: RANK_THREE,
                col: mv.to.col,
            });
        } else if from_piece == BLACK_PAWN
            && mv.from.row == RANK_SEVEN && mv.to.row == RANK_FIVE {

            self.en_passant = Some(Square {
                row: RANK_SIX,
                col: mv.to.col,
            });
        // Otherwise, clear the en passant target square
        } else {
            self.en_passant = None;
        }

        // If this was a promotion, promote the pawn
        // Also, set the promotion field of the PrevMove struct
        if let Some(promotion) = mv.promotion {
            self.set_square(&mv.to, promotion);
            prev_move.inner_move.promotion = Some(promotion);
        }

        // If this was a capture, reset the halfmove clock
        // We check the prev_move struct to account for en passant captures
        if prev_move.captured_piece.is_some() {
            self.halfmove_clock = 0;
        }

        // If this was a pawn move, reset the halfmove clock
        if from_piece == WHITE_PAWN || from_piece == BLACK_PAWN {
            self.halfmove_clock = 0;
        }

        // Advance the fullmove counter if it is black's turn
        if self.turn == BLACK_TURN {
            self.fullmove_number += 1;
        }

        // Switch the turn
        self.turn = !self.turn;

        Ok(prev_move)
    }

    // Public function to push a move onto the move stack
    // and execute it on the board
    pub fn push(&mut self, new_move: Move) -> Result<(), Error> {
        let prev_move = self.execute_move(&new_move);

        let prev_move = match prev_move {
            Ok(prev_move) => prev_move,
            Err(_) => return Err(Error),
        };

        self.history.push(prev_move);

        Ok(())
    }

    fn reverse_move(&mut self, prev_move: &PrevMove) -> Result<(), Error> {
        // Get the piece that was moved
        let moved_piece = match self.get_square(&prev_move.inner_move.to) {
            0 => return Err(Error),
            piece => piece,
        };

        // Move the piece back
        self.set_square(&prev_move.inner_move.from, moved_piece);

        //
        // HERE COME THE EDGE CASES
        //

        // If this was a castling move, move the rook back as well
        if moved_piece == WHITE_KING {
            // White king side castling
            if prev_move.inner_move.from == E1 && prev_move.inner_move.to == G1 {
                self.set_square(&H1, WHITE_ROOK);
                self.set_square(&F1, 0);
            } else if prev_move.inner_move.from == E1 && prev_move.inner_move.to == C1 {
                self.set_square(&A1, WHITE_ROOK);
                self.set_square(&D1, 0);
            }
        } else if moved_piece == BLACK_KING {
            // Black king side castling
            if prev_move.inner_move.from == E8 && prev_move.inner_move.to == G8 {
                self.set_square(&H8, BLACK_ROOK);
                self.set_square(&F8, 0);
            } else if prev_move.inner_move.from == E8 && prev_move.inner_move.to == C8 {
                self.set_square(&A8, BLACK_ROOK);
                self.set_square(&D8, 0);
            }
        }

        // If this was a promotion, demote the piece
        if let Some(promotion) = prev_move.inner_move.promotion {
            self.set_square(&prev_move.inner_move.from, match promotion {
                WHITE_QUEEN => WHITE_PAWN,
                WHITE_ROOK => WHITE_PAWN,
                WHITE_BISHOP => WHITE_PAWN,
                WHITE_KNIGHT => WHITE_PAWN,
                BLACK_QUEEN => BLACK_PAWN,
                BLACK_ROOK => BLACK_PAWN,
                BLACK_BISHOP => BLACK_PAWN,
                BLACK_KNIGHT => BLACK_PAWN,
                _ => return Err(Error),
            });
        }

        // Restore the captured piece as long as there was not an en passant capture
        
        // We know that there was an en passant capture IF

        let mut en_passant_capture = false;
        // 1. There was an en passant square
        if let Some(en_passant) = prev_move.prev_en_passant {
            // 2. The piece moved to the en passant square
            if prev_move.inner_move.to == en_passant {
                // 3. The piece moved was a pawn
                if self.get_square(&prev_move.inner_move.from) == WHITE_PAWN {
                    self.set_square(&Square {
                        row: RANK_FIVE,
                        col: prev_move.inner_move.to.col,
                    }, BLACK_PAWN);
                    
                    // Remove the pawn from the en passant square
                    self.set_square(&prev_move.inner_move.to, EMPTY_SQUARE);

                    en_passant_capture = true;
                } else if self.get_square(&prev_move.inner_move.from) == BLACK_PAWN {
                    // Restore the captured piece
                    self.set_square(&Square {
                        row: RANK_FOUR,
                        col: prev_move.inner_move.to.col,
                    }, WHITE_PAWN);

                    // Remove the pawn from the en passant square
                    self.set_square(&prev_move.inner_move.to, EMPTY_SQUARE);

                    en_passant_capture = true;
                }
            }
        }

        // If this was not an en passant capture, restore the captured piece the
        // normal way
        if !en_passant_capture {
            if let Some(captured_piece) = prev_move.captured_piece {
                self.set_square(&prev_move.inner_move.to, captured_piece);
            } else {
                self.set_square(&prev_move.inner_move.to, 0);
            }
        }

        // Restore the en passant square
        self.en_passant = prev_move.prev_en_passant;

        // Restore the half move clock
        self.halfmove_clock = prev_move.prev_halfmove_clock;

        // Restore the full move number as long as we are reversing a black move
        if self.turn {
            self.fullmove_number -= 1;
        }

        // Switch the turn
        self.turn = !self.turn;

        // Restore the castling rights
        self.castling = prev_move.prev_castling;

        Ok(())
    }

    // Public function to pop a move off the move stack
    // and undo it on the board
    pub fn pop(&mut self) -> Result<Move, Error> {
        let prev_move = match self.history.pop() {
            Some(prev_move) => prev_move,
            None => return Err(Error),
        };

        match self.reverse_move(&prev_move) {
            Ok(_) => Ok(prev_move.inner_move),
            Err(_) => Err(Error),
        }
    }

    fn is_white(&self, piece: u8) -> bool {
        if piece > 0 && piece < 7 {
            return true;
        }

        return false;
    }

    fn is_black(&self, piece: u8) -> bool {
        if piece > 6 && piece < 13 {
            return true;
        }

        return false;
    }

    // Adds all the possible promotion variants to the move list for a given
    // move. Does not check if the move is legal.
    fn bundle_promotions(&self, moves: &mut Vec<Move>, new_move: Move) {
        if new_move.to.row == RANK_EIGHT {
            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(WHITE_QUEEN),
            });

            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(WHITE_ROOK),
            });

            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(WHITE_BISHOP),
            });

            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(WHITE_KNIGHT),
            });
        } else if new_move.to.row == RANK_ONE {
            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(BLACK_QUEEN),
            });

            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(BLACK_ROOK),
            });

            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(BLACK_BISHOP),
            });

            moves.push(Move {
                from: new_move.from,
                to: new_move.to,
                promotion: Some(BLACK_KNIGHT),
            });
        } else {
            moves.push(new_move);
        }
    }

    // Adds all the pseudo-legal possible moves that the pawn at the indicated square can
    // make to the move list. TODO it might be good to write this in a way that does
    // not have as much code duplication.
    fn add_pawn_moves(&self, moves: &mut Vec<Move>, square: Square) {
        let target_piece = self.get_square(&square);

        // Get the direction the pawn is moving
        if target_piece == WHITE_PAWN {
            let direction: u8 = 1;

            // Check if the pawn can move forward one square
            if self.get_square(&Square {
                row: square.row - direction,
                col: square.col,
            }) == EMPTY_SQUARE {
                self.bundle_promotions(moves, Move {
                    from: square,
                    to: Square {
                        row: square.row - direction,
                        col: square.col,
                    },
                    promotion: None,
                });

                // Check if the pawn can move forward two squares
                // We nest this check inside the first check to avoid
                // checking if the space in front of the pawn is empty
                if square.row == RANK_TWO {
                    if self.get_square(&Square {
                        row: square.row - direction * 2,
                        col: square.col,
                    }) == EMPTY_SQUARE {
                        // We know that this won't be a promotion move
                        moves.push(Move {
                            from: square,
                            to: Square {
                                row: square.row - direction * 2,
                                col: square.col,
                            },
                            promotion: None,
                        });
                    }
                }
            }

            // Check if the pawn can capture diagonally left
            let potential_capture = self.get_square_ind(
                square.row as i8 - direction as i8,
                square.col as i8 - 1);

            if potential_capture != OUT_OF_BOUNDS {
                if potential_capture != EMPTY_SQUARE
                   && self.is_black(potential_capture) {
                    self.bundle_promotions(moves, Move {
                        from: square,
                        to: Square {
                            row: square.row - direction,
                            col: square.col - 1,
                        },
                        promotion: None,
                    });
                }
            }

            // Check if the pawn can capture diagonally right
            let potential_capture = self.get_square_ind(
                square.row as i8 - direction as i8,
                square.col as i8 + 1);

            if potential_capture != OUT_OF_BOUNDS {
                if potential_capture != EMPTY_SQUARE
                   && self.is_black(potential_capture) {
                    self.bundle_promotions(moves, Move {
                        from: square,
                        to: Square {
                            row: square.row - direction,
                            col: square.col + 1,
                        },
                        promotion: None,
                    });
                }
            }
        } else if target_piece == BLACK_PAWN {
            let direction: u8 = 1;

            // Check if the pawn can move forward one square
            if self.get_square(&Square {
                row: square.row + direction,
                col: square.col,
            }) == EMPTY_SQUARE {
                self.bundle_promotions(moves, Move {
                    from: square,
                    to: Square {
                        row: square.row + direction,
                        col: square.col,
                    },
                    promotion: None,
                });

                // Check if the pawn can move forward two squares
                // We nest this check inside the first check to avoid
                // checking if the space in front of the pawn is empty
                if square.row == RANK_SEVEN {
                    if self.get_square(&Square {
                        row: square.row + direction * 2,
                        col: square.col,
                    }) == EMPTY_SQUARE {
                        // We know that this won't be a promotion move
                        moves.push(Move {
                            from: square,
                            to: Square {
                                row: square.row + direction * 2,
                                col: square.col,
                            },
                            promotion: None,
                        });
                    }
                }
            }

            // Check if the pawn can capture diagonally left
            let potential_capture = self.get_square(&Square {
                row: square.row + direction,
                col: square.col - 1,
            });

            if potential_capture != OUT_OF_BOUNDS {
                if potential_capture != EMPTY_SQUARE
                   && self.is_black(potential_capture) {
                    self.bundle_promotions(moves, Move {
                        from: square,
                        to: Square {
                            row: square.row + direction,
                            col: square.col - 1,
                        },
                        promotion: None,
                    });
                }
            }

            // Check if the pawn can capture diagonally right
            let potential_capture = self.get_square(&Square {
                row: square.row + direction,
                col: square.col + 1,
            });

            if potential_capture != OUT_OF_BOUNDS {
                if potential_capture != EMPTY_SQUARE
                   && self.is_black(potential_capture) {
                    self.bundle_promotions(moves, Move {
                        from: square,
                        to: Square {
                            row: square.row + direction,
                            col: square.col + 1,
                        },
                        promotion: None,
                    });
                }
            }
        } else {
            panic!("add_pawn_moves called on a non-pawn piece");
        }
    }

    // Private function to generate all pseudo-legal moves
    // for the current position. This function takes all the rules into
    // account except for any rules involving check on the king.
    fn gen_psuedo_legal_moves() -> Vec<Move> {
        // TODO
        let moves: Vec<Move> = Vec::new();

        moves
    }

    // Private function that removes all pseudo-legal moves that
    // would leave the king in check
    fn trim_illegal_moves(moves: Vec<Move>) -> Vec<Move> {
        // TODO
        return moves;
    }

    // Public function to get a vector of all legal moves
    pub fn legal_moves() -> Vec<Move> {
        // TODO
        Vec::new()
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

    #[test]
    fn test_simple_push_pop() {
        let mut board = Board::new();

        let starting_fen = board.fen();

        // Push a move onto the stack
        board.push(Move::new_from_string("e2e4").unwrap()).unwrap();

        // Pop the move off the stack
        board.pop().unwrap();

        // Check that the board is back to the default position
        assert_eq!(board.fen(), starting_fen);
    }

    #[test]
    fn test_complex_push_pop() {
        let mut board = Board::new();

        // The moves from the legendary game between Bobby Fischer and Donald Byrne
        // in the 1956 "Match of the Century" in New York City: https://www.chessgames.com/perl/chessgame?gid=1008361

        // Tests most of the special cases in the push and pop functions

        let current_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("g1f3").unwrap()).unwrap();
        board.push(Move::new_from_string("g8f6").unwrap()).unwrap();
        board.push(Move::new_from_string("c2c4").unwrap()).unwrap();

        // This tests the en passant square
        let current_fen = "rnbqkb1r/pppppppp/5n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq c3 0 2";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("g7g6").unwrap()).unwrap();
        board.push(Move::new_from_string("b1c3").unwrap()).unwrap();
        board.push(Move::new_from_string("f8g7").unwrap()).unwrap();
        board.push(Move::new_from_string("d2d4").unwrap()).unwrap();
        board.push(Move::new_from_string("e8g8").unwrap()).unwrap();
        board.push(Move::new_from_string("c1f4").unwrap()).unwrap();

        let current_fen = "rnbq1rk1/ppppppbp/5np1/8/2PP1B2/2N2N2/PP2PPPP/R2QKB1R b KQ - 2 5";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("d7d5").unwrap()).unwrap();
        board.push(Move::new_from_string("d1b3").unwrap()).unwrap();
        board.push(Move::new_from_string("d5c4").unwrap()).unwrap();
        board.push(Move::new_from_string("b3c4").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c6").unwrap()).unwrap();
        board.push(Move::new_from_string("e2e4").unwrap()).unwrap();
        board.push(Move::new_from_string("b8d7").unwrap()).unwrap();
        board.push(Move::new_from_string("a1d1").unwrap()).unwrap();
        board.push(Move::new_from_string("d7b6").unwrap()).unwrap();
        board.push(Move::new_from_string("c4c5").unwrap()).unwrap();

        let current_fen = "r1bq1rk1/pp2ppbp/1np2np1/2Q5/3PPB2/2N2N2/PP3PPP/3RKB1R b K - 4 10";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("c8g4").unwrap()).unwrap();
        board.push(Move::new_from_string("f4g5").unwrap()).unwrap();
        board.push(Move::new_from_string("b6a4").unwrap()).unwrap();
        board.push(Move::new_from_string("c5a3").unwrap()).unwrap();
        board.push(Move::new_from_string("a4c3").unwrap()).unwrap();
        board.push(Move::new_from_string("b2c3").unwrap()).unwrap();
        board.push(Move::new_from_string("f6e4").unwrap()).unwrap();
        board.push(Move::new_from_string("g5e7").unwrap()).unwrap();
        board.push(Move::new_from_string("d8b6").unwrap()).unwrap();
        board.push(Move::new_from_string("f1c4").unwrap()).unwrap();

        let current_fen = "r4rk1/pp2Bpbp/1qp3p1/8/2BPn1b1/Q1P2N2/P4PPP/3RK2R b K - 2 15";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("e4c3").unwrap()).unwrap();
        board.push(Move::new_from_string("e7c5").unwrap()).unwrap();
        board.push(Move::new_from_string("f8e8").unwrap()).unwrap();
        board.push(Move::new_from_string("e1f1").unwrap()).unwrap();
        board.push(Move::new_from_string("g4e6").unwrap()).unwrap();
        board.push(Move::new_from_string("c5b6").unwrap()).unwrap();
        board.push(Move::new_from_string("e6c4").unwrap()).unwrap();
        board.push(Move::new_from_string("f1g1").unwrap()).unwrap();
        board.push(Move::new_from_string("c3e2").unwrap()).unwrap();
        board.push(Move::new_from_string("g1f1").unwrap()).unwrap();

        let current_fen = "r3r1k1/pp3pbp/1Bp3p1/8/2bP4/Q4N2/P3nPPP/3R1K1R b - - 3 20";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("e2d4").unwrap()).unwrap();
        board.push(Move::new_from_string("f1g1").unwrap()).unwrap();
        board.push(Move::new_from_string("d4e2").unwrap()).unwrap();
        board.push(Move::new_from_string("g1f1").unwrap()).unwrap();
        board.push(Move::new_from_string("e2c3").unwrap()).unwrap();
        board.push(Move::new_from_string("f1g1").unwrap()).unwrap();
        board.push(Move::new_from_string("a7b6").unwrap()).unwrap();
        board.push(Move::new_from_string("a3b4").unwrap()).unwrap();
        board.push(Move::new_from_string("a8a4").unwrap()).unwrap();
        board.push(Move::new_from_string("b4b6").unwrap()).unwrap();

        let current_fen = "4r1k1/1p3pbp/1Qp3p1/8/r1b5/2n2N2/P4PPP/3R2KR b - - 0 25";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("c3d1").unwrap()).unwrap();
        board.push(Move::new_from_string("h2h3").unwrap()).unwrap();
        board.push(Move::new_from_string("a4a2").unwrap()).unwrap();
        board.push(Move::new_from_string("g1h2").unwrap()).unwrap();
        board.push(Move::new_from_string("d1f2").unwrap()).unwrap();
        board.push(Move::new_from_string("h1e1").unwrap()).unwrap();
        board.push(Move::new_from_string("e8e1").unwrap()).unwrap();
        board.push(Move::new_from_string("b6d8").unwrap()).unwrap();
        board.push(Move::new_from_string("g7f8").unwrap()).unwrap();
        board.push(Move::new_from_string("f3e1").unwrap()).unwrap();

        let current_fen = "3Q1bk1/1p3p1p/2p3p1/8/2b5/7P/r4nPK/4N3 b - - 0 30";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("c4d5").unwrap()).unwrap();
        board.push(Move::new_from_string("e1f3").unwrap()).unwrap();
        board.push(Move::new_from_string("f2e4").unwrap()).unwrap();
        board.push(Move::new_from_string("d8b8").unwrap()).unwrap();
        board.push(Move::new_from_string("b7b5").unwrap()).unwrap();
        board.push(Move::new_from_string("h3h4").unwrap()).unwrap();
        board.push(Move::new_from_string("h7h5").unwrap()).unwrap();
        board.push(Move::new_from_string("f3e5").unwrap()).unwrap();
        board.push(Move::new_from_string("g8g7").unwrap()).unwrap();
        board.push(Move::new_from_string("h2g1").unwrap()).unwrap();

        let current_fen = "1Q3b2/5pk1/2p3p1/1p1bN2p/4n2P/8/r5P1/6K1 b - - 3 35";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("f8c5").unwrap()).unwrap();
        board.push(Move::new_from_string("g1f1").unwrap()).unwrap();
        board.push(Move::new_from_string("e4g3").unwrap()).unwrap();
        board.push(Move::new_from_string("f1e1").unwrap()).unwrap();
        board.push(Move::new_from_string("c5b4").unwrap()).unwrap();
        board.push(Move::new_from_string("e1d1").unwrap()).unwrap();
        board.push(Move::new_from_string("d5b3").unwrap()).unwrap();
        board.push(Move::new_from_string("d1c1").unwrap()).unwrap();
        board.push(Move::new_from_string("g3e2").unwrap()).unwrap();
        board.push(Move::new_from_string("c1b1").unwrap()).unwrap();
        board.push(Move::new_from_string("e2c3").unwrap()).unwrap();
        board.push(Move::new_from_string("b1c1").unwrap()).unwrap();
        board.push(Move::new_from_string("a2c2").unwrap()).unwrap();

        let current_fen = "1Q6/5pk1/2p3p1/1p2N2p/1b5P/1bn5/2r3P1/2K5 w - - 16 42";
        assert_eq!(board.fen(), current_fen);

        // Pop them all off the stack
        loop {
            if let Err(_) = board.pop() {
                break;
            }
        }

        // Check that the board is back to the default position
        let current_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(board.fen(), current_fen);
    }

    #[test]
    fn test_enpassant_capture() {
        let mut board = Board::new();

        board.push(Move::new_from_string("d2d4").unwrap()).unwrap();
        board.push(Move::new_from_string("a7a6").unwrap()).unwrap();
        board.push(Move::new_from_string("d4d5").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c5").unwrap()).unwrap();
        board.push(Move::new_from_string("d5c6").unwrap()).unwrap();

        let current_fen = "rnbqkbnr/1p1ppppp/p1P5/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 3";
        assert_eq!(board.fen(), current_fen);

        loop {
            if let Err(_) = board.pop() {
                break;
            }
        }

        // Check that the board is back to the default position
        let current_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(board.fen(), current_fen);
    }

    #[test]
    fn test_pawn_queen_promotion() {
        let mut board = Board::new();

        board.push(Move::new_from_string("d2d4").unwrap()).unwrap();
        board.push(Move::new_from_string("a7a6").unwrap()).unwrap();
        board.push(Move::new_from_string("d4d5").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c5").unwrap()).unwrap();
        board.push(Move::new_from_string("d5c6").unwrap()).unwrap();
        board.push(Move::new_from_string("d7d6").unwrap()).unwrap();
        board.push(Move::new_from_string("c2c3").unwrap()).unwrap();
        board.push(Move::new_from_string("c8e6").unwrap()).unwrap();
        board.push(Move::new_from_string("c6c7").unwrap()).unwrap();
        board.push(Move::new_from_string("d8d7").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c8Q").unwrap()).unwrap();

        // Make sure the promotion worked
        let current_fen = "rnQ1kbnr/1p1qpppp/p2pb3/8/8/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 6";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("d7d8").unwrap()).unwrap();
        board.push(Move::new_from_string("c8d8").unwrap()).unwrap();
        board.push(Move::new_from_string("e8d8").unwrap()).unwrap();

        // Make sure we got to the right final position
        let current_fen = "rn1k1bnr/1p2pppp/p2pb3/8/8/2P5/PP2PPPP/RNBQKBNR w KQ - 0 8";
        assert_eq!(board.fen(), current_fen);

        // Reverse all the moves
        loop {
            if let Err(_) = board.pop() {
                break;
            }
        }

        // Check that the board is back to the default position
        let current_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(board.fen(), current_fen);
    }

    #[test]
    fn test_pawn_knight_promotion() {
        let mut board = Board::new();

        board.push(Move::new_from_string("d2d4").unwrap()).unwrap();
        board.push(Move::new_from_string("a7a6").unwrap()).unwrap();
        board.push(Move::new_from_string("d4d5").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c5").unwrap()).unwrap();
        board.push(Move::new_from_string("d5c6").unwrap()).unwrap();
        board.push(Move::new_from_string("d7d6").unwrap()).unwrap();
        board.push(Move::new_from_string("c2c3").unwrap()).unwrap();
        board.push(Move::new_from_string("c8e6").unwrap()).unwrap();
        board.push(Move::new_from_string("c6c7").unwrap()).unwrap();
        board.push(Move::new_from_string("d8d7").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c8N").unwrap()).unwrap();

        // Make sure the promotion worked
        let current_fen = "rnN1kbnr/1p1qpppp/p2pb3/8/8/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 6";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("d7d8").unwrap()).unwrap();
        board.push(Move::new_from_string("c8d6").unwrap()).unwrap();
        board.push(Move::new_from_string("e7d6").unwrap()).unwrap();

        // Make sure we got to the right final position
        let current_fen = "rn1qkbnr/1p3ppp/p2pb3/8/8/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 8";
        assert_eq!(board.fen(), current_fen);

        // Reverse all the moves
        loop {
            if let Err(_) = board.pop() {
                break;
            }
        }

        // Check that the board is back to the default position
        let current_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(board.fen(), current_fen);
    }

    #[test]
    // Extremely simple sanity check test on the pawn move gen.
    // Does not check edge cases.
    fn test_pawn_move_gen() {
        let board = Board::new();
        let mut moves: Vec<Move> = Vec::new();
        board.add_pawn_moves(&mut moves, Square {row: RANK_TWO, col: FILE_A});
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0], Move::new_from_string("a2a3").unwrap());
        assert_eq!(moves[1], Move::new_from_string("a2a4").unwrap());

        board.add_pawn_moves(&mut moves, Square {row: RANK_SEVEN, col: FILE_B});
        assert_eq!(moves.len(), 4);
        assert_eq!(moves[2], Move::new_from_string("b7b6").unwrap());
        assert_eq!(moves[3], Move::new_from_string("b7b5").unwrap());
    }
}
