#ifndef MINIMAX_H
#define MINIMAX_H

#include "tt_table.h"
#include "evaluator.h"
#include <iostream>

class Minimax {
    private:
    TT_Table tt_table;
    Evaluator eval;

    public:
    Minimax();
};

#endif