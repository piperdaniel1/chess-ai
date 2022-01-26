// this file will contain the Perft_Test algorithm
#include "perft_test.h"

Perft_Test::Perft_Test() {
    std::cout << "Initializing Perft_Test..." << std::endl;
}

int Perft_Test::minimize(Board * board, int depth, int alpha, int beta, bool verbose=false) {
    Move * move_list = board->get_legal_moves(true);
    Move * curr_move = move_list;

    int game_over = this->eval.is_game_over(*board, move_list);
    if (game_over != 0) {
        this->positions_evaluated++;
        if (game_over == 1) {
            board->free_move_list(move_list);
            delete board;
            return 100000;
        } else if (game_over == 2) {
            board->free_move_list(move_list);
            delete board;
            return -100000;
        } else if (game_over == 3) {
            board->free_move_list(move_list);
            delete board;
            return 0;
        }
    }

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluate(*board, move_list, false);
        board->free_move_list(move_list);
        delete board;
        return final_eval;
    }

    int best_score = 100000;
    int score = 0;
    while (curr_move != nullptr) {
        Board * next_board = new Board(*board);
        next_board->push_move(curr_move);

        score = this->maximize(next_board, depth - 1, alpha, beta, false);

        if (score < best_score) {
            best_score = score;
        }

        if (best_score < beta) {
            beta = best_score;
        }

        if (alpha >= beta) {
            //break;
        }

        curr_move = curr_move->next;
    }

    board->free_move_list(move_list);
    delete board;
    return best_score;
}

int Perft_Test::maximize(Board * board, int depth, int alpha, int beta, bool verbose = false) {
    Move * move_list = board->get_legal_moves();
    Move * curr_move = move_list;

    int game_over = this->eval.is_game_over(*board, move_list);
    if (game_over != 0) {
        this->positions_evaluated++;
        if (game_over == 1) {
            board->free_move_list(move_list);
            delete board;
            return 100000;
        } else if (game_over == 2) {
            board->free_move_list(move_list);
            delete board;
            return -100000;
        } else if (game_over == 3) {
            board->free_move_list(move_list);
            delete board;
            return 0;
        }
    }

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluate(*board, move_list, false);
        board->free_move_list(move_list);
        delete board;
        return final_eval;
    }

    int best_score = -100000;
    int score = 0;
    while (curr_move != nullptr) {
        Board * next_board = new Board(*board);

        next_board->push_move(curr_move);

        score = this->minimize(next_board, depth - 1, alpha, beta, false);

        if (score > best_score) {
            best_score = score;
        }

        if (best_score > alpha) {
            alpha = best_score;
        }

        if (beta <= alpha) {
            //break;
        }

        curr_move = curr_move->next;
    }

    board->free_move_list(move_list);
    delete board;
    return best_score;
}

Move * Perft_Test::get_best_move(Board board, int depth) {
    this->positions_evaluated = 0;
    int alpha = -100000;
    int beta = 100000;

    // if board->turn is true, then we are maximizing
    // if board->turn is false, then we are minimizing

    Move * move_list = board.get_legal_moves();
    Move * curr_move = move_list;


    int game_over = this->eval.is_game_over(board, move_list);
    if (game_over != 0) {
        this->positions_evaluated++;
        return nullptr;
    }

    int best_score = 0;

    if (!board.turn) {
        best_score = -100000;
    } else {
        best_score = 100000;
    }
    Move * best_move = nullptr;
    int score = 0;
    int last_eval = 0;

    while (curr_move != nullptr) {
        Board * next_board = new Board(board);
        next_board->push_move(curr_move);

        last_eval = this->positions_evaluated;
        
        if(!board.turn) {
            score = this->maximize(next_board, depth - 1, alpha, beta, false);
        } else {
            score = this->minimize(next_board, depth - 1, alpha, beta, false);
        }
        std::string move_fen = board.get_move_fen(curr_move);
        if(move_fen.length() == 4) {
            std::cout << board.get_move_fen(curr_move) << ":  " << this->positions_evaluated - last_eval << std::endl;
        } else {
            std::cout << board.get_move_fen(curr_move) << ": " << this->positions_evaluated - last_eval << std::endl;
        }

        if(!board.turn) {
            if (score > best_score) {
                best_score = score;
                best_move = curr_move;
            }
        } else {
            if (score < best_score) {
                best_score = score;
                best_move = curr_move;
            }
        }

        if(!board.turn) {
            if (best_score > alpha) {
                alpha = best_score;
            }
        } else {
            // this is BUGGED as SHIT bro.
            if (best_score < beta) {
                beta = best_score;
            }
        }
        curr_move = curr_move->next;
    }

    Move * unfreed_move = new Move;
    unfreed_move->from_x = best_move->from_x;
    unfreed_move->from_y = best_move->from_y;
    unfreed_move->to_y = best_move->to_y;
    unfreed_move->to_x = best_move->to_x;

    board.free_move_list(move_list);

    return unfreed_move;
}

void Perft_Test::run_perft_test() {
    int passes = 0;
    int fails = 0;
    Board test_board;
    test_board.import_board_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    std::cout << "Test 1: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "197281" << std::endl;
    if(this->positions_evaluated == 197281) {
        std::cout << "Test passed" << std::endl;
        passes++;
    } else {
        std::cout << "TEST FAILURE!!!!!!!!!!" << std::endl;
        fails++;
    }

    test_board.import_board_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0");
    std::cout << "Test 2: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "4085604" << std::endl;
    if(this->positions_evaluated == 4085604) {
        std::cout << "Test passed" << std::endl;
        passes++;
    } else {
        std::cout << "TEST FAILURE!!!!!!!!!!" << std::endl;
        fails++;
    }

    test_board.import_board_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0");
    std::cout << "Test 3: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "43238" << std::endl;
    if(this->positions_evaluated == 43238) {
        std::cout << "Test passed" << std::endl;
        passes++;
    } else {
        std::cout << "TEST FAILURE!!!!!!!!!!" << std::endl;
        fails++;
    }

    test_board.import_board_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    std::cout << "Test 4: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "422355" << std::endl;
    if(this->positions_evaluated == 422355) {
        std::cout << "Test passed" << std::endl;
        passes++;
    } else {
        std::cout << "TEST FAILURE!!!!!!!!!!" << std::endl;
        fails++;
    }

    test_board.import_board_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    std::cout << "Test 5: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "2103531" << std::endl;
    if(this->positions_evaluated == 2103531) {
        std::cout << "Test passed" << std::endl;
        passes++;
    } else {
        std::cout << "TEST FAILURE!!!!!!!!!!" << std::endl;
        fails++;
    }

    test_board.import_board_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    std::cout << "Test 6: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "3894594" << std::endl;
    if(this->positions_evaluated == 3894594) {
        std::cout << "Test passed" << std::endl;
        passes++;
    } else {
        std::cout << "TEST FAILURE!!!!!!!!!!" << std::endl;
        fails++;
    }

    std::cout << "\nTests complete. " << passes << " passed, " << fails << " failed." << std::endl;
}