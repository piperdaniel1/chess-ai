use crate::board;
use std::fmt::Error;

pub struct ChessAI {
    board: board::Board,
    my_color: bool,
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
        }
    }

    pub fn import_position(&mut self, fen: &str) -> Result<(), Error> {
        match self.board.import_from_fen(fen) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn best_move(&mut self, depth: u8) -> TreeDecision {
        if self.board.turn() != self.my_color {
            panic!("It is not my turn!");
        }

        self.minimax(depth, depth, self.my_color, -1000000, 1000000)
    }

    fn minimax(&mut self, depth: u8, top_depth: u8, maximizing_player: bool, mut alpha: i32, mut beta: i32) -> TreeDecision {
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