// this file will contain the main function for the cpp-engine
#include <iostream>
#include "minimax.h"
#include "thc.h"
#include "fstream"
#include <experimental/filesystem>
#include <stdio.h>
#include <unistd.h>

// $ g++ engine.cpp minimax.cpp tt_table.cpp evaluator.cpp board.cpp perft_test.cpp
// ./a.out

int main() {
    thc::ChessRules cr;
    Minimax m;
    thc::Move bmove = m.get_best_move(cr, 3);
    std::cout << "best move: " << bmove.NaturalOut(&cr) << std::endl;
    return 0;
}