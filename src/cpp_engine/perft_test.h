#ifndef PERFT_TEST_H
#define PERFT_TEST_H

#include "evaluator.h"
#include "board.h"
#include <iostream>

class Perft_Test {
    private:
    Evaluator eval = Evaluator();
    public:
    Perft_Test();
    void run_perft_test();
    void get_best_move(Board board, int depth);
    int maximize(Board * board, int depth, int alpha, int beta, bool verbose);
    int minimize(Board * board, int depth, int alpha, int beta, bool verbose);
    int positions_evaluated;
};

#endif