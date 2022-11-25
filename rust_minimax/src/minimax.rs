use crate::board;
use std::fmt::Error;

pub struct ChessAI {
    board: board::Board,
    my_color: bool,
    debug_mode: bool,
    start_time: Option<std::time::Instant>,
    nodes_expanded: u64,
    tt_table: [Option<PositionScore>; 10_000_000],
}

#[derive(Copy, Clone)]
struct PositionScore {
    score: i32,
    depth: u8,
    hash: u64,
}

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

fn normal_position_differential(board: &board::Board) -> i32 {
    let mut differential: i32 = 0;
    // White pawns
    let white_pawns = board.get_pawn_list(board::WHITE);
    for pawn in white_pawns {
        differential += WHITE_PAWN_MAP[pawn.row as usize][pawn.col as usize];
    }

    // Black pawns
    let black_pawns = board.get_pawn_list(board::BLACK);
    for pawn in black_pawns {
        differential += BLACK_PAWN_MAP[pawn.row as usize][pawn.col as usize];
    }

    // White knights
    let white_knights = board.get_knight_list(board::WHITE);
    for knight in white_knights {
        differential += KNIGHT_MAP[knight.row as usize][knight.col as usize];
    }

    // Black knights
    let black_knights = board.get_knight_list(board::BLACK);
    for knight in black_knights {
        differential += KNIGHT_MAP[knight.row as usize][knight.col as usize];
    }

    // White bishops
    let white_bishops = board.get_bishop_list(board::WHITE);
    for bishop in white_bishops {
        differential += WHITE_BISHOP_MAP[bishop.row as usize][bishop.col as usize];
    }

    // Black bishops
    let black_bishops = board.get_bishop_list(board::BLACK);
    for bishop in black_bishops {
        differential += BLACK_BISHOP_MAP[bishop.row as usize][bishop.col as usize];
    }

    // White rooks
    let white_rooks = board.get_rook_list(board::WHITE);
    for rook in white_rooks {
        differential += WHITE_ROOK_MAP[rook.row as usize][rook.col as usize];
    }

    // Black rooks
    let black_rooks = board.get_rook_list(board::BLACK);
    for rook in black_rooks {
        differential += BLACK_ROOK_MAP[rook.row as usize][rook.col as usize];
    }

    // White queens
    let white_queens = board.get_queen_list(board::WHITE);
    for queen in white_queens {
        differential += WHITE_QUEEN_MAP[queen.row as usize][queen.col as usize];
    }

    // Black queens
    let black_queens = board.get_queen_list(board::BLACK);
    for queen in black_queens {
        differential += BLACK_QUEEN_MAP[queen.row as usize][queen.col as usize];
    }

    // White king
    let white_king = board.get_king_square(board::WHITE);
    differential += WHITE_KING_MAP[white_king.row as usize][white_king.col as usize];

    // Black king
    let black_king = board.get_king_square(board::BLACK);
    differential += BLACK_KING_MAP[black_king.row as usize][black_king.col as usize];

    // Return the differential
    differential
}

// High scores are good for white, low scores are good for black
pub fn score_board(board: &board::Board, current_depth: i32) -> i32 {
    // Extremely basic scoring function
    if board.checkmate() {
        if board.turn() {
            return -1000000 + current_depth;
        } else {
            return 1000000 - current_depth;
        }
    }
    
    if board.stalemate() {
        return 0;
    }

    let mut score = 0;
    score += board.get_pawn_differential() * 100;
    score += board.get_bishop_differential() * 300;
    score += board.get_knight_differential() * 300;
    score += board.get_rook_differential() * 500;
    score += board.get_queen_differential() * 900;

    score += normal_position_differential(board) as i32;

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
            tt_table: [None; 10_000_000],
        }
    }

    pub fn new_with_color(color: bool) -> ChessAI {
        ChessAI {
            board: board::Board::new(),
            my_color: color,
            debug_mode: false,
            start_time: None,
            nodes_expanded: 0,
            tt_table: [None; 10_000_000],
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

    #[allow(dead_code)]
    pub fn best_move(&mut self, depth: u8) -> Result<TreeDecision, Error> {
        if self.board.turn() != self.my_color {
            return Err(Error);
        }

        // Iterative deepening
        let result = self.minimax(depth, depth, self.my_color, -1000000, 1000000);

        Ok(result)
    }

    pub fn best_move_iddfs(&mut self, time_allowed_secs: f64) -> Result<TreeDecision, Error> {
        //if self.board.turn() != self.my_color {
        //    return Err(Error);
        //}

        let initial_start_time = Some(std::time::Instant::now()).unwrap();

        // Iterative deepening
        let mut result = TreeDecision {
            best_move: None,
            score: 0,
        };


        let mut times = Vec::new();

        // max depth
        let depth = 10;
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
                println!("Breaking after depth {}", i);
                break;
            }
        }

        result.best_move = score_vec.as_ref().unwrap()[0].best_move;
        result.score = score_vec.as_ref().unwrap()[0].score;

        Ok(result)
    }

    // Saves the score to the transposition table using the zobrist hash
    fn save_to_tt_table(&mut self, depth: u8, score: i32, hash: u64) {
        let entry = PositionScore {
            hash,
            depth,
            score,
        };

        self.tt_table[hash as usize % self.tt_table.len()] = Some(entry);
    }

    // Returns the score from the transposition table using the zobrist hash if it exists
    fn get_from_tt_table(&mut self, curr_depth: u8, hash: u64) -> Option<PositionScore> {
        let entry = self.tt_table[hash as usize % self.tt_table.len()];

        if entry.is_none() {
            return None;
        }

        let entry = entry.unwrap();

        if entry.depth >= curr_depth {
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

    pub fn report_search_speed(&mut self) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            let elapsed_secs = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
            println!("{} nodes expanded in {} seconds", self.nodes_expanded, elapsed_secs);
            println!("{} nodes per second", self.nodes_expanded as f64 / elapsed_secs);
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

        return scored_moves;
    }

    fn minimax(&mut self, depth: u8, top_depth: u8, maximizing_player: bool, mut alpha: i32, mut beta: i32) -> TreeDecision {
        if depth == top_depth {
            self.start_time = Some(std::time::Instant::now());
            self.nodes_expanded = 0;
        }

        self.nodes_expanded += 1;
        
        let mut best_decision = TreeDecision {
            best_move: None,
            score: score_board(&self.board, top_depth as i32 - depth as i32),
        };

        if depth == 0 {
            return best_decision
        }

        let moves = self.board.gen_legal_moves();
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

        return best_decision;
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_bad_position() {
        let mut ai = ChessAI::new();
        ai.import_position("3k2n1/p5p1/8/1n6/4BP2/r4P1P/1NR5/4K3 b - - 0 43").unwrap();
        //ai.print_internal_board();
        let bmove = ai.best_move_iddfs(1.0).unwrap();

        println!("Best move: {}", bmove.best_move.unwrap().get_move_string());
    }
}