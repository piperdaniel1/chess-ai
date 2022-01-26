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

Move * arr_to_linked_list(Board& board, std::string * arr, int size) {
    Move * head = new Move();
    Move * curr = head;
    Move * converted_move;
    for (int i = 0; i < size; i++) {
        converted_move = board.convert_move_fen(arr[i]);
        curr->from_x = converted_move->from_x;
        curr->from_y = converted_move->from_y;
        curr->to_x = converted_move->to_x;
        curr->to_y = converted_move->to_y;
        curr->promotion = converted_move->promotion;

        if (i != size - 1) {
            curr->next = new Move();
            curr = curr->next;
        }
    }

    curr->next = nullptr;
    return head;
}

int main() {
    Minimax minimax;
    Board board;
    std::ifstream input_file;
    std::ofstream myFile;
    std::string fen;
    std::string * move_fens;
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
        Move * sorted_legal_moves = board.get_legal_moves();
        // get the best move
        while(1) {
            move_fens = minimax.get_best_move(board, curr_depth, num_moves, sorted_legal_moves);
            sorted_legal_moves = arr_to_linked_list(board, move_fens, num_moves);
            curr_depth++;
            std::cout << "Depth " << curr_depth-1 << " complete, " << minimax.positions_evaluated << " positions evaluated." << std::endl;
            if (curr_depth > INITIAL_DEPTH + 2) {
                break;
            }
            std::cout << "Beginning depth " << curr_depth << "..." << std::endl;
        }

        // print the best move
        std::cout << "Best move: " << move_fens[0] << std::endl;
        std::cout << "Evaluated " << minimax.positions_evaluated << " positions." << std::endl;

        // write the best move to the output file
        myFile.open("output_file.txt");
        myFile << move_fens[0];
        myFile.close();
    }

    return 0;
}