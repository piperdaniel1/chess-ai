#ifndef EVALUATOR_H
#define EVALUATOR_H

#include <iostream>
#include "thc.h"

class Evaluator {
    private:

    public:
    Evaluator();
    ~Evaluator();
    int evaluate(thc::ChessRules cr, bool verbose);
    int is_game_over(thc::ChessRules cr);
};

#endif