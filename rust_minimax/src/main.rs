use std::net::TcpListener;
use std::io::{Read, Write};

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
            new_move = ai.best_move(4, 5).unwrap().best_move.unwrap();
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

fn start_tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:4321").unwrap();
    let mut ai: Option<minimax::ChessAI> = None;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();

                let req_string = String::from_utf8_lossy(&buffer[..]);

                // compare the request string to 'ping'
                let req_string = req_string.trim_end_matches(char::from(0));

                println!("Request from {}: {}", stream.peer_addr().unwrap(), req_string);

                if req_string.starts_with("query") {
                    let time_limit = req_string.split_whitespace().nth(1);

                    // We default to a time limit of 5 seconds if the client doesn't specify
                    // or if the client specifies a time limit that fails to parse
                    let time_limit = match time_limit {
                        Some(t) => t.parse::<u64>(),
                        None => Ok(5)
                    };
                    let time_limit = match time_limit {
                        Ok(t) => t,
                        Err(_) => 5
                    };

                    match ai {
                        Some(ref mut ai) => {
                            let best_move = ai.best_move(5, time_limit);
                            let best_move = match best_move {
                                Ok(d) => d,
                                Err(_) => {
                                    println!("Responding with: 403 error-in-eval");
                                    stream.write("403 error-in-eval".as_bytes()).unwrap();
                                    continue;
                                }
                            };

                            let best_move = match best_move.best_move {
                                Some(m) => m,
                                None => {
                                    println!("Responding with: 403 error-in-internal-parse");
                                    stream.write("403 error-in-internal-parse".as_bytes()).unwrap();
                                    continue;
                                }
                            };

                            let response = format!("bestmove {}", best_move.get_move_string());
                            println!("Responding with: {}", response);
                            stream.write(response.as_bytes()).unwrap();
                        },
                        None => {
                            println!("Responding with: 403 err-not-init");
                            stream.write("403 err-not-init".as_bytes()).unwrap();
                        }
                    }
                } else if req_string.eq("init w") {
                    ai = Some(minimax::ChessAI::new_with_color(board::WHITE));

                    println!("Responding with 200 ok");
                    stream.write("200 ok".as_bytes()).unwrap();
                } else if req_string.eq("init b") {
                    ai = Some(minimax::ChessAI::new_with_color(board::BLACK));

                    println!("Responding with 200 ok");
                    stream.write("200 ok".as_bytes()).unwrap();
                } else if req_string.starts_with("push") {
                    match ai {
                        Some(ref mut ai) => {
                            let new_move = board::Move::new_from_string(&req_string[5..]);

                            let new_move = match new_move {
                                Ok(m) => m,
                                Err(_) => {
                                    println!("Responding with: 400 error-invalid-move");
                                    stream.write("400 err-invalid-move".as_bytes()).unwrap();
                                    continue;
                                }
                            };

                            ai.push_move(new_move).unwrap();
                            println!("Responding with 200 ok");
                            stream.write("200 ok".as_bytes()).unwrap();
                        },
                        None => {
                            println!("Responding with: 403 err-not-init");
                            stream.write("403 err-not-init".as_bytes()).unwrap();
                        }
                    }
                } else if req_string.eq("ping") {
                    println!("Responding with: pong");
                    stream.write("pong".as_bytes()).unwrap();
                } else {
                    println!("Responding with: 400 err-invalid-request");
                    stream.write("400 err-invalid-request".as_bytes()).unwrap();
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
fn main() {
    start_tcp_server()
}
