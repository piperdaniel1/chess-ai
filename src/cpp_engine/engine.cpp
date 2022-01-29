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

Move * arr_to_linked_list(Board& board, std::vector<MovC> arr, int size) {
    Move * head = new Move();
    Move * curr = head;
    Move * converted_move;
    for (int i = 0; i < size; i++) {
        Move converted_move = arr[i].get_old_move();
        curr->from_x = converted_move.from_x;
        curr->from_y = converted_move.from_y;
        curr->to_x = converted_move.to_x;
        curr->to_y = converted_move.to_y;
        curr->promotion = converted_move.promotion;

        if (i != size - 1) {
            curr->next = new Move();
            curr = curr->next;
        }
    }

    curr->next = nullptr;
    return head;
}

int main() {
    //Perft_Test test;
    //test.run_perft_test();
    //return 0;
    Minimax minimax;
    Board board;
    std::ifstream input_file;
    std::ofstream myFile;
    std::string fen;
    std::vector<MovC> vetted_moves;
    std::vector<MovC> moves;
    int scores[100];
    int num_moves = 0;

    // Loop until quit command
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

        // import the fen into the board
        board.import_board_fen(fen);
        const int INITIAL_DEPTH = 2;
        int curr_depth = INITIAL_DEPTH;
        std::vector<MovC> sorted_moves;
        board.get_legal_movC(sorted_moves);
        std::uint64_t start = minimax.get_time();
        std::uint64_t max = start + 5000;
        
        minimax.score_of_best_move = 100000;

        // get the best move
        while(1) {
            minimax.get_best_move(board, curr_depth, num_moves, sorted_moves, max - minimax.get_time());

            if(minimax.get_time() < max) {
                vetted_moves = sorted_moves;
            } else {
                std::cout << "Cutting depth " << curr_depth << " short due to time constraint." << std::endl;
                break;
            }

            curr_depth++;
            std::cout << "Depth " << curr_depth-1 << " complete, " << minimax.positions_evaluated << " positions evaluated." << std::endl;
            std::cout << "Beginning depth " << curr_depth << "..." << std::endl;
            
            if(curr_depth > 25 || minimax.score_of_best_move < -1000) {
                break;
            }
        }

        // print the best move
        std::cout << "Best move: " << vetted_moves[0].get_fen() << std::endl;
        std::cout << "Evaluated " << minimax.positions_evaluated << " positions." << std::endl;

        // write the best move to the output file
        myFile.open("output_file.txt");
        myFile << vetted_moves[0].get_fen();
        myFile.close();
    }

    return 0;
}