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

    board.clear_board();
    board.set_piece(5, 3, 'r');
    board.set_piece(5, 6, 'N');
    board.print_self();
    Move * move_list = board.get_legal_moves();
    Move * curr_move = move_list;
    
    // print out the moves
    while (curr_move->next != nullptr) {
        std::cout << "Move: (" << curr_move->from_x << ", " << curr_move->from_y << ") to (" << curr_move->to_x << ", " << curr_move->to_y << ")" << std::endl;
        curr_move = curr_move->next;
    }

    board.push_move(move_list);
    board.print_self();

    // free the move list
    board.free_move_list(move_list);

    return 0;
}