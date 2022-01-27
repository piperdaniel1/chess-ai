#ifndef MINIMAX_H
#define MINIMAX_H

#include "tt_table.h"
#include "evaluator.h"
#include "thc.h"
#include <iostream>
#include <chrono>

class Minimax {
    private:
    TT_Table tt_table;
    Evaluator eval;
    bool v2;
    std::uint64_t start_time;
    std::uint64_t max_time;
    bool cut_search_early;
    

    public:
    Minimax();
    thc::Move get_best_move(thc::ChessRules &cr, int depth);
    int maximize(thc::ChessRules &cr, int depth, int alpha, int beta);
    int minimize(thc::ChessRules &cr, int depth, int alpha, int beta);
    std::uint64_t get_time();
    int positions_evaluated;
};

#endif