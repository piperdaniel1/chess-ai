#include "evaluator.h"

Evaluator::Evaluator() {
    std::cout << "Initializing evaluator..." << std::endl;
}

Evaluator::~Evaluator() {
    std::cout << "Destroying evaluator..." << std::endl;
}

// 0 = no
// 1 = white checkmate
// 2 = black checkmate
// 3 = stalemate
int Evaluator::is_game_over(Board board, Move * move_list) {
    Move * curr_move = move_list;
    int num_moves = 0;

    if(curr_move != nullptr) {
        num_moves = 1;
    }
    
    Square king_pos = board.get_king_pos();
    bool check_status = board.is_king_in_check(king_pos.row, king_pos.col);

    if (num_moves == 0 && check_status) {
        if(board.turn) {
            return 2;
        } else {
            return 1;
        }
    } else if (num_moves == 0 && !check_status) {
        return 3;
    }

    return 0;
}

int Evaluator::is_game_overC(Board board, std::vector<MovC>& legal_moves) {
    Square king_pos = board.get_king_pos();
    bool check_status = board.is_king_in_check(king_pos.row, king_pos.col);

    if (legal_moves.size() == 0 && check_status) {
        if(board.turn) {
            return 2;
        } else {
            return 1;
        }
    } else if (legal_moves.size() == 0 && !check_status) {
        return 3;
    }

    return 0;
}

int Evaluator::evaluate(Board board, Move * legal_moves, bool verbose=false) {
    int game_over_status = this->is_game_over(board, legal_moves);
    if (game_over_status != 0) {
        if (game_over_status == 1) {
            return 100000;
        } else if (game_over_status == 2) {
            return -100000;
        } else if (game_over_status == 3) {
            return 0;
        }
    }

    int row = 0;
    int col = 0;

    int score = 0;

    for (row = 0; row < 8; row++) {
        for (col = 0; col < 8; col++) {
            char piece = board.get_piece(row, col);
            if (piece == 'p') {
                score -= 10;
            } else if (piece == 'r') {
                score -= 50;
            } else if (piece == 'n') {
                score -= 30;
            } else if (piece == 'b') {
                score -= 30;
            } else if (piece == 'q') {
                score -= 90;
            } else if (piece == 'P') {
                score += 10;
            } else if (piece == 'R') {
                score += 50;
            } else if (piece == 'N') {
                score += 30;
            } else if (piece == 'B') {
                score += 30;
            } else if (piece == 'Q') {
                score += 90;
            }
        }
    }

    if (verbose) {
        std::cout << "Score: " << score << std::endl;
    }
    return score;
}

int Evaluator::evaluateC(Board board, std::vector<MovC>& legal_moves, bool verbose) {
    int game_over_status = this->is_game_overC(board, legal_moves);
    if (game_over_status != 0) {
        if (game_over_status == 1) {
            return 100000;
        } else if (game_over_status == 2) {
            return -100000;
        } else if (game_over_status == 3) {
            return 0;
        }
    }

    int row = 0;
    int col = 0;

    int score = 0;

    for (row = 0; row < 8; row++) {
        for (col = 0; col < 8; col++) {
            char piece = board.get_piece(row, col);
            if (piece == 'p') {
                score -= 10;
            } else if (piece == 'r') {
                score -= 50;
            } else if (piece == 'n') {
                score -= 30;
            } else if (piece == 'b') {
                score -= 30;
            } else if (piece == 'q') {
                score -= 90;
            } else if (piece == 'P') {
                score += 10;
            } else if (piece == 'R') {
                score += 50;
            } else if (piece == 'N') {
                score += 30;
            } else if (piece == 'B') {
                score += 30;
            } else if (piece == 'Q') {
                score += 90;
            }
        }
    }

    if (verbose) {
        std::cout << "Score: " << score << std::endl;
    }
    return score;
}