#ifndef MINIMAX_H
#define MINIMAX_H

#include "tt_table.h"
#include "evaluator.h"
#include "board.h"
#include <iostream>

class Minimax {
    private:
    TT_Table tt_table;
    Evaluator eval;
    bool v2;

    public:
    Minimax();
    Move * get_best_move(Board board, int depth);
    int maximize(Board * board, int depth, int alpha, int beta, bool verbose);
    int minimize(Board * board, int depth, int alpha, int beta, bool verbose);
    int positions_evaluated;
};

#endif