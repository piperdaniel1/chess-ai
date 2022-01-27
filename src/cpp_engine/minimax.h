#ifndef MINIMAX_H
#define MINIMAX_H

#include "tt_table.h"
#include "evaluator.h"
#include "board.h"
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
    std::string* get_best_move(Board board, int depth, int& num_moves, Move* sorted_legal_moves, std::uint64_t max);
    int maximize(Board * board, int depth, int alpha, int beta, bool verbose);
    int minimize(Board * board, int depth, int alpha, int beta, bool verbose);
    std::uint64_t get_time();
    int positions_evaluated;
};

#endif