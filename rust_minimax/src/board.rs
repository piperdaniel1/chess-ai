use std::fmt::Error;
use rand_chacha::rand_core::SeedableRng;
use rand_core::RngCore;
use rand_chacha;
// TODO
// - Update the attacking maps every time psuedo legal moves
//   are made. These maps are already in the board struct.

#[allow(dead_code)]
const SAFE_MODE : bool = true;

pub const BLACK: bool = false;
pub const WHITE: bool = true;

pub const RANK_ONE: u8 = 7;
pub const RANK_TWO: u8 = 6;
pub const RANK_THREE: u8 = 5;
pub const RANK_FOUR: u8 = 4;
pub const RANK_FIVE: u8 = 3;
pub const RANK_SIX: u8 = 2;
pub const RANK_SEVEN: u8 = 1;
pub const RANK_EIGHT: u8 = 0;

#[allow(dead_code)]
pub const FILE_A: u8 = 0;
#[allow(dead_code)]
pub const FILE_B: u8 = 1;
#[allow(dead_code)]
pub const FILE_C: u8 = 2;
#[allow(dead_code)]
pub const FILE_D: u8 = 3;
#[allow(dead_code)]
pub const FILE_E: u8 = 4;
#[allow(dead_code)]
pub const FILE_F: u8 = 5;
#[allow(dead_code)]
pub const FILE_G: u8 = 6;
#[allow(dead_code)]
pub const FILE_H: u8 = 7;

pub const WHITE_KINGSIDE_CASTLE: usize = 0;
pub const WHITE_QUEENSIDE_CASTLE: usize = 1;
pub const BLACK_KINGSIDE_CASTLE: usize = 2;
pub const BLACK_QUEENSIDE_CASTLE: usize = 3;
pub const BLACK_PAWN: u8 = 12;
pub const BLACK_KNIGHT: u8 = 11;
pub const BLACK_BISHOP: u8 = 10;
pub const BLACK_ROOK: u8 = 9;
pub const BLACK_QUEEN: u8 = 8;
pub const BLACK_KING: u8 = 7;
pub const WHITE_PAWN: u8 = 6;
pub const WHITE_KNIGHT: u8 = 5;
pub const WHITE_BISHOP: u8 = 4;
pub const WHITE_ROOK: u8 = 3;
pub const WHITE_QUEEN: u8 = 2;
pub const WHITE_KING: u8 = 1;
pub const EMPTY_SQUARE: u8 = 0;
pub const OUT_OF_BOUNDS: u8 = 255;

#[allow(dead_code)]
pub const A8: Square = Square { row: 0, col: 0 };
#[allow(dead_code)]
pub const B8: Square = Square { row: 0, col: 1 };
#[allow(dead_code)]
pub const C8: Square = Square { row: 0, col: 2 };
#[allow(dead_code)]
pub const D8: Square = Square { row: 0, col: 3 };
#[allow(dead_code)]
pub const E8: Square = Square { row: 0, col: 4 };
#[allow(dead_code)]
pub const F8: Square = Square { row: 0, col: 5 };
#[allow(dead_code)]
pub const G8: Square = Square { row: 0, col: 6 };
#[allow(dead_code)]
pub const H8: Square = Square { row: 0, col: 7 };
#[allow(dead_code)]
pub const A7: Square = Square { row: 1, col: 0 };
#[allow(dead_code)]
pub const B7: Square = Square { row: 1, col: 1 };
#[allow(dead_code)]
pub const C7: Square = Square { row: 1, col: 2 };
#[allow(dead_code)]
pub const D7: Square = Square { row: 1, col: 3 };
#[allow(dead_code)]
pub const E7: Square = Square { row: 1, col: 4 };
#[allow(dead_code)]
pub const F7: Square = Square { row: 1, col: 5 };
#[allow(dead_code)]
pub const G7: Square = Square { row: 1, col: 6 };
#[allow(dead_code)]
pub const H7: Square = Square { row: 1, col: 7 };
#[allow(dead_code)]
pub const A6: Square = Square { row: 2, col: 0 };
#[allow(dead_code)]
pub const B6: Square = Square { row: 2, col: 1 };
#[allow(dead_code)]
pub const C6: Square = Square { row: 2, col: 2 };
#[allow(dead_code)]
pub const D6: Square = Square { row: 2, col: 3 };
#[allow(dead_code)]
pub const E6: Square = Square { row: 2, col: 4 };
#[allow(dead_code)]
pub const F6: Square = Square { row: 2, col: 5 };
#[allow(dead_code)]
pub const G6: Square = Square { row: 2, col: 6 };
#[allow(dead_code)]
pub const H6: Square = Square { row: 2, col: 7 };
#[allow(dead_code)]
pub const A5: Square = Square { row: 3, col: 0 };
#[allow(dead_code)]
pub const B5: Square = Square { row: 3, col: 1 };
#[allow(dead_code)]
pub const C5: Square = Square { row: 3, col: 2 };
#[allow(dead_code)]
pub const D5: Square = Square { row: 3, col: 3 };
#[allow(dead_code)]
pub const E5: Square = Square { row: 3, col: 4 };
#[allow(dead_code)]
pub const F5: Square = Square { row: 3, col: 5 };
#[allow(dead_code)]
pub const G5: Square = Square { row: 3, col: 6 };
#[allow(dead_code)]
pub const H5: Square = Square { row: 3, col: 7 };
#[allow(dead_code)]
pub const A4: Square = Square { row: 4, col: 0 };
#[allow(dead_code)]
pub const B4: Square = Square { row: 4, col: 1 };
#[allow(dead_code)]
pub const C4: Square = Square { row: 4, col: 2 };
#[allow(dead_code)]
pub const D4: Square = Square { row: 4, col: 3 };
#[allow(dead_code)]
pub const E4: Square = Square { row: 4, col: 4 };
#[allow(dead_code)]
pub const F4: Square = Square { row: 4, col: 5 };
#[allow(dead_code)]
pub const G4: Square = Square { row: 4, col: 6 };
#[allow(dead_code)]
pub const H4: Square = Square { row: 4, col: 7 };
#[allow(dead_code)]
pub const A3: Square = Square { row: 5, col: 0 };
#[allow(dead_code)]
pub const B3: Square = Square { row: 5, col: 1 };
#[allow(dead_code)]
pub const C3: Square = Square { row: 5, col: 2 };
#[allow(dead_code)]
pub const D3: Square = Square { row: 5, col: 3 };
#[allow(dead_code)]
pub const E3: Square = Square { row: 5, col: 4 };
#[allow(dead_code)]
pub const F3: Square = Square { row: 5, col: 5 };
#[allow(dead_code)]
pub const G3: Square = Square { row: 5, col: 6 };
#[allow(dead_code)]
pub const H3: Square = Square { row: 5, col: 7 };
#[allow(dead_code)]
pub const A2: Square = Square { row: 6, col: 0 };
#[allow(dead_code)]
pub const B2: Square = Square { row: 6, col: 1 };
#[allow(dead_code)]
pub const C2: Square = Square { row: 6, col: 2 };
#[allow(dead_code)]
pub const D2: Square = Square { row: 6, col: 3 };
#[allow(dead_code)]
pub const E2: Square = Square { row: 6, col: 4 };
#[allow(dead_code)]
pub const F2: Square = Square { row: 6, col: 5 };
#[allow(dead_code)]
pub const G2: Square = Square { row: 6, col: 6 };
#[allow(dead_code)]
pub const H2: Square = Square { row: 6, col: 7 };
#[allow(dead_code)]
pub const A1: Square = Square { row: 7, col: 0 };
#[allow(dead_code)]
pub const B1: Square = Square { row: 7, col: 1 };
#[allow(dead_code)]
pub const C1: Square = Square { row: 7, col: 2 };
#[allow(dead_code)]
pub const D1: Square = Square { row: 7, col: 3 };
#[allow(dead_code)]
pub const E1: Square = Square { row: 7, col: 4 };
#[allow(dead_code)]
pub const F1: Square = Square { row: 7, col: 5 };
#[allow(dead_code)]
pub const G1: Square = Square { row: 7, col: 6 };
#[allow(dead_code)]
pub const H1: Square = Square { row: 7, col: 7 };
#[allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
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

    // Zobrist keys for each piece on each square
    // The theory is that it is okay to store all these keys in here
    // because the board is not supposed to be lightweight -- it is supposed to be fast.
    square_keys: [[u64; 64]; 12],

    // Zobrist keys for other board properties
    turn_key: u64,
    en_passant_file_keys: [u64; 8],
    castling_keys: [u64; 4],

    // The current zobrist hash of the board
    hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square { pub row: u8, pub col: u8 }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move { pub from: Square, pub to: Square, pub promotion: Option<u8> }

// These PrevMove objects are used to keep track of the previous move
// with enough detail to undo it
#[derive(Debug, Clone, Copy, PartialEq)]
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

    // Stores the zobrist hash that was present
    hash: u64,
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

// We always return a lowercase value because
// we don't care about the color of the piece if we are
// parsing a move string (which is what this function is used for)
fn get_char_from_piece(piece: u8) -> Result<char, Error> {
    match piece {
        BLACK_PAWN => Ok('p'),
        BLACK_KNIGHT => Ok('n'),
        BLACK_BISHOP => Ok('b'),
        BLACK_ROOK => Ok('r'),
        BLACK_QUEEN => Ok('q'),
        BLACK_KING => Ok('k'),
        WHITE_PAWN => Ok('p'),
        WHITE_KNIGHT => Ok('n'),
        WHITE_BISHOP => Ok('b'),
        WHITE_ROOK => Ok('r'),
        WHITE_QUEEN => Ok('q'),
        WHITE_KING => Ok('k'),
        _ => Err(Error),
    }
}

fn get_zobrist_keys() -> [u64; 793] {
    let seed: u64 = 8675309_2357_31415926;
    let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let mut keys = [0; 793];

    for i in 0..793 {
        keys[i] = gen.next_u64();
    }

    keys
}

impl Move {
    pub fn new(from: Square, to: Square, promotion: Option<u8>) -> Move {
        Move { from, to, promotion }
    }

    pub fn new_from_string(s: &str) -> Result<Move, Error> {
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

    pub fn get_move_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.from.get_square_string());
        s.push_str(&self.to.get_square_string());

        if self.promotion.is_some() {
            s.push_str(&get_char_from_piece(self.promotion.unwrap()).unwrap().to_string());
        }

        s
    }
}

impl Square {
    fn new(row: u8, col: u8) -> Square {
        Square { row, col }
    }

    pub fn get_square_string(&self) -> String {
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

    fn get_index(&self) -> usize {
        (self.row * 8 + self.col) as usize
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
            castling: [false, false, false, false],
            halfmove_clock: 0,
            fullmove_number: 1,
            piece_positions: [vec![], vec![], vec![], vec![], vec![], vec![], vec![],
                              vec![], vec![], vec![], vec![], vec![], vec![]],
            square_keys: [[0; 64]; 12],
            turn_key: 0,
            en_passant_file_keys: [0; 8],
            castling_keys: [0; 4],
            hash: 0,
        }
    }
        
    pub fn turn(&self) -> bool {
        self.turn
    }

    /* These functions are useful for scoring the board */
    pub fn get_pawn_differential(&self) -> i32 {
        return self.piece_positions[WHITE_PAWN as usize].len() as i32 -
            self.piece_positions[BLACK_PAWN as usize].len() as i32;
    }

    pub fn get_knight_differential(&self) -> i32 {
        return self.piece_positions[WHITE_KNIGHT as usize].len() as i32 -
            self.piece_positions[BLACK_KNIGHT as usize].len() as i32;
    }

    pub fn get_bishop_differential(&self) -> i32 {
        return self.piece_positions[WHITE_BISHOP as usize].len() as i32 -
            self.piece_positions[BLACK_BISHOP as usize].len() as i32;
    }

    pub fn get_rook_differential(&self) -> i32 {
        return self.piece_positions[WHITE_ROOK as usize].len() as i32 -
            self.piece_positions[BLACK_ROOK as usize].len() as i32;
    }

    pub fn get_queen_differential(&self) -> i32 {
        return self.piece_positions[WHITE_QUEEN as usize].len() as i32 -
            self.piece_positions[BLACK_QUEEN as usize].len() as i32;
    }

    pub fn get_pawn_list(&self, color: bool) -> &Vec<Square> {
        &self.piece_positions[if color { WHITE_PAWN } else { BLACK_PAWN } as usize]
    }

    pub fn get_knight_list(&self, color: bool) -> &Vec<Square> {
        &self.piece_positions[if color { WHITE_KNIGHT } else { BLACK_KNIGHT } as usize]
    }

    pub fn get_bishop_list(&self, color: bool) -> &Vec<Square> {
        &self.piece_positions[if color { WHITE_BISHOP } else { BLACK_BISHOP } as usize]
    }

    pub fn get_rook_list(&self, color: bool) -> &Vec<Square> {
        &self.piece_positions[if color { WHITE_ROOK } else { BLACK_ROOK } as usize]
    }

    pub fn get_queen_list(&self, color: bool) -> &Vec<Square> {
        &self.piece_positions[if color { WHITE_QUEEN } else { BLACK_QUEEN } as usize]
    }

    pub fn get_king_square(&self, color: bool) -> Square {
        self.piece_positions[if color { WHITE_KING } else { BLACK_KING } as usize][0]
    }

    pub fn checkmate(&self) -> bool {
        self.has_check().is_some() && self.gen_legal_moves().len() == 0
    }

    pub fn stalemate(&self) -> bool {
        self.has_check().is_none() && self.gen_legal_moves().len() == 0
    }

    pub fn threefold_repetition(&self) -> bool {
        let mut count = 0;

        for i in 0..self.history.len() {
            if self.history[i].hash == self.hash {
                count += 1;
            }
        }

        return count >= 3;
    }

    pub fn get_game_phase(&self) -> i32 {
        // The opening lasts for the first 10 moves each.
        // This should probably give the engine time to develop its pieces.
        if self.history.len() < 20 {
            return 0;
        }

        // The endgame starts when there are four or fewer major pieces on the board (besides kings).
        let mut major_pieces = 0;
        major_pieces += self.piece_positions[WHITE_QUEEN as usize].len();
        major_pieces += self.piece_positions[BLACK_QUEEN as usize].len();
        major_pieces += self.piece_positions[WHITE_ROOK as usize].len();
        major_pieces += self.piece_positions[BLACK_ROOK as usize].len();
        major_pieces += self.piece_positions[WHITE_BISHOP as usize].len();
        major_pieces += self.piece_positions[BLACK_BISHOP as usize].len();
        major_pieces += self.piece_positions[WHITE_KNIGHT as usize].len();
        major_pieces += self.piece_positions[BLACK_KNIGHT as usize].len();
        if major_pieces <= 4 {
            return 2;
        }

        // The middle game is when both the opening and endgame are not happening.
        return 1;
    }

    #[allow(dead_code)]
    fn is_other_cache_equivalent(&self, other: &Board) -> bool {
        for i in 0..13 {
            for j in 0..self.piece_positions[i].len() {
                if !self.piece_positions[i].contains(&other.piece_positions[i][j]) {
                    return false;
                }
            }
        }

        true
    }

    #[allow(dead_code)]
    fn is_cache_desynced(&self) -> bool {
        // Make sure everything in the cache is on the board
        for i in 0..13 {
            for j in 0..self.piece_positions[i].len() {
                if self.get_square(&self.piece_positions[i][j]) != i as u8 {
                    return true;
                }
            }
        }

        // Make sure everything on the board is in the cache
        for i in 0..8 {
            for j in 0..8 {
                let piece = self.get_square(&Square::new(i, j));
                if piece != EMPTY_SQUARE && !self.piece_positions[piece as usize].contains(&Square::new(i, j)) {
                    return true;
                }
            }
        }

        false
    }

    #[allow(dead_code)]
    fn recache(&mut self) {
        for i in 0..13 {
            self.piece_positions[i].clear();
        }

        for i in 0..8 {
            for j in 0..8 {
                let piece = self.get_square(&Square::new(i, j));
                if piece != EMPTY_SQUARE {
                    self.piece_positions[piece as usize].push(Square::new(i, j));
                }
            }
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
                '-' => break,
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

        // Fill in the zobrist keys
        let mut i = 0;
        let keys = get_zobrist_keys();

        for piece in 0..12 {
            for square in 0..64 {
                board.square_keys[piece][square] = keys[i];
                i += 1;
            }
        }

        board.turn_key = keys[i];
        i += 1;

        for j in 0..4 {
            board.castling_keys[j] = keys[i];
            i += 1;
        }

        for j in 0..8 {
            board.en_passant_file_keys[j] = keys[i];
            i += 1;
        }

        // Calculate the hash using the zobrist keys
        board.hash = board.calculate_zobrist_hash();

        Ok(board)
    }

    fn calculate_zobrist_hash(&self) -> u64 {
        let mut hash = 0;

        for i in 0..8 {
            for j in 0..8 {
                let piece = self.get_square_ind(i as i8, j as i8);

                if piece != EMPTY_SQUARE {
                    hash ^= self.square_keys[(piece-1) as usize][(i * 8 + j) as usize];
                }
            }
        }

        if !self.turn {
            hash ^= self.turn_key;
        }

        for i in 0..4 {
            if self.get_castling_bool(i) {
                hash ^= self.castling_keys[i];
            }
        }

        if let Some(square) = self.en_passant {
            hash ^= self.en_passant_file_keys[square.col as usize];
        }

        hash
    }

    #[allow(dead_code)]
    pub fn get_hash(&self) -> u64 {
        self.hash
    }
    
    // Setup the board to the starting position
    pub fn clear(&mut self) {
        self.cells = [[EMPTY_SQUARE; 8]; 8];
        self.history = Vec::new();
        self.turn = true;
        self.en_passant = None;
        self.castling = [false, false, false, false];
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
                'K' => self.set_castling_rights(WHITE_KINGSIDE_CASTLE, true),
                'Q' => self.set_castling_rights(WHITE_QUEENSIDE_CASTLE, true),
                'k' => self.set_castling_rights(BLACK_KINGSIDE_CASTLE, true),
                'q' => self.set_castling_rights(BLACK_QUEENSIDE_CASTLE, true),
                '-' => break,
                _ => return Err(Error),
            }
        }

        // Set en passant target square
        if fen_parts[3] != "-" {
            self.set_en_passant_square(Some(Square::new_from_string(fen_parts[3])));
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

        // Calculate the hash using the zobrist keys
        self.hash = self.calculate_zobrist_hash();

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

        if self.get_castling_bool(WHITE_KINGSIDE_CASTLE) {
            castling_rights.push('K');
        }

        if self.get_castling_bool(WHITE_QUEENSIDE_CASTLE) {
            castling_rights.push('Q');
        }

        if self.get_castling_bool(BLACK_KINGSIDE_CASTLE) {
            castling_rights.push('k');
        }

        if self.get_castling_bool(BLACK_QUEENSIDE_CASTLE) {
            castling_rights.push('q');
        }

        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        castling_rights
    }

    #[allow(dead_code)]
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
        if self.get_castling_bool(WHITE_KINGSIDE_CASTLE) {
            fen.push('K');
        }
        if self.get_castling_bool(WHITE_QUEENSIDE_CASTLE) {
            fen.push('Q');
        }
        if self.get_castling_bool(BLACK_KINGSIDE_CASTLE) {
            fen.push('k');
        }
        if self.get_castling_bool(BLACK_QUEENSIDE_CASTLE) {
            fen.push('q');
        }
        if !self.get_castling_bool(WHITE_KINGSIDE_CASTLE)
           && !self.get_castling_bool(WHITE_QUEENSIDE_CASTLE)
           && !self.get_castling_bool(BLACK_KINGSIDE_CASTLE)
           && !self.get_castling_bool(BLACK_QUEENSIDE_CASTLE) {
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

    fn set_castling_rights(&mut self, ind: usize, value: bool) {
        if self.castling[ind] != value {
            // Update the hash
            self.hash ^= self.castling_keys[ind];
        }

        // Set the castling rights
        self.castling[ind] = value;
    }

    fn get_castling_bool(&self, ind: usize) -> bool {
        self.castling[ind]
    }

    fn set_en_passant_square(&mut self, square: Option<Square>) {
        // If the old en passant square is not None, remove it from the hash
        if let Some(old_square) = &self.en_passant {
            self.hash ^= self.en_passant_file_keys[old_square.col as usize];
        }

        // If the en passant square is not None, update the hash
        if let Some(square) = square {
            // Update the hash
            self.hash ^= self.en_passant_file_keys[square.col as usize];
        }

        // Set the en passant square
        self.en_passant = square;
    }

    fn flip_turn(&mut self) {
        self.turn = !self.turn;
        self.hash ^= self.turn_key;
    }

    #[allow(dead_code)]
    fn set_square_ind(&mut self, row: u8, col: u8, piece: u8) {
        self.set_square(&Square {row, col}, piece);
    }

    /* These functions are used to cheaply mutate the zobrist hash of the board */
    fn xor_hash_piece(&mut self, piece: u8, square: &Square) {
        self.hash ^= self.square_keys[(piece-1) as usize][square.get_index()];
    }

    fn set_square(&mut self, square: &Square, piece: u8) {
        // TODO Maybe there is a faster way to do this
        // Potentially could use a hash map?

        // Make sure to remove the piece that used to be in that square
        // from the piece position cache (if there was one)
        
        // Check what is in the square right now
        let current_piece = self.get_square(square);

        // Check if there actually was a piece there
        if current_piece != EMPTY_SQUARE {
            let ind = self.piece_positions[current_piece as usize].iter().position(|&x| &x == square);
            let ind =  match ind {
                Some(size) => size,
                None => panic!("Fragmented piece position cache!"),
            };

            // remove the piece at square
            self.piece_positions[current_piece as usize].swap_remove(ind);

            // xor the piece out of the hash
            self.xor_hash_piece(current_piece, square);
        }

        // Update the piece list
        if piece != EMPTY_SQUARE {
            self.piece_positions[piece as usize].push(*square);

            // xor the piece into the hash
            self.xor_hash_piece(piece, square);
        }

        // Update the board
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
            hash: self.hash,
        };

        // Set the square we are leaving to empty
        self.set_square(&mv.from, EMPTY_SQUARE);

        // Set the square we are moving to to the piece we are moving
        self.set_square(&mv.to, from_piece);

        //
        // HERE COME THE EDGE CASES
        //

        // If we captured a rook, we cannot castle with that side anymore
        if to_piece.is_some() {
            if to_piece.unwrap() == WHITE_ROOK  {
                if mv.to == H1 {
                    self.set_castling_rights(WHITE_KINGSIDE_CASTLE, false);
                } else if mv.to == A1 {
                    self.set_castling_rights(WHITE_QUEENSIDE_CASTLE, false);
                }
            } else if to_piece.unwrap() == BLACK_ROOK {
                if mv.to == H8 {
                    self.set_castling_rights(BLACK_KINGSIDE_CASTLE, false);
                } else if mv.to == A8 {
                    self.set_castling_rights(BLACK_QUEENSIDE_CASTLE, false);
                }
            }
        }

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
            self.set_castling_rights(WHITE_KINGSIDE_CASTLE, false);
            self.set_castling_rights(WHITE_QUEENSIDE_CASTLE, false);
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
            self.set_castling_rights(BLACK_KINGSIDE_CASTLE, false);
            self.set_castling_rights(BLACK_QUEENSIDE_CASTLE, false);
        }

        // If this was a rook move, castling is no longer possible on that side
        if from_piece == WHITE_ROOK {
            if mv.from == A1 {
                self.set_castling_rights(WHITE_QUEENSIDE_CASTLE, false);
            } else if mv.from == H1 {
                self.set_castling_rights(WHITE_KINGSIDE_CASTLE, false);
            }
        } else if from_piece == BLACK_ROOK {
            if mv.from == A8 {
                self.set_castling_rights(BLACK_QUEENSIDE_CASTLE, false);
            } else if mv.from == H8 {
                self.set_castling_rights(BLACK_KINGSIDE_CASTLE, false);
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

            self.set_en_passant_square(Some(Square {
                row: RANK_THREE,
                col: mv.to.col,
            }));
        } else if from_piece == BLACK_PAWN
            && mv.from.row == RANK_SEVEN && mv.to.row == RANK_FIVE {

            self.set_en_passant_square(Some(Square {
                row: RANK_SIX,
                col: mv.to.col,
            }));
        // Otherwise, clear the en passant target square
        } else {
            self.set_en_passant_square(None);
        }

        // If this was a promotion, promote the pawn
        // Also, set the promotion field of the PrevMove struct
        if let Some(promotion) = mv.promotion {
            if from_piece == BLACK_PAWN {
                self.set_square(&mv.to, promotion);
            } else {
                if promotion >= BLACK_KING {
                    self.set_square(&mv.to, promotion - 6);
                } else {
                    self.set_square(&mv.to, promotion);
                }
            }
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
        if self.turn == BLACK {
            self.fullmove_number += 1;
        }

        // Switch the turn
        self.flip_turn();

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
        self.set_en_passant_square(prev_move.prev_en_passant);

        // Restore the half move clock
        self.halfmove_clock = prev_move.prev_halfmove_clock;

        // Restore the full move number as long as we are reversing a black move
        if self.turn {
            self.fullmove_number -= 1;
        }

        // Switch the turn
        self.flip_turn();

        // Restore the castling rights
        for (i, &castling) in prev_move.prev_castling.iter().enumerate() {
            self.set_castling_rights(i, castling);
        }

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
            return Some(WHITE);
        } else if self.is_black(piece) {
            return Some(BLACK);
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
            if self.is_suitable_pawn_capture(to_row, to_col, WHITE) {
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
            if self.is_suitable_pawn_capture(to_row, to_col, WHITE) {
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
            if self.is_suitable_pawn_capture(to_row, to_col, BLACK) {
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
            if self.is_suitable_pawn_capture(to_row, to_col, BLACK) {
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
            println!("Piece: {}", target_piece);
            println!("Square: {:?}", square);
            self.print();
            println!("Piece Cache: {:#?}", self.piece_positions);
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

    fn add_castling_moves(&self, moves: &mut Vec<Move>) {
        // White castling
        if self.turn {
            // Kingside
            if self.get_castling_bool(WHITE_KINGSIDE_CASTLE) {
                if self.square_has_check(F1).is_none() && self.get_square(&F1) == EMPTY_SQUARE {
                    if self.square_has_check(G1).is_none() && self.get_square(&G1) == EMPTY_SQUARE {
                        moves.push(Move {
                            from: E1,
                            to: G1,
                            promotion: None,
                        });
                    }
                }

            }
            // Queenside
            if self.get_castling_bool(WHITE_QUEENSIDE_CASTLE) {
                if self.square_has_check(D1).is_none() && self.get_square(&D1) == EMPTY_SQUARE {
                    if self.square_has_check(C1).is_none() && self.get_square(&C1) == EMPTY_SQUARE {
                        if self.get_square(&B1) == EMPTY_SQUARE {
                            moves.push(Move {
                                from: E1,
                                to: C1,
                                promotion: None,
                            });
                        }
                    }
                }
            }
        // Black castling
        } else {
            // Kingside
            if self.get_castling_bool(BLACK_KINGSIDE_CASTLE) {
                if self.square_has_check(F8).is_none() && self.get_square(&F8) == EMPTY_SQUARE {
                    if self.square_has_check(G8).is_none() && self.get_square(&G8) == EMPTY_SQUARE {
                        moves.push(Move {
                            from: E8,
                            to: G8,
                            promotion: None,
                        });
                    }
                }

            }
            // Queenside
            if self.get_castling_bool(BLACK_QUEENSIDE_CASTLE) {
                if self.square_has_check(D8).is_none() && self.get_square(&D8) == EMPTY_SQUARE {
                    if self.square_has_check(C8).is_none() && self.get_square(&C8) == EMPTY_SQUARE {
                        if self.get_square(&B8) == EMPTY_SQUARE {
                            moves.push(Move {
                                from: E8,
                                to: C8,
                                promotion: None,
                            });
                        }
                    }
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
        if self.turn == BLACK {
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

                    if self.has_check().is_none() {
                        self.add_castling_moves(&mut moves);
                    }
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

    #[allow(dead_code)]
    pub fn get_piece_at(&self, square: &Square) -> Option<u8> {
        let piece = self.get_square(square);

        return match piece {
            EMPTY_SQUARE => None,
            _ => Some(piece),
        };
    }

    // There is a bug that allows us a pseudo legal move to slip through if
    // there is a double check on the king.
    pub fn gen_legal_moves(& self) -> Vec<Move> {
        let moves = self.gen_psuedo_legal_moves();
        let mut legal_moves = Vec::new();

        // If we are in check, we need to filter out all moves that don't
        // protect the king
        let check_vec = self.has_check();
        let double_check: bool;
        let check_square: Option<Square>;

        let check_piece = match check_vec {
            Some(square) => {
                if square.len() > 1 {
                    double_check = true;
                } else {
                    double_check = false;
                }

                check_square = Some(square[0]);
                Some(self.get_square(&square[0]))
            },
            None => {double_check = false; check_square = None; None},
        };

        let king = if self.turn == WHITE { WHITE_KING } else { BLACK_KING };
        let blockable_pieces = [WHITE_QUEEN, BLACK_QUEEN, WHITE_ROOK, BLACK_ROOK, WHITE_BISHOP, BLACK_BISHOP];

        for m in moves {
            // We are moving the king
            if self.get_square(&m.from) == king {
                if self.square_has_check(m.to).is_some() {
                    continue;
                }

                legal_moves.push(m);
            // We are moving a piece other than the king
            // but we are in check
            } else if check_square.is_some() {
                // We cannot resolve the check by taking a piece if multiple
                // pieces are checking the king at the same time
                if double_check {
                    continue;
                }

                // We are taking the piece that is checking us without
                // creating a new discovered check
                if m.to == check_square.unwrap() && !self.creates_discovered_attack(m) {
                    legal_moves.push(m);
                    continue;
                }

                // It is possible to block the check
                if blockable_pieces.contains(&check_piece.unwrap()) {
                    let king_square = self.piece_positions[king as usize][0];
                    // We are moving to a square that will block the check
                    // Strict triple aligned ensures that p2 is between p1 and p3 (and therefore blocks the check)
                    if self.is_strict_triple_aligned(king_square, m.to, check_square.unwrap()) {
                        if !self.creates_discovered_attack(m) {
                            legal_moves.push(m);
                        }
                    }
                }

                // It is possible to capture the piece that is checking us by 
                // taking it with en passant.
                let en_passant_square = self.is_en_passant_move(m);
                if en_passant_square.is_some() {
                    if en_passant_square.unwrap() == check_square.unwrap()
                       && !self.creates_discovered_attack(m) {
                        legal_moves.push(m);
                    }
                }
            // We are moving a piece other than the king
            // and we are not in check
            } else {
                // In this case we just have to make sure that we are not
                // creating a discovered attack on our king
                if !self.creates_discovered_attack(m) {
                    legal_moves.push(m);
                }
            }
        }

        legal_moves
    }

    fn is_diagonal(&self, from: Square, to: Square) -> bool {
        let row_diff = (from.row as i8 - to.row as i8).abs();
        let col_diff = (from.col as i8 - to.col as i8).abs();

        row_diff == col_diff
    }

    fn is_straight(&self, from: Square, to: Square) -> bool {
        let row_diff = (from.row as i8 - to.row as i8).abs();
        let col_diff = (from.col as i8 - to.col as i8).abs();

        row_diff == 0 || col_diff == 0
    }
    
    // TODO optimize this
    fn is_triple_aligned(&self, p1: Square, p2: Square, p3: Square) -> bool {
        if self.is_diagonal(p1, p2)
           && self.is_diagonal(p2, p3)
           && self.is_diagonal(p1, p3) {
            return true
        }

        if self.is_straight(p1, p2) 
           && self.is_straight(p2, p3)
           && self.is_straight(p1, p3) {
            return true
        }

        false
    }

    fn is_strict_triple_aligned(&self, p1: Square, p2: Square, p3: Square) -> bool {
        if self.is_diagonal(p1, p2)
           && self.is_diagonal(p2, p3)
           && self.is_diagonal(p1, p3) {
            // check if p2 is between p1 and p3
            if (p1.row < p2.row && p2.row < p3.row) || (p1.row > p2.row && p2.row > p3.row) {
                if (p1.col < p2.col && p2.col < p3.col) || (p1.col > p2.col && p2.col > p3.col) {
                    return true
                }
            }
        }

        if self.is_straight(p1, p2) 
           && self.is_straight(p2, p3)
           && self.is_straight(p1, p3) {
            // check if p2 is between p1 and p3
            if (p1.row <= p2.row && p2.row <= p3.row) || (p1.row >= p2.row && p2.row >= p3.row) {
                if (p1.col <= p2.col && p2.col <= p3.col) || (p1.col >= p2.col && p2.col >= p3.col) {
                    return true
                }
            }
        }

        false
    }

    // Get the current turn's king
    fn get_king_pos(&self) -> Square {
        return if self.turn == WHITE {
            self.piece_positions[WHITE_KING as usize][0]
        } else {
            self.piece_positions[BLACK_KING as usize][0]
        };
    }

    #[allow(dead_code)]
    fn is_move_on_line(&self, square_one: Square, square_two: Square, m: Move) -> bool {
        return self.is_triple_aligned(square_one, square_two, m.to);
    }

    // Returns the direction to go from squareOne to squareTwo
    fn get_dir(&self, square_one: Square, square_two: Square) -> (i8, i8) {
        let row_diff = square_one.row as i8 - square_two.row as i8;
        let col_diff = square_one.col as i8 - square_two.col as i8;

        let row_dir = if row_diff > 0 { -1 } else if row_diff < 0 { 1 } else { 0 };
        let col_dir = if col_diff > 0 { -1 } else if col_diff < 0 { 1 } else { 0 };

        (row_dir, col_dir)
    }

    fn is_en_passant_move(&self, m: Move) -> Option<Square> {
        let piece = self.get_square(&m.from);
        
        // The move cannot be an en passant move if the piece moving is not a pawn
        if piece != WHITE_PAWN && piece != BLACK_PAWN {
            return None;
        }

        // The move cannot be an en passant move if the pawn is not moving diagonally
        if m.to.col == m.from.col {
            return None;
        }

        // The move cannot be an en passant move if it is actively capturing a piece
        // on the to square
        if self.get_square(&m.to) != EMPTY_SQUARE {
            return None;
        }

        // Return the square that the pawn is capturing en passant
        if piece == WHITE_PAWN {
            Some(Square { row: m.to.row + 1, col: m.to.col })
        } else {
            Some(Square { row: m.to.row - 1, col: m.to.col })
        }
    }

    fn creates_discovered_attack(&self, m: Move) -> bool {
        // Check if the piece we are moving is pinned
        // If it is, we need to make sure that we are moving
        // it along the line of the pin

        // If the piece is not pinned, we can move it anywhere

        // The algorithm to do this:
        //  1. Are we on a diagonal/straight line to our king?
        //  (if we are on the line and the move moves along the line we can skip all this nonsense)
        //  2. If we are, scan along this line to see if there is an enemy piece
        //     that could attack our king if we were not there (stop if you find a friendly piece / OB)
        //     (if this is an en passant move treat the en passant square as an empty square)
        // 3. If we found a piece, scan back towards our king to see if there is another piece blocking the attack
        //    (stop if we find a piece / get to the king)
        //    (if this is an en passant move treat the en passant square as an empty square)
        // 4. If we found no blocking piece, then we are pinned. We can only move along the line of the pin.

        let king_pos = self.get_king_pos();
        let king_val = self.get_square(&king_pos);

        let is_diag = self.is_diagonal(m.from, king_pos);
        let is_straight = self.is_straight(m.from, king_pos);

        let en_passant_square = self.is_en_passant_move(m);

        // We could be diagonally pinned
        if is_diag || is_straight {
            // If we are moving along the line of the pin, we are good no matter what
            if self.is_triple_aligned(king_pos, m.from, m.to) {
                return false;
            }

            // Kind of weird but it keeps target_pieces in the right scope
            let target_pieces: [u8; 2];

            if is_diag {
                target_pieces = if king_val == WHITE_KING {
                    [BLACK_QUEEN, BLACK_BISHOP]
                } else {
                    [WHITE_QUEEN, WHITE_BISHOP]
                };
            } else {
                target_pieces = if king_val == WHITE_KING {
                    [BLACK_QUEEN, BLACK_ROOK]
                } else {
                    [WHITE_QUEEN, WHITE_ROOK]
                };
            }

            // Scan along the line away from the king to see if there is an enemy piece that could attack our king
            let (row_dir, col_dir) = self.get_dir(king_pos, m.from); // this gives the direction away from the king

            let mut curr_row = m.from.row as i8;
            let mut curr_col = m.from.col as i8;

            curr_row += row_dir;
            curr_col += col_dir;

            for _ in 0..8 {
                let val_at_target = self.get_square_ind(curr_row as i8, curr_col as i8);

                // If we hit an empty square, stop the search, there is no piece that could attack our king
                if val_at_target == OUT_OF_BOUNDS {
                    return false;
                // If we found an enemy piece break from the search
                } else if target_pieces.contains(&val_at_target) {
                    // There is an enemy piece that could attack
                    break;
                // If we are on the en passant square treat it as an empty square
                } else if let Some(square) = en_passant_square {
                    if square.row as i8 == curr_row && square.col as i8 == curr_col {
                        curr_row += row_dir;
                        curr_col += col_dir;

                        continue;
                    // If it wasn't the en passant square and it wasn't empty stop the search,
                    // this piece will block an attack if there is one
                    } else if val_at_target != EMPTY_SQUARE {
                        return false;
                    }
                // There is a piece that will block the attack
                } else if val_at_target != EMPTY_SQUARE {
                    // We found a piece that cannot attack the king, break
                    return false;
                }

                curr_row += row_dir;
                curr_col += col_dir;
            }

            // If we did not return false yet we must have found an enemy piece
            // Now we need to scan back towards the king to see if there is another piece besides us blocking the attack

            curr_row = m.from.row as i8;
            curr_col = m.from.col as i8;

            curr_row -= row_dir;
            curr_col -= col_dir;

            for _ in 0..8 {
                let val_at_target = self.get_square_ind(curr_row as i8, curr_col as i8);

                if val_at_target == OUT_OF_BOUNDS {
                    panic!("This should never happen");
                } else if val_at_target == king_val {
                    // We found the king first therefore we are pinned
                    // We already know the move is not along this line so we must be creating
                    // a discovered attack by moving this piece
                    return true;
                } else if let Some(square) = en_passant_square {
                    if square.row as i8 == curr_row && square.col as i8 == curr_col {
                        curr_row -= row_dir;
                        curr_col -= col_dir;

                        continue;
                    // If it wasn't the en passant square and it wasn't empty stop the search,
                    // this piece will block an attack if there is one
                    } else if val_at_target != EMPTY_SQUARE {
                        return false;
                    }
                } else if val_at_target != EMPTY_SQUARE {
                    // We found a piece that will block the attack
                    return false;
                }

                curr_row -= row_dir;
                curr_col -= col_dir;
            }
        }

        match en_passant_square {
            Some(square) => return self.removing_creates_check(square),
            None => return false,
        }
    }

    fn removing_creates_check(&self, square: Square) -> bool {
        let king_square = if self.turn { self.piece_positions[WHITE_KING as usize][0] } 
                                  else { self.piece_positions[BLACK_KING as usize][0] };

        // Check for knights
        let knight_attacks = self.is_knight_attacking(king_square, !self.turn);

        match knight_attacks {
            Some(_) => return true,
            None => (),
        }

        let diagonal_attacks = self.is_diagonal_attacking(king_square, !self.turn, Some(square));
        match diagonal_attacks {
            Some(_) => return true,
            None => (),
        }

        let straight_attacks = self.is_straight_attacking(king_square, !self.turn, Some(square));
        match straight_attacks {
            Some(_) => return true,
            None => (),
        }

        false
    }

    // Returns true if a knight of 'color' is attacking 'target_square'
    fn is_knight_attacking(&self, target_square: Square, color: bool) -> Option<Square> {
        let x_offsets = [1, 2, 2, 1, -1, -2, -2, -1];
        let y_offsets = [2, 1, -1, -2, -2, -1, 1, 2];

        let target_piece = if color == WHITE { WHITE_KNIGHT } else { BLACK_KNIGHT };

        for i in 0..x_offsets.len() {
            let to_row = target_square.row as i8 + y_offsets[i];
            let to_col = target_square.col as i8 + x_offsets[i];

            let piece_at_square = self.get_square_ind(to_row, to_col);

            if piece_at_square == target_piece {
                return Some(Square {
                    row: to_row as u8,
                    col: to_col as u8,
                });
            }
        }

        return None
    }

    // Returns true if a diagonal sliding piece of 'color' is attacking 'target_square'
    fn is_diagonal_attacking(&self, target_square: Square, color: bool, ignore_square: Option<Square>) -> Option<Square> {
        let x_dirs = [1, 1, -1, -1];
        let y_dirs = [1, -1, 1, -1];

        let target_pieces = if color == WHITE {
            [WHITE_QUEEN, WHITE_BISHOP]
        } else {
            [BLACK_QUEEN, BLACK_BISHOP]
        };

        let short_target_piece = if color == WHITE { WHITE_PAWN } else { BLACK_PAWN };
        let king_target_piece = if color == WHITE { WHITE_KING } else { BLACK_KING };

        // The friendly king is the opposite color from the attacking piece
        // The friendly king cannot block enemy attacks
        let friendly_king = if color == BLACK { WHITE_KING } else { BLACK_KING };

        for i in 0..x_dirs.len() {
            let mut to_row = target_square.row as i8;
            let mut to_col = target_square.col as i8;

            for j in 1..8 {
                to_row += y_dirs[i];
                to_col += x_dirs[i];

                let piece_at_square = self.get_square_ind(to_row, to_col);

                if piece_at_square == EMPTY_SQUARE {
                    continue;
                } else if ignore_square.is_some() && 
                    ignore_square.unwrap().row as i8 == to_row &&
                    ignore_square.unwrap().col as i8 == to_col {
                    continue;
                } else if j == 1 && piece_at_square == short_target_piece {
                    // Make sure that the pawn is attacking from the correct direction
                    if short_target_piece == WHITE_PAWN {
                        if to_row == target_square.row as i8 + 1 {
                            return Some(Square {
                                row: to_row as u8,
                                col: to_col as u8,
                            });
                        // The pawn is not able to attack us, however it will block
                        // the attack from this direction
                        } else {
                            break;
                        }
                    } else {
                        if to_row == target_square.row as i8 - 1 {
                            return Some(Square {
                                row: to_row as u8,
                                col: to_col as u8,
                            });
                        // The pawn is not able to attack us, however it will block
                        // the attack from this direction
                        } else {
                            break;
                        }
                    }
                } else if target_pieces.contains(&piece_at_square) ||  (j == 1 && piece_at_square == king_target_piece) {
                    return Some(Square {
                        row: to_row as u8,
                        col: to_col as u8,
                    });
                // We found a piece that will block the attack
                } else if piece_at_square != friendly_king {
                    break;
                }
            }
        }

        return None
    }

    // Returns true if a straight sliding piece of 'color' is attacking 'target_square'
    // This one is a bit simpler than the diagonal one because we don't have to worry
    // about the pawns
    fn is_straight_attacking(&self, target_square: Square, color: bool, ignore_square: Option<Square>) -> Option<Square> {
        let x_dirs = [1, -1, 0, 0];
        let y_dirs = [0, 0, 1, -1];

        let target_pieces = if color == WHITE {
            [WHITE_QUEEN, WHITE_ROOK]
        } else {
            [BLACK_QUEEN, BLACK_ROOK]
        };

        // The friendly king is the opposite color from the attacking piece
        // The friendly king cannot block enemy attacks
        let friendly_king = if color == BLACK { WHITE_KING } else { BLACK_KING };

        for i in 0..x_dirs.len() {
            let mut to_row = target_square.row as i8;
            let mut to_col = target_square.col as i8;

            for j in 1..8 {
                to_row += y_dirs[i];
                to_col += x_dirs[i];

                let piece_at_square = self.get_square_ind(to_row, to_col);

                let king_target_piece = if color == WHITE { WHITE_KING } else { BLACK_KING };

                if piece_at_square == EMPTY_SQUARE {
                    continue;
                } else if ignore_square.is_some() && 
                    ignore_square.unwrap().row as i8 == to_row &&
                    ignore_square.unwrap().col as i8 == to_col {
                    continue;
                } else if target_pieces.contains(&piece_at_square) || (j == 1 && piece_at_square == king_target_piece) {
                    return Some(Square {
                        row: to_row as u8,
                        col: to_col as u8,
                    });
                } else if piece_at_square != friendly_king {
                    break;
                }
            }
        }

        return None
    }

    /*
     * Board.has_check() returns true if the player whose turn it is is in check
     * Relies on three helper functions to check knight moves, diagonal moves, and straight moves.
     * 100% sure to not change the state of the board.
     * Relies on the fact that the king is stored in the piece positions array.
     * Also relies on the fact that the turn is stored in the board struct.
     */
    fn has_check(&self) -> Option<Vec<Square>> {
        let king_square;

        if self.turn {
            if self.piece_positions[WHITE_KING as usize].len() == 0 {
                println!("White king is missing from the cache... panic time!");
                println!("Prepanic info:");
                self.print();
                println!("Cache: {:#?}", self.piece_positions);
                for i in self.history.len()-4..self.history.len() {
                    println!("History: {:#?}", self.history[i])
                }
            }
            king_square = self.piece_positions[WHITE_KING as usize][0]
        } else {
            if self.piece_positions[BLACK_KING as usize].len() == 0 {
                println!("Black king is missing from the cache... panic time!");
                println!("Prepanic info:");
                self.print();
                println!("Cache: {:#?}", self.piece_positions);
                for i in self.history.len()-4..self.history.len() {
                    println!("History: {:#?}", self.history[i])
                }
            }
            king_square = self.piece_positions[BLACK_KING as usize][0];
        }
        

        self.square_has_check(king_square)
    }

    fn square_has_check(&self, king_square: Square) -> Option<Vec<Square>> {
        let mut checks = Vec::new();

        // Check for knights
        let knight_attacks = self.is_knight_attacking(king_square, !self.turn);

        match knight_attacks {
            Some(square) => checks.push(square),
            None => (),
        }

        let diagonal_attacks = self.is_diagonal_attacking(king_square, !self.turn, None);
        match diagonal_attacks {
            Some(square) => checks.push(square),
            None => (),
        }

        let straight_attacks = self.is_straight_attacking(king_square, !self.turn, None);
        match straight_attacks {
            Some(square) => checks.push(square),
            None => (),
        }

        if checks.len() > 0 {
            return Some(checks);
        } else {
            return None;
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
        assert_eq!(board.fen(), "8/8/8/8/8/8/8/8 w - - 0 1");
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

        assert_eq!(board.get_hash(), 7467127324528350544);

        let current_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("g1f3").unwrap()).unwrap();
        board.push(Move::new_from_string("g8f6").unwrap()).unwrap();
        board.push(Move::new_from_string("c2c4").unwrap()).unwrap();

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

        // This tests the en passant square
        let current_fen = "rnbqkb1r/pppppppp/5n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq c3 0 2";
        assert_eq!(board.fen(), current_fen);

        board.push(Move::new_from_string("g7g6").unwrap()).unwrap();
        board.push(Move::new_from_string("b1c3").unwrap()).unwrap();
        board.push(Move::new_from_string("f8g7").unwrap()).unwrap();
        board.push(Move::new_from_string("d2d4").unwrap()).unwrap();
        board.push(Move::new_from_string("e8g8").unwrap()).unwrap();
        board.push(Move::new_from_string("c1f4").unwrap()).unwrap();

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

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

        assert_eq!(board.calculate_zobrist_hash(), board.get_hash());

        let current_fen = "1Q6/5pk1/2p3p1/1p2N2p/1b5P/1bn5/2r3P1/2K5 w - - 16 42";
        assert_eq!(board.fen(), current_fen);

        // Pop them all off the stack
        loop {
            if let Err(_) = board.pop() {
                break;
            }
        }

        assert_eq!(board.get_hash(), 7467127324528350544);

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

    #[test]
    fn test_knight_attacking() {
        let mut board = Board::new_empty();

        board.set_square(&Square {row: RANK_THREE, col: FILE_B}, WHITE_KNIGHT);

        let test_square = Square {row: RANK_FIVE, col: FILE_C}; // true
        assert!(board.is_knight_attacking(test_square, WHITE).is_some());

        let test_square = Square {row: RANK_FOUR, col: FILE_C}; // false
        assert!(!board.is_knight_attacking(test_square, WHITE).is_some());
    }

    #[test]
    fn test_diagonal_attacking() {
        let mut board = Board::new_from_fen("8/5p2/6K1/7q/1k6/8/8/1b6 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_G};
        assert!(board.is_diagonal_attacking(test_square, BLACK, None).is_some());

        board.import_from_fen("8/5p2/6K1/8/1k6/8/8/1b6 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_G};
        assert!(board.is_diagonal_attacking(test_square, BLACK, None).is_some());

        board.import_from_fen("8/5p2/6K1/8/1k6/8/8/8 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_G};
        assert!(board.is_diagonal_attacking(test_square, BLACK, None).is_some());

        board.import_from_fen("8/8/6K1/8/1k6/8/8/8 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_G};
        assert!(!board.is_diagonal_attacking(test_square, BLACK, None).is_some());
    }

    #[test]
    fn test_straight_attacking() {
        let mut board = Board::new_from_fen("7r/8/3K3q/5P2/8/b5k1/3r4/8 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_D};
        assert!(board.is_straight_attacking(test_square, BLACK, None).is_some());

        board.import_from_fen("7r/8/3K1P1q/8/8/b5k1/3r4/8 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_D};
        assert!(board.is_straight_attacking(test_square, BLACK, None).is_some());

        board.import_from_fen("7r/8/3K1P1q/8/8/b5k1/4r3/8 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_D};
        assert!(!board.is_straight_attacking(test_square, BLACK, None).is_some());

        board.import_from_fen("1q5r/8/3K1P1q/8/8/b5k1/4r3/8 w - - 0 1").unwrap();
        let test_square = Square {row: RANK_SIX, col: FILE_D};
        assert!(!board.is_straight_attacking(test_square, BLACK, None).is_some());
    }

    #[test]
    fn test_has_check() {
        let mut board = Board::new_from_fen("1q6/8/3K1P1q/8/8/b2r2k1/4r3/8 w - - 0 1").unwrap();
        assert!(board.has_check().is_some());

        board.import_from_fen("1q6/8/3K1P1q/8/3B4/b2r2k1/4r3/8 w - - 0 1").unwrap();
        assert!(board.has_check().is_some());

        board.import_from_fen("1q6/8/3K1P1q/2Q5/3B4/b2r2k1/4r3/8 w - - 0 1").unwrap();
        assert!(board.has_check().is_some());

        board.import_from_fen("1q6/2P5/3K1P1q/2Q5/3B4/b2r2k1/4r3/8 w - - 0 1").unwrap();
        assert!(!board.has_check().is_some());

        board.import_from_fen("1q6/2P5/3K1P1q/2Q5/2nB4/b2r2k1/4r3/8 w - - 0 1").unwrap();
        assert!(board.has_check().is_some());
    }

    #[test]
    fn test_creates_discovered_attack() {
        let mut board = Board::new_from_fen("8/7b/8/5Q2/8/3K4/8/1k6 w - - 0 1").unwrap();
        let test_move = Move::new_from_string("f5e5").unwrap();
        assert!(board.creates_discovered_attack(test_move));

        board.import_from_fen("8/7b/8/5Q2/4N3/3K4/8/1k6 w - - 0 1").unwrap();
        let test_move = Move::new_from_string("f5e5").unwrap();
        assert!(!board.creates_discovered_attack(test_move));

        board.import_from_fen("8/8/2K5/8/8/8/8/1k1r1Q2 b - - 0 1").unwrap();
        let test_move = Move::new_from_string("d1d2").unwrap();
        assert!(board.creates_discovered_attack(test_move));

        board.import_from_fen("8/8/2K5/8/8/8/8/1k1r1Q2 b - - 0 1").unwrap();
        let test_move = Move::new_from_string("d1e1").unwrap();
        assert!(!board.creates_discovered_attack(test_move));
    }

    #[test]
    fn test_new_legal_moves() {
        let board = Board::new_from_fen("8/8/2K5/8/8/8/8/1k1n1Q2 b - - 0 1").unwrap();
        let legal_moves = board.gen_legal_moves();
        assert_eq!(legal_moves.len(), 5);

        let board = Board::new_from_fen("8/8/2K5/1R6/8/8/8/1k1n1Q2 b - - 0 1").unwrap();
        let legal_moves = board.gen_legal_moves();
        assert_eq!(legal_moves.len(), 4);
    }

    fn perft(starting_board: &mut Board, depth: u8, top_depth: u8, verbosity: u32) -> u64 {
        // Verbosity 0 = no output
        // Verbosity 1 = output perft results for top depth without subtotals
        // Verbosity 2 = output perft results for top depth with subtotals
        // Verbosity 3 = output perft results for all depths without subtotals
        // Verbosity 4 = output perft results for all depths with subtotals

        if depth == 0 {
            return 1;
        }

        if depth == top_depth && verbosity > 0 {
            println!("Starting perft with depth {}", depth);
        }

        let mut total = 0;

        let spacer = "  ".repeat((top_depth - depth) as usize);

        let mut moves = starting_board.gen_legal_moves();

        moves.sort_by(|a, b| a.get_move_string().cmp(&b.get_move_string()));

        let mut last_starting_file = FILE_A;

        let mut subcount = 0;
        let mut subtotal = 0;

        for elem in moves {
            //let cloned_board = starting_board.clone();
            starting_board.push(elem).unwrap();

            let count = perft(starting_board, depth - 1, top_depth, verbosity);

            if elem.from.col != last_starting_file {
                if (depth == top_depth && verbosity > 1) || verbosity > 3 {
                    println!("{}Subtotal: {} (from {} moves)", spacer, subtotal, subcount);
                    println!("");
                }
                subcount = 0;
                subtotal = 0;
                last_starting_file = elem.from.col;
            }

            if (depth == top_depth && verbosity > 0) || verbosity > 2 {
                println!("{}{}: {}", spacer, elem.get_move_string(), count);
            }

            subcount += 1;
            subtotal += count;

            total += count;

            starting_board.pop().unwrap();
            /*if starting_board.fen() != cloned_board.fen() {
                println!("Original FEN: {}", cloned_board.fen());
                println!(" New FEN STR: {}", starting_board.fen());
                println!("Move: {}", elem.get_move_string());
                println!("Full move details: {:#?}", elem);
                panic!("Board state mismatch after pop");
            }*/
            //assert_eq!(starting_board.fen(), cloned_board.fen());
            //assert!(starting_board.is_other_cache_equivalent(&cloned_board));
        }

        if depth == top_depth && verbosity > 0 {
            println!("Total: {}", total);
        }

        total
    }

    #[test]
    fn perft_pos1_d1() {
        let mut board = Board::new();
        let count = perft(&mut board, 1, 1, 1);
        assert_eq!(count, 20);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos1_d2() {
        let mut board = Board::new();
        let count = perft(&mut board, 2, 2, 1);
        assert_eq!(count, 400);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos1_d3() {
        let mut board = Board::new();
        let count = perft(&mut board, 3, 3, 1);
        assert_eq!(count, 8_902);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos1_d4() {
        let mut board = Board::new();
        let count = perft(&mut board, 4, 4, 1);
        assert_eq!(count, 197_281);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos2_d1() {
        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        let count = perft(&mut board, 1, 1, 1);
        assert_eq!(count, 48);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos2_d2() {
        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        let count = perft(&mut board, 2, 2, 1);
        assert_eq!(count, 2039);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos2_d3() {
        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        let count = perft(&mut board, 3, 3, 1);
        assert_eq!(count, 97862);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos2_d4() {
        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        let count = perft(&mut board, 4, 4, 1);
        assert_eq!(count, 4085603);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos3_d1() {
        let mut board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        let count = perft(&mut board, 1, 1, 2);
        assert_eq!(count, 14);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos3_d2() {
        let mut board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        let count = perft(&mut board, 2, 2, 2);
        assert_eq!(count, 191);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos3_d3() {
        let mut board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        let count = perft(&mut board, 3, 3, 2);
        assert_eq!(count, 2812);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos3_d4() {
        let mut board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        let count = perft(&mut board, 4, 4, 2);
        assert_eq!(count, 43238);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos3_d5() {
        let mut board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        let count = perft(&mut board, 5, 5, 2);
        assert_eq!(count, 674624);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos4_d4() {
        let mut board = Board::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        let count = perft(&mut board, 4, 4, 2);
        assert_eq!(count, 422333);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos4_mirrored_d4() {
        let mut board = Board::new_from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1").unwrap();
        let count = perft(&mut board, 4, 4, 2);
        assert_eq!(count, 422333);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos5_d4() {
        let mut board = Board::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        let count = perft(&mut board, 4, 4, 2);
        assert_eq!(count, 2_103_487);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn perft_pos6_d4() {
        let mut board = Board::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();
        let count = perft(&mut board, 4, 4, 2);
        assert_eq!(count, 3_894_594);
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn test_checkmate() {
        let board = Board::new_from_fen("1k6/1Q6/2K5/8/8/8/8/8 b - - 1 1").unwrap();
        assert!(board.checkmate());
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn test_stalemate() {
        let board = Board::new_from_fen("k7/2Q5/1K6/8/8/8/8/8 b - - 3 2").unwrap();
        assert!(board.stalemate());
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    #[test]
    fn test_promotion() {
        let mut board = Board::new_from_fen("8/1R5P/8/3k4/8/8/8/7K w - - 0 1").unwrap();
        board.push(Move::new_from_string("h7h8q").unwrap()).unwrap();

        board.print();

        assert_eq!(board.fen(), "7Q/1R6/8/3k4/8/8/8/7K b - - 0 1");
        assert_eq!(board.get_hash(), board.calculate_zobrist_hash());
    }

    // Somehow this desync happens in the middle of a game
    // but it never happens in any of the perft tests
    // Future me: the reason it happens is because you allow the king to remain
    // in check after a move when there is a double check and one of the pieces
    // checking is taken (but not the other one).
    #[test]
    fn test_cache_desync() {
        let mut board = Board::new();
        // Cause a desync
        board.cells[0][4] = EMPTY_SQUARE;
        assert!(board.is_cache_desynced());

        // Fix the desync
        board.recache();
        assert!(!board.is_cache_desynced());


        let mut board = Board::new();
        // Cause a different desync
        board.piece_positions[BLACK_KING as usize].clear();
        assert!(board.is_cache_desynced());

        // Fix the desync
        board.recache();
        assert!(!board.is_cache_desynced());
    }

    #[test]
    fn recreate_king_bug() {
        let board = Board::new_from_fen("r1b1r1n1/p7/1p1k4/2p5/4N3/4P3/PPK5/3R4 b - - 5 34").unwrap();

        let moves = board.gen_legal_moves();

        assert_eq!(moves.len(), 5);
    }

    #[test]
    fn test_psuedo_random() {
        let keys = get_zobrist_keys();

        assert_eq!(keys[0], 5717137336360897368);
        assert_eq!(keys[206], 6427430163083693146);
        assert_eq!(keys[399], 15028282355671535552);
        assert_eq!(keys[617], 4186175680712125785);
        assert_eq!(keys[792], 6153439440954588254);
    }

    #[test]
    fn test_zobrist_hashing() {
        let board = Board::new();
        println!("{}", board.get_hash());
        assert_eq!(board.get_hash(), 7467127324528350544);
    }
}
