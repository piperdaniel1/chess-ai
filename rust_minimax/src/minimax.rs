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
#[derive(Debug)]
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

    pub fn best_move(&mut self, depth: u8) -> Result<TreeDecision, Error> {
        if self.board.turn() != self.my_color {
            return Err(Error);
        }

        let start_time = Some(std::time::Instant::now());

        // Iterative deepening
        let result = self.minimax(depth, depth, self.my_color, -1000000, 1000000);

        Ok(result)
    }

    pub fn best_move_iddfs(&mut self, time_allowed_secs: u64) -> Result<TreeDecision, Error> {
        if self.board.turn() != self.my_color {
            return Err(Error);
        }

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
            score_vec = Some(self.iddfs(i, self.my_color, score_vec));
            let end_time = Some(std::time::Instant::now()).unwrap();

            let elapsed = end_time.duration_since(start_time).as_millis();
            times.push(elapsed);

            let projected_ms = self.project_next_ms(&times);

            let next_end_time = end_time + std::time::Duration::from_millis(projected_ms as u64);

            if next_end_time.duration_since(initial_start_time).as_secs() > time_allowed_secs {
                println!("Breaking after depth {}", i);
                break;
            }
        }

        result.best_move = score_vec.as_ref().unwrap()[0].best_move;
        result.score = score_vec.as_ref().unwrap()[0].score;

        Ok(result)
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
