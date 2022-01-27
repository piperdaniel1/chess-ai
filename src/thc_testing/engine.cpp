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
    thc::Move mv;
    Minimax m;
    std::ifstream input_file;
    std::ofstream myFile;
    std::string fen;

    while(1) {
        sleep(0.25);

        // Try to open the input file
        input_file.open("input_file.txt");

        // If we opened the file, check it for a valid fen
        if (input_file.is_open()) {
            std::getline(input_file, fen);
            input_file.close();

            // If fen is "quit", exit the program. If fen is empty, wait.
            if(fen == "quit") {
                break;
            } else if (fen == "") {
                continue;
            }

            // If fen was valid remove the file so we do not process it twice.
            remove("input_file.txt");
        } else {
            // if we did not open the file, wait for a new fen
            continue;
        }

        // If we got here, we have a valid fen.
        cr.Forsyth(fen.c_str());

        // Get the best move
        mv = m.get_best_move(cr, 4);
        myFile.open("output_file.txt");
        myFile << mv.TerseOut();
        myFile.close();
    }

    //thc::Move bmove = m.get_best_move(cr, 3);
    //std::cout << "best move: " << bmove.NaturalOut(&cr) << std::endl;
    return 0;
}