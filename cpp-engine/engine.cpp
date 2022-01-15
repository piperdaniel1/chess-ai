// this file will contain the main function for the cpp-engine
#include <iostream>
#include "minimax.h"
#include "board.h"

// $ g++ engine.cpp minimax.cpp tt_table.cpp b_score.cpp board.cpp
// ./a.out

/*
 * TODO:
 * Main functionality:
 *  Loop that periodically checks for new input file.
 *  If new input is found give the appropiate FEN to the board and run the minimax.
 *  Delete the input file.
 *  Output the Minimax's chosen move to an output file.
 *  Restart process.
 */
int main() {
    std::cout << "Initializing chess engine..." << std::endl;
    Minimax minimax;
    Board board;

    return 0;
}