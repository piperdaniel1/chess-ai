use crate::board;
use std::fmt::Error;

pub struct ChessAI {
    board: board::Board,
    my_color: bool,
    debug_mode: bool,
    start_time: Option<std::time::Instant>,
    nodes_expanded: u64,
}

/* 
 * This struct stores the state of the minimax algorithm at a given point.
 */
pub struct TreeDecision {
    pub best_move: Option<board::Move>,
    pub score: i32,
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

    return score;
}

impl ChessAI {
    pub fn new() -> ChessAI {
        ChessAI {
            board: board::Board::new(),
            my_color: board::WHITE,
            debug_mode: false,
            start_time: None,
            nodes_expanded: 0,
        }
    }

    pub fn new_with_color(color: bool) -> ChessAI {
        ChessAI {
            board: board::Board::new(),
            my_color: color,
            debug_mode: false,
            start_time: None,
            nodes_expanded: 0,
        }
    }

    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    pub fn disable_debug(&mut self) {
        self.debug_mode = false;
    }

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

    pub fn print_internal_board(&self) {
        self.board.print();
    }

    pub fn best_move(&mut self, depth: u8, time_limit_secs: u64) -> Result<TreeDecision, Error> {
        if self.board.turn() != self.my_color {
            return Err(Error);
        }

        Ok(self.minimax(depth, depth, self.my_color, -1000000, 1000000))
    }

    pub fn report_search_speed(&mut self) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            let elapsed_secs = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
            println!("{} nodes expanded in {} seconds", self.nodes_expanded, elapsed_secs);
            println!("{} nodes per second", self.nodes_expanded as f64 / elapsed_secs);
        }
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
                }
                if best_decision.score > alpha {
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
                let decision = self.minimax(depth - 1, top_depth, true, alpha, beta);
                self.board.pop().unwrap();

                if decision.score < best_decision.score {
                    best_decision = decision;
                    best_decision.best_move = Some(m);
                }
                if best_decision.score < beta {
                    beta = best_decision.score;
                }
                if beta <= alpha {
                    break;
                }
            }
        }

        return best_decision;
    }
}
