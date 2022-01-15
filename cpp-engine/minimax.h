#ifndef MINIMAX_H
#define MINIMAX_H

#include "tt_table.h"
#include "b_score.h"
#include <iostream>

class Minimax {
    private:
    TT_Table tt_table;
    Board_Score eval;

    public:
    Minimax();
};

#endif