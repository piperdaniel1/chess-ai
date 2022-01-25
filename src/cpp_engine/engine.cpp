// this file will contain the main function for the cpp-engine
#include <iostream>
#include "minimax.h"
#include "board.h"
#include "perft_test.h"
#include "fstream"
#include <experimental/filesystem>

// $ g++ engine.cpp minimax.cpp tt_table.cpp evaluator.cpp board.cpp perft_test.cpp
// ./a.out

int main(int argc, char * argv[]) {
    std::string fen;

    if(argc == 1) {
        std::cout << "Usage: ./a.out '<fen_string>'" << std::endl;
        std::cout << "Using default board as FEN string" << std::endl;
        fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    } else {
        fen = argv[1];
    }
    
    std::cout << "Initializing chess engine..." << std::endl;
    Minimax minimax;
    Board board;

    board.import_board_fen(fen);

    Move * best_move = minimax.get_best_move(board, 4);
    std::cout << "Best move: " << best_move->from_y << " " << best_move->from_x << "  " << best_move->to_y << " " << best_move->to_x << std::endl;
    std::cout << "Evaluated " << minimax.positions_evaluated << " positions." << std::endl;

    std::ofstream myFile;
    myFile.open("output_file.txt");
    myFile << board.get_move_fen(best_move);
    myFile.close();

    return 0;
}