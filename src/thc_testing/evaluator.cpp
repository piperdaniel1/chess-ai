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
int Evaluator::is_game_over(thc::ChessRules cr) {
    int num_moves = 0;

    thc::TERMINAL eval_final_position;
    bool legal2 = cr.Evaluate( eval_final_position );
        
    if(eval_final_position == thc::TERMINAL_WCHECKMATE) {
        return 1;
    } else if(eval_final_position == thc::TERMINAL_BCHECKMATE) {
        return 2;
    } else if(eval_final_position == thc::TERMINAL_BSTALEMATE || eval_final_position == thc::TERMINAL_WSTALEMATE) {
        return 3;
    } else {
        return 0;
    }

    return 0;
}

int Evaluator::evaluate(thc::ChessRules cr, bool verbose) {
    int game_over_status = this->is_game_over(cr);
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

    for(int i=0; i< 64; i++) {
        char piece = cr.squares[i];
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

    if (verbose) {
        std::cout << "Score: " << score << std::endl;
    }

    return score;
}