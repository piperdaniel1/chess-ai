// Local Networking stuff
use std::net::TcpListener;
use std::io::{Read, Write};

// Lichess stuff
use std::fs::File;
use futures_util::StreamExt;

mod board;
mod minimax;
mod lichess_api;

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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn play_against_ai(player_color: bool) {
    let mut ai = minimax::ChessAI::new_with_color(!player_color);
    // ai.import_position("8/4q3/3k4/8/8/5K2/8/8 w - - 0 1").unwrap();

    let mut board = board::Board::new();
    // board.import_from_fen("8/4q3/3k4/8/8/5K2/8/8 w - - 0 1").unwrap();

    while !board.checkmate() && !board.stalemate() {
        println!("DISPLAY BOARD =====================");
        board.print();

        let new_move;
        if board.turn() == player_color {
            println!("Your turn!");
            new_move = get_move_from_player(board.gen_legal_moves());
        } else {
            println!("AI's turn!");
            new_move = ai.best_move_iddfs(2.0).unwrap().best_move.unwrap();
            ai.report_search_speed();
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

#[allow(dead_code)]
fn start_tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:4321").unwrap();
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
                        Some(t) => t.parse::<f64>(),
                        None => Ok(0.75)
                    };
                    let time_limit = match time_limit {
                        Ok(t) => t,
                        Err(_) => 0.75
                    };

                    match ai {
                        Some(ref mut ai) => {
                            let best_move = ai.best_move_iddfs(time_limit);
                            let best_move = match best_move {
                                Ok(d) => d,
                                Err(_) => {
                                    println!("Responding with: 403 error-in-eval");
                                    stream.write("403 error-in-eval".as_bytes()).unwrap();
                                    continue;
                                }
                            };

                            let eval = best_move.score;
                            let best_move = match best_move.best_move {
                                Some(m) => m,
                                None => {
                                    println!("Responding with: 403 error-in-internal-parse");
                                    stream.write("403 error-in-internal-parse".as_bytes()).unwrap();
                                    continue;
                                }
                            };

                            ai.report_search_speed();
                            let response = format!("bestmove {} {}", best_move.get_move_string(), eval);
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
                } else if req_string.starts_with("init b fen ") {
                    ai = Some(minimax::ChessAI::new_with_color(board::BLACK));

                    let fen = req_string.split_at(11).1;

                    ai.as_mut().unwrap().import_position(fen).unwrap();

                    println!("Responding with 200 ok");
                    stream.write("200 ok".as_bytes()).unwrap();
                } else if req_string.starts_with("init w fen ") {
                    ai = Some(minimax::ChessAI::new_with_color(board::WHITE));

                    let fen = req_string.split_at(11).1;

                    ai.as_mut().unwrap().import_position(fen).unwrap();

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

fn get_lichess_token() -> String {
    let mut file = File::open("/home/daniel/personal-projects/chess-ai/rust_minimax/src/.lichess-token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).unwrap();
    token.trim().to_string()
}

async fn play_on_lichess() {
    let api = lichess_api::Lichess::new(get_lichess_token());

    // Main event loop
    // Structure:
    // First half gets a Game struct that contains the game ID we want
    // to play in
    // Second half (the game loop) plays this game until it is complete
    //
    // The issue with this design is that it will only allow us to play one
    // person at a time. This can be fixed in the future by refactoring the event loop
    // to get rid of the blocking functions and storing a list of games that we are in.
    // However, we will not do this yet because it is a bit more complicated
    loop {
        // Attempt to get a game we can play in
        let curr_game = api.get_current_game().await;

        // If there was no game, wait for someone to challenge us
        if curr_game.is_none() {
            // Wait for someone to challenge us
            let new_challenge = api.block_until_challenge().await;
            let name = new_challenge.challenger.name.clone();

            match api.accept_challenge(new_challenge).await {
                Ok(_) => {
                    println!("Accepted challenge from {}", name);
                },
                Err(e) => {
                    println!("Error accepting challenge: {}", e);
                }
            }
        }

        // Get the game struct
        let game= api.block_until_game_start().await;

        println!("Entering game against {}", game.opponent.username);

        let bot_color = match game.color.as_str() {
            "white" => board::WHITE,
            "black" => board::BLACK,
            _ => panic!("Invalid color")
        };
        let mut ai = minimax::ChessAI::new_with_color(bot_color);

        let mut stream = api.get_game_stream(&game).await;

        // Play the game (game loop)
        // Handle stream events
        while let Some(event) = stream.next().await {
            let event = match event {
                Ok(e) => e,
                Err(_) => continue,
            };
            let event = match api.parse_game_event(event) {
                Ok(e) => e,
                Err(_) => continue,
            };

            let state = match event {
                lichess_api::GameEvent::GameFull(game) => game.state,
                lichess_api::GameEvent::GameState(state) => state,
            };

            // Print the state
            println!("State: {:#?}", state);

            ai.reset_internal_board();

            if state.moves.len() > 0 {
                let move_vec = state.moves.split(" ").collect::<Vec<&str>>();

                // Push all moves to the AI (this might fail if moves are sent twice)
                for m in move_vec {
                    let m = board::Move::new_from_string(m).unwrap();
                    ai.push_move(m).unwrap();
                }
            }

            // If it is our turn, make a move
            if ai.get_board_turn() == bot_color {
                let time_rem: u32;
                if bot_color == board::WHITE {
                    time_rem = state.wtime + 5*state.winc;
                } else {
                    time_rem = state.btime + 5*state.binc;
                }

                let move_time = (time_rem as f64 / 30.0) / 1000.0;
                let move_time = std::cmp::min_by(move_time, 10.0, |a, b| a.partial_cmp(b).unwrap());
                let best_move = ai.best_move_iddfs(move_time as f64).unwrap().best_move.unwrap();
                println!("Best move: {}", best_move.get_move_string());

                api.make_move(&game, &best_move.get_move_string()).await.unwrap();
                println!("Made move");
            }
        }
    }
}

async fn challenge_lichess(username: &str) {
    let api = lichess_api::Lichess::new(get_lichess_token());
    api.create_challenge(&username).await;
}

fn test_ai() {
    let board = board::Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0").unwrap();

    let mut ai = minimax::ChessAI::new();
    ai.import_position(&board.fen()).unwrap();
    ai.enable_perf_test();

    let best_move = ai.best_move_iddfs(4.0).unwrap();

    let (best_move, eval) = (best_move.best_move.unwrap().get_move_string(), best_move.score);

    println!("Best move: {}", best_move);
    println!("Eval: {}", eval);
    ai.report_search_speed();
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("Usage: {} [server|client|lichess|test]", args[0]);
        return;
    }

    match args[1].as_str() {
        "server" => {
            start_tcp_server();
        },
        "client" => {
            play_against_ai(board::WHITE);
        },
        "lichess" => {
            play_on_lichess().await;
        },
        "test" => {
            test_ai();
        },
        "challenge" => {
            // let api = lichess_api::Lichess::new(get_lichess_token());
            // api.get_bot_to_challenge().await;

            if args.len() != 3 {
                println!("Usage: {} challenge <username>", args[0]);
                return;
            }

            challenge_lichess(&args[2]).await;
        },
        _ => {
            println!("Usage: {} [server|client|lichess]", args[0]);
        }
    }
}
