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
        std::vector<thc::Move> new_moves;
        std::vector<thc::Move> moves;
        std::vector<bool> check;
        std::vector<bool> mate;
        std::vector<bool> stalemate;
        cr.GenLegalMoveList(  moves, check, mate, stalemate );

        int idepth = 2;
        std::uint64_t stime = m.get_time();
        m.max_time = stime + 5000;
        m.cut_search_early = false;
        m.start_time = stime;
        new_moves = moves;
    
        while(true) {
            moves = new_moves;
            new_moves = m.get_best_move(cr, moves, idepth);
            idepth++;

            if(m.get_time() - stime > 5000) {
                break;
            }
        }

        myFile.open("output_file.txt");
        myFile << moves[0].TerseOut();
        myFile.close();
    }

    //thc::Move bmove = m.get_best_move(cr, 3);
    //std::cout << "best move: " << bmove.NaturalOut(&cr) << std::endl;
    return 0;
}