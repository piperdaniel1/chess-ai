#ifndef EVALUATOR_H
#define EVALUATOR_H

#include <iostream>
#include "board.h"

class Evaluator {
    private:

    public:
    Evaluator();
    ~Evaluator();
    int evaluate(Board board, bool verbose=false);
    int is_game_over(Board board);
};

#endif