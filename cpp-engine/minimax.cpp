// this file will contain the minimax algorithm
#include "minimax.h"

Minimax::Minimax() {
    std::cout << "Initializing minimax..." << std::endl;
}

int Minimax::minimize(Board * board, int depth, int alpha, int beta) {
    Move * move_list = board->get_legal_moves();
    Move * curr_move = move_list;

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluate(*board, move_list, false);
        delete board;
        return final_eval;
    }

    int game_over = this->eval.is_game_over(*board, move_list);
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

    int best_score = 100000;
    int score = 0;

    while (curr_move != nullptr) {
        Board * next_board = new Board(*board);
        next_board->push_move(curr_move);
        score = this->maximize(next_board, depth - 1, alpha, beta);

        if (score < best_score) {
            best_score = score;
        }

        if (best_score < beta) {
            beta = best_score;
        }

        if (alpha >= beta) {
            break;
        }

        curr_move = curr_move->next;
    }

    board->free_move_list(move_list);

    return best_score;
}

int Minimax::maximize(Board * board, int depth, int alpha, int beta) {
    //std::cout << "MAXIMIZE: " << board.turn << std::endl;
    Move * move_list = board->get_legal_moves();
    Move * curr_move = move_list;

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluate(*board, move_list, false);
        delete board;
        return final_eval;
    }

    int game_over = this->eval.is_game_over(*board, move_list);
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

    int best_score = -100000;
    int score = 0;

    while (curr_move != nullptr) {
        Board * next_board = new Board(*board);
        next_board->push_move(curr_move);
        score = this->minimize(next_board, depth - 1, alpha, beta);

        if (score > best_score) {
            best_score = score;
        }

        if (best_score > alpha) {
            alpha = best_score;
        }

        if (beta <= alpha) {
            break;
        }

        curr_move = curr_move->next;
    }

    board->free_move_list(move_list);

    return best_score;
}

Move * Minimax::get_best_move(Board board, int depth) {
    this->positions_evaluated = 0;
    int alpha = -100000;
    int beta = 100000;

    Move * move_list = board.get_legal_moves();
    Move * curr_move = move_list;


    int game_over = this->eval.is_game_over(board, move_list);
    if (game_over != 0) {
        return nullptr;
    }

    int best_score = -100000;
    Move * best_move = nullptr;
    int score = 0;

    while (curr_move != nullptr) {
        Board * next_board = new Board(board);
        next_board->push_move(curr_move);

        score = this->maximize(next_board, depth - 1, alpha, beta);
        std::cout << "Score of move (" << curr_move->from_y << " " << curr_move->from_x << " - " << curr_move->to_y << " " << curr_move->to_x << "): " << score << std::endl;
        
        if (score > best_score) {
            best_score = score;
            best_move = curr_move;
        }

        if (best_score > alpha) {
            alpha = best_score;
        }

        if (beta <= alpha) {
            break;
        }

        curr_move = curr_move->next;
    }

    board.free_move_list(move_list);
    return best_move;
}