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

    // Piece positions,
    // Used for fast lookup of pieces
    // Index using the piece type defined in the constant
    piece_positions: [Vec<Square>; 13],
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
        const EMPTY_VEC: Vec<Square> = Vec::new();

        Board {
            cells: [[EMPTY_SQUARE; 8]; 8],
            history: Vec::new(),
            turn: true,
            en_passant: None,
            castling: [true, true, true, true],
            halfmove_clock: 0,
            fullmove_number: 1,
            piece_positions: [EMPTY_VEC; 13],
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
            //
            // By routing through set square, we ensure that the piece
            // positions are updated
            board.set_square(&Square::new(row, col), get_piece_from_char(c)?);

            /*board.cells[row as usize][col as usize] = match get_piece_from_char(c) {
                Ok(piece) => piece,
                _ => return Err(Error),
            };*/

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

        const EMPTY_VEC: Vec<Square> = Vec::new();
        self.piece_positions = [EMPTY_VEC; 13];
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
            self.set_square(&Square::new(row, col), get_piece_from_char(c)?);
            /*self.cells[row as usize][col as usize] = match get_piece_from_char(c) {
                Ok(piece) => piece,
                _ => return Err(Error),
            };*/

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
        // Update the piece list
        if piece != EMPTY_SQUARE {
            self.piece_positions[piece as usize].push(*square);
        } else {
            // TODO Maybe there is a faster way to do this
            // Potentially could use a hash map?

            // figure out the index
            let current_piece = self.get_square(square);
            let ind = self.piece_positions[current_piece as usize].iter().position(|&x| &x == square);
            let ind =  match ind {
                Some(size) => size,
                None => panic!("Fragmented piece position cache!"),
            };

            // remove the piece at square
            self.piece_positions[current_piece as usize].swap_remove(ind);
        }

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

    fn get_color(&self, piece: u8) -> Option<bool> {
        if self.is_white(piece) {
            return Some(WHITE_TURN);
        } else if self.is_black(piece) {
            return Some(BLACK_TURN);
        }

        return None;
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
     
    // Returns true if
    //   1. The square contains a piece
    //   2. The piece is the opposite color as attacking_color
    //   OR
    //   3. The square is the en passant square (we do not need to check
    //   if the en passant piece is the opposite color because it has to be
    //   due to the rules of chess and en passant)
    fn is_suitable_pawn_capture(&self, to_row: i8, to_col: i8, attacking_color: bool) -> bool {
        let target_piece = self.get_square_ind(to_row, to_col);

        // Rules 1 and 2
        if target_piece != EMPTY_SQUARE && target_piece != OUT_OF_BOUNDS {
            let color = self.get_color(self.get_square_ind(to_row, to_col)).unwrap();
            if color != attacking_color {
                return true;
            }
        // Rule 3
        } else if self.en_passant.is_some() {
            let en_passant = self.en_passant.unwrap();

            if en_passant.row as i16 == to_row as i16 
               && en_passant.col as i16 == to_col as i16 {
                return true;
            }
        }

        // If we get here, the square is not suitable
        return false;
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
            let to_row = square.row as i8 - direction as i8;
            let to_col = square.col as i8 - 1;
            if self.is_suitable_pawn_capture(to_row, to_col, WHITE_TURN) {
                self.bundle_promotions(moves, Move {
                    from: square,
                    to: Square {
                        row: square.row - direction,
                        col: square.col - 1,
                    },
                    promotion: None,
                });
            }

            // Check if the pawn can capture diagonally right
            let to_row = square.row as i8 - direction as i8;
            let to_col = square.col as i8 + 1;
            if self.is_suitable_pawn_capture(to_row, to_col, WHITE_TURN) {
                self.bundle_promotions(moves, Move {
                    from: square,
                    to: Square {
                        row: square.row - direction,
                        col: square.col + 1,
                    },
                    promotion: None,
                });
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
            let to_row = square.row as i8 + direction as i8;
            let to_col = square.col as i8 - 1;
            if self.is_suitable_pawn_capture(to_row, to_col, BLACK_TURN) {
                self.bundle_promotions(moves, Move {
                    from: square,
                    to: Square {
                        row: square.row + direction,
                        col: square.col - 1,
                    },
                    promotion: None,
                });
            }

            // Check if the pawn can capture diagonally right
            let to_row = square.row as i8 + direction as i8;
            let to_col = square.col as i8 + 1;
            if self.is_suitable_pawn_capture(to_row, to_col, BLACK_TURN) {
                self.bundle_promotions(moves, Move {
                    from: square,
                    to: Square {
                        row: square.row + direction,
                        col: square.col + 1,
                    },
                    promotion: None,
                });
            }
        } else {
            panic!("add_pawn_moves called on a non-pawn piece");
        }
    }

    fn add_knight_moves(&self, moves: &mut Vec<Move>, square: Square) {
        let target_color = self.get_color(self.get_square(&square)).unwrap();
        let x_offsets = [1, 2, 2, 1, -1, -2, -2, -1];
        let y_offsets = [2, 1, -1, -2, -2, -1, 1, 2];

        for i in 0..x_offsets.len() {
            let to_row = square.row as i8 + y_offsets[i];
            let to_col = square.col as i8 + x_offsets[i];

            let target_piece = self.get_square_ind(to_row, to_col);
            let to_square = Square {
                row: to_row as u8,
                col: to_col as u8,
            };

            if target_piece == EMPTY_SQUARE {
                moves.push(Move {
                    from: square,
                    to: to_square,
                    promotion: None,
                });
            } else {
                let square_color = self.get_color(target_piece);

                match square_color {
                    Some(color) => {
                        if color != target_color {
                            moves.push(Move {
                                from: square,
                                to: to_square,
                                promotion: None,
                            });
                        }
                    }
                    None => {}
                }
            }
        }
    }

    // Range specifies how far we should search diagonally (useful for king moves)
    fn add_diagonal_moves(&self, moves: &mut Vec<Move>, square: Square, range: u8) {
        let x_dirs = [1, 1, -1, -1];
        let y_dirs = [1, -1, 1, -1];

        let target_color = self.get_color(self.get_square(&square)).unwrap();

        for i in 0..x_dirs.len() {
            let mut to_row = square.row as i8;
            let mut to_col = square.col as i8;

            for _ in 0..range {
                to_row += y_dirs[i];
                to_col += x_dirs[i];

                let target_piece = self.get_square_ind(to_row, to_col);
                let to_square = Square {
                    row: to_row as u8,
                    col: to_col as u8,
                };

                if target_piece == EMPTY_SQUARE {
                    moves.push(Move {
                        from: square,
                        to: to_square,
                        promotion: None,
                    });
                } else {
                    let square_color = self.get_color(target_piece);

                    match square_color {
                        Some(color) => {
                            if color != target_color {
                                moves.push(Move {
                                    from: square,
                                    to: to_square,
                                    promotion: None,
                                });
                            }
                        }
                        // We break because we have gone off the board
                        None => break,
                    }

                    break;
                }
            }
        }
    }

    // completly untested and written by copilot
    // Edit: I tested it and it works perfectly (scary how well copilot works (copilot just wrote
    // that))
    fn add_straight_moves(&self, moves: &mut Vec<Move>, square: Square, range: u8) {
        let x_dirs = [1, 0, -1, 0];
        let y_dirs = [0, 1, 0, -1];

        let target_color = self.get_color(self.get_square(&square)).unwrap();

        for i in 0..x_dirs.len() {
            let mut to_row = square.row as i8;
            let mut to_col = square.col as i8;

            for _ in 0..range {
                to_row += y_dirs[i];
                to_col += x_dirs[i];

                let target_piece = self.get_square_ind(to_row, to_col);
                let to_square = Square {
                    row: to_row as u8,
                    col: to_col as u8,
                };

                if target_piece == EMPTY_SQUARE {
                    moves.push(Move {
                        from: square,
                        to: to_square,
                        promotion: None,
                    });
                } else {
                    let square_color = self.get_color(target_piece);

                    match square_color {
                        Some(color) => {
                            if color != target_color {
                                moves.push(Move {
                                    from: square,
                                    to: to_square,
                                    promotion: None,
                                });
                            }
                        }
                        // We break because we have gone off the board
                        None => break,
                    }

                    break;
                }
            }
        }
    }

    // Private function to generate all pseudo-legal moves
    // for the current position. This function takes all the rules into
    // account except for any rules involving check on the king.
    fn gen_psuedo_legal_moves(&self) -> Vec<Move> {
        // lmao in theory it works
        let mut moves: Vec<Move> = Vec::new();

        // This weird system makes sure that we only add
        // pseudo legal moves for the player that currently has
        // their turn
        let mut left_bound = 1;
        let mut right_bound = 7;
        if self.turn == BLACK_TURN {
            left_bound = 7;
            right_bound = 13;
        }

        for i in left_bound..right_bound {
            for j in 0..self.piece_positions[i].len() {
                let curr_square = self.piece_positions[i][j];

                // Kings
                if i == 1 || i == 7 {
                    self.add_diagonal_moves(&mut moves, curr_square, 1);
                    self.add_straight_moves(&mut moves, curr_square, 1);
                // Queens
                } else if i == 2 || i == 8 {
                    self.add_diagonal_moves(&mut moves, curr_square, 8);
                    self.add_straight_moves(&mut moves, curr_square, 8);
                // Rooks
                } else if i == 3 || i == 9 {
                    self.add_straight_moves(&mut moves, curr_square, 8);
                // Bishops
                } else if i == 4 || i == 10 {
                    self.add_diagonal_moves(&mut moves, curr_square, 8);
                // Knights
                } else if i == 5 || i == 11 {
                    self.add_knight_moves(&mut moves, curr_square);
                // Pawns
                } else if i == 6 || i == 12 {
                    self.add_pawn_moves(&mut moves, curr_square);
                }
            }
        }

        moves
    }

    // Returns true if
    //   1. White is in check
    //   2. It is black's turn
    // or,
    //   1. Black is in check
    //   2. It is white's turn
    fn has_illegal_check(&self) -> bool {
        // We should store all the squares that are being attacked
        // by each color. However, we can also just get the psuedo legal moves
        // and check if one of them ends on the king.
        let psuedo_legal = self.gen_psuedo_legal_moves();

        let king_square: Square;
        // White turn
        if self.turn == WHITE_TURN {
            // This should always work because there should always be one black king
            king_square = self.piece_positions[BLACK_KING as usize][0];
        // Black turn
        } else {
            king_square = self.piece_positions[WHITE_KING as usize][0];
        }

        for elem in psuedo_legal.iter() {
            // If one of the pieces can take the king this is an illegal check
            if elem.to == king_square {
                return true;
            }
        }

        return false;
    }

    // Private function that removes all pseudo-legal moves that
    // would leave the king in check

    // I hope that the lifetimes are good idk what I am doing
    fn trim_illegal_moves<'a>(&'a mut self , moves: &'a mut Vec<Move>) -> &mut Vec<Move> {
        // TODO
        // The bad way of doing this is to make each move on the board and
        // then check if it leaves the current players turn in check.
        //
        // The good way would be to store all squares that are being attacked by each
        // color and then make sure that the king does not move into these
        //
        // Also you would have to somehow make sure that pieces that are pinned cannot
        // move out of the pin

        // We'll start with the bad way because it is easier:

        let mut i = 0;
        while i < moves.len() {
            // The move should always push correctly because
            // it should have been just generated
            self.push(moves[i]).unwrap();

            if self.has_illegal_check() {
                moves.remove(i);
                i -= 1;
            }
            
            // There should always be a move to pop because we just pushed one
            self.pop().unwrap();

            i += 1;
        }

        return moves;
    }

    // Public function to get a vector of all legal moves
    pub fn legal_moves(&mut self) -> Vec<Move> {
        // TODO
        let mut moves = self.gen_psuedo_legal_moves();

        self.trim_illegal_moves(&mut moves);

        moves
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

    #[test]
    // Extremely simple sanity check test on the knight move gen.
    // Does not check edge cases.
    fn test_knight_move_gen() {
        let board = Board::new();
        let mut moves: Vec<Move> = Vec::new();
        board.add_knight_moves(&mut moves, Square {row: RANK_ONE, col: FILE_B});
        board.add_knight_moves(&mut moves, Square {row: RANK_EIGHT, col: FILE_G});

        assert_eq!(moves.len(), 4);
        assert!(moves.contains(&Move::new_from_string("b1a3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b1c3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g8f6").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g8h6").unwrap()));
    }

    #[test]
    // Extremely simple sanity check test on the diagonal piece move gen.
    // Does not check edge cases except for capturing opposite color pieces.
    fn test_diagonal_move_gen() {
        let mut board = Board::new();
        let mut moves: Vec<Move> = Vec::new();
        board.add_diagonal_moves(&mut moves, Square {row: RANK_ONE, col: FILE_C}, 8);
        assert_eq!(moves.len(), 0);

        board.push(Move::new_from_string("e2e4").unwrap()).unwrap();
        board.add_diagonal_moves(&mut moves, Square {row: RANK_ONE, col: FILE_D}, 8);
        assert_eq!(moves.len(), 4);
        assert!(moves.contains(&Move::new_from_string("d1e2").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d1f3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d1g4").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d1h5").unwrap()));

        board.push(Move::new_from_string("e7e5").unwrap()).unwrap();
        board.push(Move::new_from_string("d2d3").unwrap()).unwrap();
        board.push(Move::new_from_string("c7c5").unwrap()).unwrap();
        moves.clear();
        board.add_diagonal_moves(&mut moves, Square {row: RANK_EIGHT, col: FILE_D}, 8);
        assert_eq!(moves.len(), 7);
        assert!(moves.contains(&Move::new_from_string("d8e7").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d8f6").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d8g5").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d8h4").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d8c7").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d8b6").unwrap()));
        assert!(moves.contains(&Move::new_from_string("d8a5").unwrap()));

        board.push(Move::new_from_string("d1g4").unwrap()).unwrap();
        moves.clear();
        board.add_diagonal_moves(&mut moves, Square {row: RANK_FOUR, col: FILE_G}, 8);
        assert_eq!(moves.len(), 8);
        assert!(moves.contains(&Move::new_from_string("g4f5").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4e6").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4f3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4e2").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4d1").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4h5").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4h3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4d7").unwrap()));

        moves.clear();
        board.add_diagonal_moves(&mut moves, Square {row: RANK_FOUR, col: FILE_G}, 1);
        assert_eq!(moves.len(), 4);
        assert!(moves.contains(&Move::new_from_string("g4f5").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4h5").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4h3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("g4f3").unwrap()));
    }

    #[test]
    // Extremely simple sanity check test on the straight piece move gen.
    // Does not check edge cases except for capturing opposite color.
    fn test_straight_move_gen() {
        let mut board = Board::new();
        let mut moves: Vec<Move> = Vec::new();
        board.add_straight_moves(&mut moves, Square {row: RANK_ONE, col: FILE_C}, 8);
        assert_eq!(moves.len(), 0);

        board.push(Move::new_from_string("a2a4").unwrap()).unwrap();
        board.add_straight_moves(&mut moves, Square {row: RANK_ONE, col: FILE_A}, 8);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Move::new_from_string("a1a2").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a1a3").unwrap()));

        board.push(Move::new_from_string("a7a5").unwrap()).unwrap();
        board.push(Move::new_from_string("a1a3").unwrap()).unwrap();

        moves.clear();
        board.add_straight_moves(&mut moves, Square {row: RANK_THREE, col: FILE_A}, 8);
        assert_eq!(moves.len(), 9);
        assert!(moves.contains(&Move::new_from_string("a3a2").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3a1").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3b3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3c3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3d3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3e3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3f3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3g3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("a3h3").unwrap()));

        board.push(Move::new_from_string("b7b6").unwrap()).unwrap();
        board.push(Move::new_from_string("a3b3").unwrap()).unwrap();

        moves.clear();
        board.add_straight_moves(&mut moves, Square {row: RANK_THREE, col: FILE_B}, 8);
        assert_eq!(moves.len(), 10);
        assert!(moves.contains(&Move::new_from_string("b3a3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3c3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3d3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3e3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3f3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3g3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3h3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3b4").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3b5").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3b6").unwrap()));

        moves.clear();
        board.add_straight_moves(&mut moves, Square {row: RANK_THREE, col: FILE_B}, 1);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&Move::new_from_string("b3b4").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3a3").unwrap()));
        assert!(moves.contains(&Move::new_from_string("b3c3").unwrap()));
    }

    #[test]
    // Just using this as a main function sort of thing
    fn dummy_test() {
        const EMPTY_VEC: Vec<Square> = Vec::new();
        let mut piece_cache = [EMPTY_VEC; 12];

        piece_cache[0].push(Square {row: RANK_ONE, col: FILE_A});

        assert_eq!(piece_cache[0].len(), 1);
        assert_eq!(piece_cache[1].len(), 0);
    }

    #[test]
    // Tests the psuedo legal move generation
    // We do not test this much at all, that is saved for the legal move tests
    fn test_psuedo_legal() {
        let board = Board::new();

        let moves = board.gen_psuedo_legal_moves();
        assert_eq!(moves.len(), 20);
    }

    fn perft(starting_board: &mut Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut total = 0;

        for elem in starting_board.legal_moves() {
            starting_board.push(elem).unwrap();

            total += perft(starting_board, depth-1);

            starting_board.pop().unwrap();
        }

        return total;
    }

    #[test]
    fn test_legal() {
        let mut board = Board::new();

        assert_eq!(perft(&mut board, 1), 20);
        assert_eq!(perft(&mut board, 2), 400);
         
        board.print();
        assert_eq!(perft(&mut board, 3), 8902);
    }
}
