#ifndef EVALUATOR_H
#define EVALUATOR_H

#include <iostream>
#include "board.h"

class Evaluator {
    private:

    public:
    Evaluator();
    ~Evaluator();
    int evaluate(Board board, Move * legal_moves, bool verbose);
    int evaluateC(Board board, std::vector<MovC>& legal_moves, bool verbose);
    int is_game_over(Board board, Move * legal_moves);
    int is_game_overC(Board board, std::vector<MovC>& legal_moves);
};

#endif