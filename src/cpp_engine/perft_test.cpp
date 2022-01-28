// this file will contain the Perft_Test algorithm
#include "perft_test.h"

Perft_Test::Perft_Test() {
    std::cout << "Initializing Perft_Test..." << std::endl;
}

int Perft_Test::minimize(Board * board, int depth, int alpha, int beta, bool verbose=false) {
    std::vector<MovC> movesC;
    board->get_legal_movC(movesC);

    int game_over = this->eval.is_game_overC(*board, movesC);
    if (game_over != 0) {
        this->positions_evaluated++;
        if (game_over == 1) {
            delete board;
            return 100000;
        } else if (game_over == 2) {
            delete board;
            return -100000;
        } else if (game_over == 3) {
            delete board;
            return 0;
        }
    }

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluateC(*board, movesC, false);
        delete board;
        return final_eval;
    }

    int best_score = 100000;
    int score = 0;
    for(MovC mov : movesC) {
        Board * next_board = new Board(*board);
        next_board->push_movC(mov);

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
    }

    delete board;
    return best_score;
}

int Perft_Test::maximize(Board * board, int depth, int alpha, int beta, bool verbose = false) {
    std::vector<MovC> movesC;
    board->get_legal_movC(movesC);

    int game_over = this->eval.is_game_overC(*board, movesC);
    if (game_over != 0) {
        this->positions_evaluated++;
        if (game_over == 1) {
            delete board;
            return 100000;
        } else if (game_over == 2) {
            delete board;
            return -100000;
        } else if (game_over == 3) {
            delete board;
            return 0;
        }
    }

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluateC(*board, movesC, false);
        delete board;
        return final_eval;
    }

    int best_score = -100000;
    int score = 0;
    for(MovC mov : movesC) {
        Board * next_board = new Board(*board);

        next_board->push_movC(mov);

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
    }

    delete board;
    return best_score;
}

void Perft_Test::get_best_move(Board board, int depth) {
    this->positions_evaluated = 0;
    int alpha = -100000;
    int beta = 100000;

    // if board->turn is true, then we are maximizing
    // if board->turn is false, then we are minimizing

    std::vector<MovC> movesC;
    board.get_legal_movC(movesC);

    int game_over = this->eval.is_game_overC(board, movesC);
    if (game_over != 0) {
        this->positions_evaluated++;
    }

    int best_score = 0;

    if (!board.turn) {
        best_score = -100000;
    } else {
        best_score = 100000;
    }

    int score = 0;
    int last_eval = 0;

    for (MovC mov : movesC) {
        Board * next_board = new Board(board);
        next_board->push_movC(mov);

        last_eval = this->positions_evaluated;
        
        if(!board.turn) {
            score = this->maximize(next_board, depth - 1, alpha, beta, false);
        } else {
            score = this->minimize(next_board, depth - 1, alpha, beta, false);
        }
        
        std::string move_fen = mov.get_fen();
        if(move_fen.length() == 4) {
            std::cout << move_fen << ":  " << this->positions_evaluated - last_eval << std::endl;
        } else {
            std::cout << move_fen << ": " << this->positions_evaluated - last_eval << std::endl;
        }
    }
}

void Perft_Test::run_perft_test() {
    int passes = 0;
    int fails = 0;
    Board test_board;

    test_board.import_board_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    std::cout << "Test 1: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "422355" << std::endl;
    if(this->positions_evaluated == 422355) {
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

    test_board.import_board_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    std::cout << "Test 4: " << std::endl;
    this->get_best_move(test_board, 4);
    std::cout << "Test finished, results (actual vs expected): " << this->positions_evaluated << " vs " << "197281" << std::endl;
    if(this->positions_evaluated == 197281) {
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