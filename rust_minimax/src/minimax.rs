use crate::board;
use std::fmt::Error;

pub struct ChessAI {
    board: board::Board,
    my_color: bool,
}

/* 
 * This struct stores the state of the minimax algorithm at a given point.
 */
struct TreeDecision {
    best_move: Option<board::Move>,
    score: i32,
}

// High scores are good for white, low scores are good for black
fn score_board(board: &board::Board) -> i32 {
    return 0;
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

    pub fn best_move(&self) -> board::Move {
        if self.board.turn() != self.my_color {
            panic!("It is not my turn!");
        }

        return board::Move::new_from_string("a2a4").unwrap()
    }

    fn minimax(&mut self, depth: u8, top_depth: u8, maximizing_player: bool, mut alpha: i32, mut beta: i32) -> TreeDecision {
        if depth == 0 {
            return TreeDecision {
                best_move: None,
                score: score_board(&self.board),
            };
        }

        let mut best_decision = TreeDecision {
            best_move: None,
            score: 0,
        };

        let moves = self.board.gen_legal_moves();
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