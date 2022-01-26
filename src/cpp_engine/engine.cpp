// this file will contain the main function for the cpp-engine
#include <iostream>
#include "minimax.h"
#include "board.h"
#include "perft_test.h"
#include "fstream"
#include <experimental/filesystem>
#include <stdio.h>
#include <unistd.h>

// $ g++ engine.cpp minimax.cpp tt_table.cpp evaluator.cpp board.cpp perft_test.cpp
// ./a.out

int main() {
    Minimax minimax;
    Board board;
    Move * best_move;
    std::ifstream input_file;
    std::ofstream myFile;
    std::string fen;

    while(1) {

        input_file.open("input_file.txt");
        if (input_file.is_open()) {
            std::getline(input_file, fen);
            input_file.close();

            if(fen == "quit") {
                break;
            } else if (fen == "") {
                continue;
            }

            remove("input_file.txt");
        } else {
            continue;
        }

        board.import_board_fen(fen);

        best_move = minimax.get_best_move(board, 4);
        std::cout << "Best move: " << board.get_move_fen(best_move) << std::endl;
        std::cout << "Evaluated " << minimax.positions_evaluated << " positions." << std::endl;

        myFile.open("output_file.txt");
        myFile << board.get_move_fen(best_move);
        myFile.close();

        sleep(0.25);
    }

    return 0;
}