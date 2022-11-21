use minimax::score_board;

mod board;
mod minimax;

// Goal: Make board.legal_moves() a function that takes
// an immutable self reference while also not having it
// make a copy of itself.
//
// This is tricky because we have to statically analyze the
// board to determine if a given psuedo-legal move will be
// legal or not.
//
// Mainly, we worry about three main cases:
// 1. Normal pieces moving
//     - Here we are mainly just worried about the piece being
//       pinned to the king.
// 2. The king moving one square
//     - Here we are only worried about the king moving into
//       a square that is attacked by an enemy piece.
// 3. The king castling
//     - Here we are also worried about (2) but also must consider
//       the square between the king and where it is castling to.
//
// It seems like 2 and 3 could be solved by storing which squares
// are under attack by which pieces.

fn main() {
    //let board = board::Board::new_from_fen("k7/2Q5/8/1K6/8/8/8/8 b - - 0 1").unwrap();

    //println!("{}", score_board(&board, 0));

    let mut ai = minimax::ChessAI::new();
    //ai.import_position("k7/8/2q5/8/8/5Q2/4K3/8 w - - 0 1").unwrap();
    //ai.import_position("k7/2Q5/1K6/8/8/8/8/8 w - - 0 1").unwrap();
    ai.import_position("1k6/3Q4/2K5/8/8/8/8/8 w - - 0 1").unwrap();
    let res = ai.best_move(2);
    
    println!("{} with score {}", res.best_move.unwrap().get_move_string(), res.score);
}
