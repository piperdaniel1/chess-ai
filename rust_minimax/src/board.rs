use std::fmt::Error;

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
        self.cells[square.row as usize][square.col as usize]
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
                    en_passant_capture = true;
                } else if self.get_square(&prev_move.inner_move.from) == BLACK_PAWN {
                    self.set_square(&Square {
                        row: RANK_FOUR,
                        col: prev_move.inner_move.to.col,
                    }, WHITE_PAWN);
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
