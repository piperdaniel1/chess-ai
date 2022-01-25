// this file will contain the main function for the cpp-engine
#include <iostream>
#include "minimax.h"
#include "board.h"
#include "perft_test.h"
#include "fstream"

// $ g++ engine.cpp minimax.cpp tt_table.cpp evaluator.cpp board.cpp perft_test.cpp
// ./a.out

int main(int argc, char * argv[]) {
    if(argc == 1) {
        std::cout << "Usage: ./a.out '<fen_string>'" << std::endl;
        return 0;
    }
    
    std::cout << "Initializing chess engine..." << std::endl;
    Minimax minimax;
    Board board;

    board.import_board_fen(argv[1]);

    Move * best_move = minimax.get_best_move(board, 4);
    std::cout << "Best move: " << best_move->from_y << " " << best_move->from_x << "  " << best_move->to_y << " " << best_move->to_x << std::endl;
    std::cout << "Evaluated " << minimax.positions_evaluated << " positions." << std::endl;

    std::ofstream myFile;
    myFile.open("output_file.txt");
    myFile << board.get_move_fen(best_move);
    myFile.close();

    return 0;
}