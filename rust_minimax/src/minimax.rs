use crate::board;
use std::fmt::Error;

pub struct ChessAI {
    board: board::Board,
    my_color: bool,
    debug_mode: bool,
    start_time: Option<std::time::Instant>,
    nodes_expanded: u64,
    tt_table: Vec<Option<PositionScore>>,
    use_tt: bool,
    perf_test: bool,
    time_scoring: std::time::Duration,
    time_legal_moves: std::time::Duration,
    // 0: Checkmate test
    // 1: Stalemate test
    // 2: Repitition test
    // 3: Other tests
    time_scoring_vec: Vec<std::time::Duration>,
}

#[derive(Copy, Clone)]
struct PositionScore {
    score: i32,
    depth: u8,
    hash: u64,
}

// Misanalyzed position (8/P4k2/5p2/7q/4P3/3PK3/R7/8 b - - 12 64) and lost, it played Queen H6 and it should have played Queen H3

/* 
 * This struct stores the state of the minimax algorithm at a given point.
 */
#[derive(Debug)]
pub struct TreeDecision {
    pub best_move: Option<board::Move>,
    pub score: i32,
}

const WHITE_PAWN_MAP: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5,  5, 10, 25, 25, 10,  5,  5],
    [0,  0,  0, 20, 20,  0,  0,  0],
    [5, -5,-10,  0,  0,-10, -5,  5],
    [5, 10, 10,-20,-20, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

const BLACK_PAWN_MAP: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [5, 10, 10,-20,-20, 10, 10,  5],
    [5, -5,-10,  0,  0,-10, -5,  5],
    [0,  0,  0, 20, 20,  0,  0,  0],
    [5,  5, 10, 25, 25, 10,  5,  5],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

const KNIGHT_MAP: [[i32; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
];

const WHITE_BISHOP_MAP: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

const BLACK_BISHOP_MAP: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

const WHITE_ROOK_MAP: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [0,  0,  0,  5,  5,  0,  0,  0],
];

const BLACK_ROOK_MAP: [[i32; 8]; 8] = [
    [0,  0,  0,  5,  5,  0,  0,  0],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

const WHITE_QUEEN_MAP: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [-5,  0,  5,  5,  5,  5,  0, -5],
    [0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
];

const BLACK_QUEEN_MAP: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [0,  0,  5,  5,  5,  5,  0, -5],
    [-5,  0,  5,  5,  5,  5,  0, -5],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
];

const WHITE_KING_MAP: [[i32; 8]; 8] = [
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-15, -17, -20, -20, -20, -20, -17, -15],
    [-10, -12, -15, -20, -20, -15, -12, -10],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [20, 20, 15, -10, -10, 15, 20, 20],
];

const ENDGAME_KING_MAP: [[i32; 8]; 8] = [
    [-50,-40,-30,-20,-20,-30,-40,-50],
    [-30,-20,-10,  0,  0,-10,-20,-30],
    [-30,-10, 20, 30, 30, 20,-10,-30],
    [-30,-10, 30, 40, 40, 30,-10,-30],
    [-30,-10, 30, 40, 40, 30,-10,-30],
    [-30,-10, 20, 30, 30, 20,-10,-30],
    [-30,-30,  0,  0,  0,  0,-30,-30],
    [-50,-30,-30,-30,-30,-30,-30,-50],
];

const BLACK_KING_MAP: [[i32; 8]; 8] = [
    [20, 20, 15, -10, -10, 15, 20, 20],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [-10, -12, -15, -20, -20, -15, -12, -10],
    [-15, -17, -20, -20, -20, -20, -17, -15],
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-20, -20, -20, -20, -20, -20, -20, -20],
    [-20, -20, -20, -20, -20, -20, -20, -20],
];

fn opening_position_differential(board: &board::Board, debug: bool) -> i32 {
    if debug { println!("\nNormal Position Differential ====================") }
    let mut differential: i32 = 0;
    // White pawns
    let white_pawns = board.get_pawn_list(board::WHITE);
    for pawn in white_pawns {
        differential += WHITE_PAWN_MAP[pawn.row as usize][pawn.col as usize];
    }

    if debug { println!("After White Pawns: {}", differential) }

    // Black pawns
    let black_pawns = board.get_pawn_list(board::BLACK);
    for pawn in black_pawns {
        differential -= BLACK_PAWN_MAP[pawn.row as usize][pawn.col as usize];
    }

    if debug { println!("After Black Pawns: {}", differential) }

    // White knights
    let white_knights = board.get_knight_list(board::WHITE);
    for knight in white_knights {
        differential += KNIGHT_MAP[knight.row as usize][knight.col as usize];
    }

    if debug { println!("After White Knights: {}", differential) }

    // Black knights
    let black_knights = board.get_knight_list(board::BLACK);
    for knight in black_knights {
        differential -= KNIGHT_MAP[knight.row as usize][knight.col as usize];
    }

    if debug { println!("After Black Knights: {}", differential) }

    // White bishops
    let white_bishops = board.get_bishop_list(board::WHITE);
    for bishop in white_bishops {
        differential += WHITE_BISHOP_MAP[bishop.row as usize][bishop.col as usize];
    }

    if debug { println!("After White Bishops: {}", differential) }

    // Black bishops
    let black_bishops = board.get_bishop_list(board::BLACK);
    for bishop in black_bishops {
        differential -= BLACK_BISHOP_MAP[bishop.row as usize][bishop.col as usize];
    }

    if debug { println!("After Black Bishops: {}", differential) }

    // White rooks
    let white_rooks = board.get_rook_list(board::WHITE);
    for rook in white_rooks {
        differential += WHITE_ROOK_MAP[rook.row as usize][rook.col as usize];
    }

    if debug { println!("After White Rooks: {}", differential) }

    // Black rooks
    let black_rooks = board.get_rook_list(board::BLACK);
    for rook in black_rooks {
        differential -= BLACK_ROOK_MAP[rook.row as usize][rook.col as usize];
    }

    if debug { println!("After Black Rooks: {}", differential) }

    // White queens
    let white_queens = board.get_queen_list(board::WHITE);
    for queen in white_queens {
        differential += WHITE_QUEEN_MAP[queen.row as usize][queen.col as usize];
    }

    if debug { println!("After White Queens: {}", differential) }

    // Black queens
    let black_queens = board.get_queen_list(board::BLACK);
    for queen in black_queens {
        differential -= BLACK_QUEEN_MAP[queen.row as usize][queen.col as usize];
    }

    if debug { println!("After Black Queens: {}", differential) }

    // White king
    let white_king = board.get_king_square(board::WHITE);
    differential += WHITE_KING_MAP[white_king.row as usize][white_king.col as usize];

    if debug { println!("After White King: {}", differential) }

    // Black king
    let black_king = board.get_king_square(board::BLACK);
    differential -= BLACK_KING_MAP[black_king.row as usize][black_king.col as usize];

    if debug { println!("After Black King: {}", differential) }

    // Return the differential
    differential / 2
}

fn endgame_position_differential(board: &board::Board, debug: bool) -> i32 {
    if debug { println!("\nEndgame Position Differential ====================") }
    let mut differential: i32 = 0;
    // White pawns
    let white_pawns = board.get_pawn_list(board::WHITE);
    for pawn in white_pawns {
        differential += WHITE_PAWN_MAP[pawn.row as usize][pawn.col as usize];
    }

    if debug { println!("After White Pawns: {}", differential) }

    // Black pawns
    let black_pawns = board.get_pawn_list(board::BLACK);
    for pawn in black_pawns {
        differential -= BLACK_PAWN_MAP[pawn.row as usize][pawn.col as usize];
    }

    if debug { println!("After Black Pawns: {}", differential) }

    // White king
    let white_king = board.get_king_square(board::WHITE);
    differential += ENDGAME_KING_MAP[white_king.row as usize][white_king.col as usize];

    if debug { println!("After White King: {}", differential) }

    // Black king
    let black_king = board.get_king_square(board::BLACK);
    differential -= ENDGAME_KING_MAP[black_king.row as usize][black_king.col as usize];

    if debug { println!("After Black King: {}", differential) }

    // Return the differential
    differential
}

// The pawns should be stored sorted by row and column in the board struct already
// Then we could figure out how many pawns are on each row easily
// We can also figure out the pawn on the left and the right of each pawn to figure out
// isolation
pub fn pawn_structure_differential(board: &board::Board, debug: bool) -> i32 {
    let mut differential: i32 = 0;

    let white_pawn_counts = board.get_white_pawn_cols();
    let black_pawn_counts = board.get_black_pawn_cols();

    for i in 0..8 {
        if white_pawn_counts[i] > 1 {
            // Doubled / Tripled / Quadrupled pawns are bad
            differential -= 20 * (white_pawn_counts[i] - 1);
            if debug { println!("White doubled pawns on col {}: {}", i, white_pawn_counts[i]) }
        }

        if black_pawn_counts[i] > 1 {
            // Doubled / Tripled / Quadrupled pawns are bad
            differential += 20 * (black_pawn_counts[i] - 1);
            if debug { println!("Black doubled pawns on col {}: {}", i, black_pawn_counts[i]) }
        }

        // White Isolated Pawns
        if white_pawn_counts[i] > 0 {
            let mut isolated_left = true;
            if i > 0 && white_pawn_counts[i - 1] > 0 {
                isolated_left = false;
            }

            let mut isolated_right = true;
            if i < 7 && white_pawn_counts[i + 1] > 0 {
                isolated_right = false;
            }

            if isolated_left && isolated_right {
                differential -= 20;
                if debug { println!("White isolated pawn on col {}", i) }
            }
        }

        // Black Isolated Pawns
        if black_pawn_counts[i] > 0 {
            let mut isolated_left = true;
            if i > 0 && black_pawn_counts[i - 1] > 0 {
                isolated_left = false;
            }

            let mut isolated_right = true;
            if i < 7 && black_pawn_counts[i + 1] > 0 {
                isolated_right = false;
            }

            if isolated_left && isolated_right {
                differential += 20;
                if debug { println!("Black isolated pawn on col {}", i) }
            }
        }

        // White Passed Pawns
        // Passed pawns count as an extra pawn (is that good??)
        if white_pawn_counts[i] > 0 {
            let mut passed = true;
            let left_side = if i > 0 { i - 1 } else { 0 };
            let right_side = if i < 7 { i + 1 } else { 7 };

            for j in left_side..=right_side {
                if black_pawn_counts[j] > 0 {
                    passed = false;
                }
            }

            if passed {
                differential += 100;
                if debug { println!("White passed pawn on col {}", i) }
            }
        }

        // Black Passed Pawns
        // Passed pawns count as an extra pawn (is that good??)
        if black_pawn_counts[i] > 0 {
            let mut passed = true;
            let left_side = if i > 0 { i - 1 } else { 0 };
            let right_side = if i < 7 { i + 1 } else { 7 };

            for j in left_side..=right_side {
                if white_pawn_counts[j] > 0 {
                    passed = false;
                }
            }

            if passed {
                differential -= 100;
                if debug { println!("Black passed pawn on col {}", i) }
            }
        }
    }

    // King safety
    let white_king_col = board.get_king_square(board::WHITE).col;
    let left_side = if white_king_col > 0 { white_king_col - 1 } else { 0 };
    let right_side = if white_king_col < 7 { white_king_col + 1 } else { 7 };
    for i in left_side..=right_side {
        if white_pawn_counts[i as usize] == 0 {
            differential -= 40;
            if debug { println!("White king is unsafe") }
        }
    }

    let black_king_col = board.get_king_square(board::BLACK).col;
    let left_side = if black_king_col > 0 { black_king_col - 1 } else { 0 };
    let right_side = if black_king_col < 7 { black_king_col + 1 } else { 7 };
    for i in left_side..=right_side {
        if black_pawn_counts[i as usize] == 0 {
            differential += 40;
            if debug { println!("Black king is unsafe") }
        }
    }

    // Doubled pawns

    // A pawn is considered "isolated" if it has no friendly pawns on either side of it
    
    // A pawn is considered "doubled" if it has a friendly pawn on the same file

    // A pawn is considered "passed" if it has no enemy pawns on either side of it
    // and on the same file on the way to the other side of the board

    return differential;
}

// High scores are good for white, low scores are good for black
pub fn score_board(board: &mut board::Board, current_depth: i32, debug: bool, perf_time_vec: &mut Option<&mut Vec<std::time::Duration>>) -> i32 {
    // Extremely basic scoring function
    let mut is_checkmate = false;
    match perf_time_vec {
        Some(vec) => {
            let start_time = std::time::Instant::now();
            if board.checkmate() {
                is_checkmate = true;
            }
            let end_time = std::time::Instant::now();
            vec[0] += end_time - start_time;
        },
        None => {
            if board.checkmate() {
                is_checkmate = true;
            }
        }
    }
    if is_checkmate {
        if board.turn() {
            if debug { println!("Returning -1000000 because white is checkmated") }
            return -1000000 + current_depth;
        } else {
            if debug { println!("Returning 1000000 because black is checkmated") }
            return 1000000 - current_depth;
        }
    }

    // assert!(board.has_check_cache());
    // assert!(board.has_move_cache());
    // println!("Passed asserts");

    let mut is_stalemate = false;
    match perf_time_vec {
        Some(vec) => {
            let start_time = std::time::Instant::now();
            if board.stalemate() {
                is_stalemate = true;
            }
            let end_time = std::time::Instant::now();
            vec[1] += end_time - start_time;
        },
        None => {
            if board.stalemate() {
                is_stalemate = true;
            }
        }
    }
    if is_stalemate {
        if debug { println!("Returning 0 because stalemate") }
        return 0;
    }

    let mut is_repetition = false;
    match perf_time_vec {
        Some(vec) => {
            let start_time = std::time::Instant::now();
            if board.threefold_repetition() {
                is_repetition = true;
            }
            let end_time = std::time::Instant::now();
            vec[2] += end_time - start_time;
        },
        None => {
            if board.threefold_repetition() {
                is_repetition = true;
            }
        }
    }
    if is_repetition {
        if debug { println!("Returning 0 because threefold repitition") }
        return 0;
    }

    let start_time = std::time::Instant::now();

    let mut score = 0;
    score += board.get_pawn_differential() * 100;
    if debug { println!("Pawn differential: {}", board.get_pawn_differential()) }
    let (diff, bonus) = board.get_bishop_differential();
    score += diff * 300;
    score += (bonus * 300.0) as i32;
    if debug { println!("Bishop differential: {}", board.get_bishop_differential().0) }
    score += board.get_knight_differential() * 300;
    if debug { println!("Knight differential: {}", board.get_knight_differential()) }
    score += board.get_rook_differential() * 500;
    if debug { println!("Rook differential: {}", board.get_rook_differential()) }
    score += board.get_queen_differential() * 900;
    if debug { println!("Queen differential: {}", board.get_queen_differential()) }

    let phase = board.get_game_phase();

    if phase == 0 {
        score += opening_position_differential(board, debug) as i32;
        match perf_time_vec {
            Some(vec) => {
                let inner_stime = std::time::Instant::now();
                score += pawn_structure_differential(board, debug) as i32;
                vec[4] += std::time::Instant::now() - inner_stime;
            },
            None => {
                score += pawn_structure_differential(board, debug) as i32;
            }
        }
    } else if phase == 1 {
        score += opening_position_differential(board, debug) as i32 / 2;
        match perf_time_vec {
            Some(vec) => {
                let inner_stime = std::time::Instant::now();
                score += pawn_structure_differential(board, debug) as i32;
                vec[4] += std::time::Instant::now() - inner_stime;
            },
            None => {
                score += pawn_structure_differential(board, debug) as i32;
            }
        }
    } else if phase == 2 {
        score += endgame_position_differential(board, debug) as i32;
    }

    if debug { println!("Normal position differential: {}", opening_position_differential(board, false)) }

    match perf_time_vec {
        Some(vec) => {
            let end_time = std::time::Instant::now();
            vec[3] += end_time - start_time;
        },
        None => {}
    }

    return score;
}

impl ChessAI {
    #[allow(dead_code)]
    pub fn new() -> ChessAI {
        ChessAI {
            board: board::Board::new(),
            my_color: board::WHITE,
            debug_mode: false,
            start_time: None,
            nodes_expanded: 0,
            tt_table: vec![None; 100_000_000],
            use_tt: true,
            perf_test: false,
            time_scoring: std::time::Duration::from_secs(0),
            time_legal_moves: std::time::Duration::from_secs(0),
            time_scoring_vec: vec![std::time::Duration::from_secs(0); 8],
        }
    }

    pub fn new_with_color(color: bool) -> ChessAI {
        ChessAI {
            board: board::Board::new(),
            my_color: color,
            debug_mode: false,
            start_time: None,
            nodes_expanded: 0,
            tt_table: vec![None; 100_000_000],
            use_tt: true,
            perf_test: false,
            time_scoring: std::time::Duration::from_secs(0),
            time_legal_moves: std::time::Duration::from_secs(0),
            time_scoring_vec: vec![std::time::Duration::from_secs(0); 8],
        }
    }

    #[allow(dead_code)]
    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    #[allow(dead_code)]
    pub fn disable_debug(&mut self) {
        self.debug_mode = false;
    }

    #[allow(dead_code)]
    pub fn import_position(&mut self, fen: &str) -> Result<(), Error> {
        match self.board.import_from_fen(fen) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn push_move(&mut self, m: board::Move) -> Result<(), Error> {
        match self.board.push(m) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn print_internal_board(&self) {
        self.board.print();
    }

    pub fn reset_internal_board(&mut self) {
        self.board = board::Board::new();
    }

    #[allow(dead_code)]
    pub fn best_move(&mut self, depth: u8) -> Result<TreeDecision, Error> {
        if self.board.turn() != self.my_color {
            return Err(Error);
        }

        // Iterative deepening
        let result = self.minimax(depth, depth, self.my_color, -1000000, 1000000);

        Ok(result)
    }

    pub fn get_board_turn(&self) -> bool {
        self.board.turn()
    }

    pub fn best_move_iddfs(&mut self, time_allowed_secs: f64) -> Result<TreeDecision, Error> {
        let initial_start_time = Some(std::time::Instant::now()).unwrap();
        self.time_scoring = std::time::Duration::from_secs(0);
        self.time_legal_moves = std::time::Duration::from_secs(0);

        // Iterative deepening
        let mut result = TreeDecision {
            best_move: None,
            score: 0,
        };

        let mut times = Vec::new();

        // max depth
        let depth = 100;
        let mut score_vec: Option<Vec<TreeDecision>> = Option::from(None);

        for i in 1..depth {
            let start_time = Some(std::time::Instant::now()).unwrap();
            score_vec = Some(self.iddfs(i, self.board.turn(), score_vec));
            let end_time = Some(std::time::Instant::now()).unwrap();

            let elapsed = end_time.duration_since(start_time).as_millis();
            times.push(elapsed);

            let projected_ms = self.project_next_ms(&times);

            let next_end_time = end_time + std::time::Duration::from_millis(projected_ms as u64);

            if next_end_time.duration_since(initial_start_time).as_secs() as f64 > time_allowed_secs {
                println!("Breaking after depth {} with evaluation {}", i, score_vec.as_ref().unwrap()[0].score);

                break;
            }
        }

        result.best_move = score_vec.as_ref().unwrap()[0].best_move;
        result.score = score_vec.as_ref().unwrap()[0].score;

        Ok(result)
    }

    // Saves the score to the transposition table using the zobrist hash
    #[allow(dead_code)]
    fn save_to_tt_table(&mut self, depth: u8, score: i32, hash: u64) {
        let entry = PositionScore {
            hash,
            depth,
            score,
        };

        let index = (hash as usize % self.tt_table.len()) as usize;
        self.tt_table[index] = Some(entry);
    }

    // Returns the score from the transposition table using the zobrist hash if it exists
    #[allow(dead_code)]
    fn get_from_tt_table(&mut self, curr_depth: u8, hash: u64) -> Option<PositionScore> {
        let entry = self.tt_table[hash as usize % self.tt_table.len()];

        if entry.is_none() {
            return None;
        }

        let entry = entry.unwrap();

        if entry.depth >= curr_depth && entry.hash == hash {
            return Some(entry);
        }

        None
    }

    fn project_next_ms(&self, times: &Vec<u128>) -> u128 {
        if times.len() < 2 {
            return 0;
        }

        let mut avg_multiplier = 0;
        for i in 0..times.len() - 1 {
            if times[i] == 0 {
                continue;
            }
            avg_multiplier += times[i + 1] / times[i];
        }

        avg_multiplier /= times.len() as u128;

        let projected_ms = times[times.len() - 1] * avg_multiplier;

        return projected_ms;
    }

    #[allow(dead_code)]
    pub fn enable_perf_test(&mut self) {
        self.perf_test = true;
    }

    pub fn report_search_speed(&mut self) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            let elapsed_secs = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
            println!("{} nodes expanded in {} seconds", self.nodes_expanded, elapsed_secs);
            println!("{} nodes per second", self.nodes_expanded as f64 / elapsed_secs);
            println!("Percent spent generating legal moves: {}%", self.time_legal_moves.as_secs_f64() / elapsed_secs * 100.0);
            println!("Percent spent scoring: {}% ({}s)", self.time_scoring.as_secs_f64() / elapsed_secs * 100.0, self.time_scoring.as_secs_f64());
            println!("  - Checkmates: {}% ({}s, {})", self.time_scoring_vec[0].as_secs_f64() / elapsed_secs * 100.0, self.time_scoring_vec[0].as_secs_f64(), self.time_scoring_vec[4].as_nanos());
            println!("  - Stalemates: {}% ({}s, {})", self.time_scoring_vec[1].as_secs_f64() / elapsed_secs * 100.0, self.time_scoring_vec[1].as_secs_f64(), self.time_scoring_vec[5].as_nanos());
            println!("  - Threefold rep: {}% ({}s)", self.time_scoring_vec[2].as_secs_f64() / elapsed_secs * 100.0, self.time_scoring_vec[2].as_secs_f64());
            println!("  - Others: {}% ({}s)", self.time_scoring_vec[3].as_secs_f64() / elapsed_secs * 100.0, self.time_scoring_vec[3].as_secs_f64());
            println!("     * Pawn Structure: {}% ({}s)", self.time_scoring_vec[4].as_secs_f64() / elapsed_secs * 100.0, self.time_scoring_vec[4].as_secs_f64());
        }
    }

    // This is always the top depth
    // Modified version of minimax function that will return a vector of all the moves
    // that are possible at the top level, sorted by their score (best to worst)
    fn iddfs(&mut self, depth: u8, maximizing_player: bool, scored_moves: Option<Vec<TreeDecision>>) -> Vec<TreeDecision> {
        let mut alpha = -1000000;
        let mut beta = 1000000;

        self.start_time = Some(std::time::Instant::now());
        self.nodes_expanded = 0;

        self.nodes_expanded += 1;
        
        let mut best_decision = TreeDecision {
            best_move: None,
            score: 0,
        };

        let moves = match scored_moves {
            Some(m) => {
                m.iter().map(|x| x.best_move.unwrap()).collect()
            },
            None => {
                self.board.gen_legal_moves()
            }
        };

        let mut scored_moves: Vec<TreeDecision> = Vec::new();

        if maximizing_player {
            best_decision.score = -1000000;

            for m in moves {
                self.board.push(m).unwrap();
                let decision = self.minimax(depth - 1, depth, false, alpha, beta);
                self.board.pop().unwrap();

                scored_moves.push(TreeDecision { best_move: Some(m), score: decision.score });

                if decision.score > alpha {
                    alpha = best_decision.score;
                }

                if beta <= alpha {
                    break;
                }
            }
        } else {
            best_decision.score = 1000000;
            for m in moves {
                self.board.push(m).unwrap();
                let decision = self.minimax(depth - 1, depth, true, alpha, beta);
                self.board.pop().unwrap();

                scored_moves.push(TreeDecision { best_move: Some(m), score: decision.score });

                if decision.score < beta {
                    beta = best_decision.score;
                }

                if beta <= alpha {
                    break;
                }
            }
        }

        scored_moves.sort_by(|a, b| a.score.cmp(&b.score));

        // Should be descending order if we are maximizing, ascending if we are minimizing
        if maximizing_player {
            scored_moves.reverse();
        }

        /*println!("===================================");
        for m in scored_moves.iter() {
            println!("Move: {:?}, Score: {}", m.best_move.unwrap().get_move_string(), m.score);
        }*/
        
        /*scored_moves.sort_by(|a, b| {
            if a.score > b.score {
                std::cmp::Ordering::Less
            } else if a.score < b.score {
                std::cmp::Ordering::Greater
            } else {
                // There should always be a best_move and a piece should always be at the from square
                let from_piece_a = self.board.get_piece_at(&a.best_move.unwrap().from).unwrap();
                let from_piece_b = self.board.get_piece_at(&b.best_move.unwrap().from).unwrap();

                if from_piece_a > from_piece_b {
                    std::cmp::Ordering::Less
                } else if from_piece_a < from_piece_b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });*/

        /*println!("===================================");
        for m in scored_moves.iter() {
            println!("Move: {:?}, Score: {}", m.best_move.unwrap().get_move_string(), m.score);
        }*/

        return scored_moves;
    }

    fn minimax(&mut self, depth: u8, top_depth: u8, maximizing_player: bool, mut alpha: i32, mut beta: i32) -> TreeDecision {
        if depth == top_depth {
            self.start_time = Some(std::time::Instant::now());
            self.nodes_expanded = 0;
        }

        self.nodes_expanded += 1;
        let mut best_decision;
        
        if self.perf_test {
            let start = std::time::Instant::now();
            best_decision = TreeDecision {
                best_move: None,
                score: score_board(&mut self.board, top_depth as i32 - depth as i32, false, &mut Some(&mut self.time_scoring_vec)),
            };
            self.time_scoring += start.elapsed();
        } else {
            best_decision = TreeDecision {
                best_move: None,
                score: score_board(&mut self.board, top_depth as i32 - depth as i32, false, &mut None),
            };
        }

        if self.use_tt {
            let tt_entry = self.get_from_tt_table(depth, self.board.get_hash());

            match tt_entry {
                Some(entry) => {
                    // We want to favor checkmates that are closer to the top of the tree
                    if entry.score > 100000 {
                        return TreeDecision { best_move: None, score: entry.score - depth as i32 };
                    } else if entry.score < -100000 {
                        return TreeDecision { best_move: None, score: entry.score + depth as i32 };
                    } else {
                        return TreeDecision { best_move: None, score: entry.score };
                    }
                },
                None => {}
            }
        }

        if depth == 0 {
            return best_decision
        }

        let moves;
        if self.perf_test {
            let start = std::time::Instant::now();
            moves = self.board.gen_legal_moves();
            self.time_legal_moves += start.elapsed();
        } else {
            moves = self.board.gen_legal_moves();
        }
        if moves.len() == 0 {
            return best_decision
        }

        if maximizing_player {
            best_decision.score = -1000000;

            for m in moves {
                self.board.push(m).unwrap();
                let decision = self.minimax(depth - 1, top_depth, false, alpha, beta);
                self.board.pop().unwrap();

                if decision.score > best_decision.score {
                    best_decision = decision;
                    best_decision.best_move = Some(m);

                    if best_decision.score > alpha {
                        alpha = best_decision.score;
                    }
                }

                if beta <= alpha {
                    break;
                }
            }
        } else {
            best_decision.score = 1000000;
            for m in moves {
                self.board.push(m).unwrap();
                let decision = self.minimax(depth - 1, top_depth, true, alpha, beta);
                self.board.pop().unwrap();

                if decision.score < best_decision.score {
                    best_decision = decision;
                    best_decision.best_move = Some(m);

                    if best_decision.score < beta {
                        beta = best_decision.score;
                    }
                }

                if beta <= alpha {
                    break;
                }
            }
        }

        if self.use_tt {
            self.save_to_tt_table(depth, best_decision.score, self.board.get_hash());
        }

        return best_decision;
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_bad_position() {
        let mut ai = ChessAI::new();
        let bmove = ai.best_move_iddfs(1.0).unwrap();

        println!("Best move: {}", bmove.best_move.unwrap().get_move_string());
    }

    #[test]
    fn test_score() {
        let mut ai = ChessAI::new();
        ai.import_position("rnbqkbnr/pppppppp/8/8/4P3/2NPBNP1/PPPQ1PBP/R2R2K1 w kq - 0 1").unwrap();

        ai.print_internal_board();
        println!("");
        let score = score_board(&mut ai.board, 4, true, &mut None);
        println!("Score: {}", score);
    }
}
