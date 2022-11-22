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

fn get_move_from_player(possible_moves: Vec<board::Move>) -> board::Move {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    let new_move = board::Move::new_from_string(&input);

    match new_move {
        Ok(m) => {
            if possible_moves.contains(&m) {
                m
            } else {
                println!("Invalid move!");
                get_move_from_player(possible_moves)
            }
        },
        Err(_) => {
            println!("Error, that's not valid input.");
            get_move_from_player(possible_moves)
        }
    }
}

fn play_against_ai(player_color: bool) {
    let mut ai = minimax::ChessAI::new_with_color(!player_color);

    let mut board = board::Board::new();

    while !board.checkmate() && !board.stalemate() {
        println!("DISPLAY BOARD =====================");
        board.print();

        let new_move;
        if board.turn() == player_color {
            println!("Your turn!");
            new_move = get_move_from_player(board.gen_legal_moves());
        } else {
            println!("AI's turn!");
            new_move = ai.best_move(4).best_move.unwrap();
            println!("AI chose: {}", new_move.get_move_string());
        }

        // We have to push the move to both the display board and the AI's internal board
        board.push(new_move).unwrap();
        ai.push_move(new_move).unwrap();
    }

    println!("Game over!");
    if board.checkmate() {
        if board.turn() == player_color {
            println!("You lost to checkmate!");
        } else {
            println!("You won with checkmate!");
        }
    } else {
        println!("Stalemate!");
    }

    println!("Final board: {:#?}", board);

}

fn main() {
    play_against_ai(board::WHITE);
}
