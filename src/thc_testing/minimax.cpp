// this file will contain the minimax algorithm
#include "minimax.h"

Minimax::Minimax() {
    std::cout << "Initializing minimax..." << std::endl;
}

std::uint64_t Minimax::get_time() {
    return std::chrono::duration_cast<std::chrono::milliseconds>(
    std::chrono::system_clock::now().time_since_epoch()).count();
}

int Minimax::minimize(thc::ChessRules &cr, int depth, int alpha, int beta) {
    if (depth == 0) {
        return eval.evaluate(cr, false);
    }

    int best_score = 100000;
    thc::Move mv;
    std::vector<thc::Move> moves;
    std::vector<bool> check;
    std::vector<bool> mate;
    std::vector<bool> stalemate;
    cr.GenLegalMoveList(  moves, check, mate, stalemate );

    for (int i = 0; i < moves.size(); i++) {
        mv = moves[i];
        cr.PlayMove(mv);
        int score = maximize(cr, depth-1, alpha, beta);
        cr.PopMove(mv);
        if (score < best_score) {
            best_score = score;
        }

        beta = std::min(beta, best_score);

        if (alpha > beta) {
            break;
        }
    }
    return best_score;
}

int Minimax::maximize(thc::ChessRules &cr, int depth, int alpha, int beta) {
    if (depth == 0) {
        return eval.evaluate(cr, false);
    }
    thc::Move mv;
    std::vector<thc::Move> moves;
    std::vector<bool> check;
    std::vector<bool> mate;
    std::vector<bool> stalemate;
    cr.GenLegalMoveList(  moves, check, mate, stalemate );

    int best_score = -999999;

    for (unsigned int i=0; i<moves.size(); i++) {
        mv = moves[i];
        cr.PlayMove(mv);
        int score = minimize(cr, depth-1, alpha, beta);
        cr.PopMove(mv);
        if (score > best_score) {
            best_score = score;
        }

        alpha = std::max(alpha, best_score);

        if (alpha > beta) {
            break;
        }
    }

    return best_score;
}

thc::Move Minimax::get_best_move(thc::ChessRules &cr, int depth) {
    bool turn = cr.WhiteToPlay();

    thc::Move mv;
    std::vector<thc::Move> moves;
    std::vector<bool> check;
    std::vector<bool> mate;
    std::vector<bool> stalemate;
    int alpha = -999999;
    int beta = 999999;

    cr.GenLegalMoveList(  moves, check, mate, stalemate );
    int best_score;
    if(turn) {
        best_score = -999999;
    } else {
        best_score = 999999;
    }
    int score;
    thc::Move best_move;

    for( unsigned int i=0; i<moves.size(); i++ ) {
        mv = moves[i];
        cr.PlayMove(mv);
        if(turn) {
            score = minimize(cr, depth-1, -999999, 999999);
        } else {
            score = maximize(cr, depth-1, -999999, 999999);
        }
        cr.PopMove(mv);

        if(turn) {
            if(score > best_score) {
                best_score = score;
                best_move = mv;
            }

            alpha = std::max(alpha, best_score);
        } else {
            if(score < best_score) {
                best_score = score;
                best_move = mv;
            }

            beta = std::min(beta, best_score);
        }
    }

    return best_move;
}