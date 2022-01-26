// this file will contain the minimax algorithm
#include "minimax.h"

Minimax::Minimax() {
    std::cout << "Initializing minimax..." << std::endl;
    this->v2 = false;
    /*Board test;
    std::cout << "TT Table integrity test: " << std::endl;
    test.print_self();
    std::cout << "Hash: " << this->tt_table.get_hash(test.board) << std::endl;
    std::cout << "Hash2:\n" << this->tt_table.test_thing(test.board) << std::endl;

    this->tt_table.store_board(test.board, 5, 5);
    std::cout << "Stored board..." << std::endl;
    std::cout << "Querying for board:"  << std::endl;
    Entry test_entry = this->tt_table.query_board(test.board);
    std::cout << "Depth: " << test_entry.depth << std::endl;
    std::cout << "Eval: " << test_entry.eval << std::endl;
    std::cout << "Hash: " << test_entry.hash << std::endl;
    std::cout << "Test complete." << std::endl;*/
}

int Minimax::minimize(Board * board, int depth, int alpha, int beta, bool verbose=false) {
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

        Entry tt_entry = this->tt_table.query_board(next_board->board);
        if (tt_entry.depth >= depth) {
            score = tt_entry.eval;
        } else {
            score = this->maximize(next_board, depth - 1, alpha, beta, false);

            this->tt_table.store_board(next_board->board, depth, score);
        }

        if (score < best_score) {
            best_score = score;
        }

        if (best_score < beta) {
            beta = best_score;
        }

        if (alpha > beta) {
            break;
        }

        curr_move = curr_move->next;
    }

    board->free_move_list(move_list);
    delete board;
    return best_score;
}

int Minimax::maximize(Board * board, int depth, int alpha, int beta, bool verbose = false) {
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

        Entry tt_entry = this->tt_table.query_board(next_board->board);
        if (tt_entry.depth >= depth) {
            score = tt_entry.eval;
        } else {
            score = this->minimize(next_board, depth - 1, alpha, beta, false);

            this->tt_table.store_board(next_board->board, depth, score);
        }

        if (score > best_score) {
            best_score = score;
        }

        if (best_score > alpha) {
            alpha = best_score;
        }

        if (alpha > beta) {
            break;
        }

        curr_move = curr_move->next;
    }

    board->free_move_list(move_list);
    delete board;
    return best_score;
}

std::string* Minimax::get_best_move(Board board, int depth, int& num_moves, Move* sorted_legal_moves) {
    this->positions_evaluated = 0;
    int alpha = -100000;
    int beta = 100000;

    Move * move_list = sorted_legal_moves;
    Move * curr_move = move_list;

    int game_over = this->eval.is_game_over(board, move_list);
    if (game_over != 0) {
        this->positions_evaluated++;
        return nullptr;
    }

    int best_score = 0;

    if (board.turn) {
        best_score = -100000;
    } else {
        best_score = 100000;
    }
    Move * best_move = nullptr;
    int score = 0;
    int scores[100]; // i really don't think there will ever be more than 100 moves
    std::string * move_fens = new std::string[100];
    num_moves = 0;

    while (curr_move != nullptr) {
        Board * next_board = new Board(board);
        next_board->push_move(curr_move);

        Entry tt_entry = this->tt_table.query_board(next_board->board);
        if (tt_entry.depth >= depth) {
            score = tt_entry.eval;
        } else {
            if(!board.turn) {
                score = this->maximize(next_board, depth - 1, alpha, beta, false);
            } else {
                score = this->minimize(next_board, depth - 1, alpha, beta, false);
            }

            this->tt_table.store_board(next_board->board, depth, score);
        }

        scores[num_moves] = score;
        move_fens[num_moves] = board.get_move_fen(curr_move);
        num_moves++;

        // minimizing
        if(!board.turn) {
            if (best_score < beta) {
                beta = best_score;
            }
        // maximizing
        } else {
            if (best_score > alpha) {
                alpha = best_score;
            }
        }
        curr_move = curr_move->next;
    }

    board.free_move_list(move_list);
    
    if(board.turn) {
        // rank the moves by score highest to lowest
        int swaps = 0;
        while(1) {
            for(int i=0; i<num_moves-1; i++) {
                if(scores[i] < scores[i+1]) {
                    int temp = scores[i];
                    scores[i] = scores[i+1];
                    scores[i+1] = temp;

                    std::string temp_fen = move_fens[i];
                    move_fens[i] = move_fens[i+1];
                    move_fens[i+1] = temp_fen;

                    swaps++;
                }
            }
            if(swaps == 0) {
                break;
            }
            swaps = 0;
        }
    } else {
        // rank the moves by score lowest to highest
        int swaps = 0;
        while(1) {
            for(int i=0; i<num_moves-1; i++) {
                if(scores[i] > scores[i+1]) {
                    int temp = scores[i];
                    scores[i] = scores[i+1];
                    scores[i+1] = temp;

                    std::string temp_fen = move_fens[i];
                    move_fens[i] = move_fens[i+1];
                    move_fens[i+1] = temp_fen;

                    swaps++;
                }
            }
            if(swaps == 0) {
                break;
            }
            swaps = 0;
        }
    }

    return move_fens;
}