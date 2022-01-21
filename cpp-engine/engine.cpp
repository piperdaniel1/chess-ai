// this file will contain the main function for the cpp-engine
#include <iostream>
#include "minimax.h"
#include "board.h"
#include "fstream"

// $ g++ engine.cpp minimax.cpp tt_table.cpp evaluator.cpp board.cpp
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

/*
 * We can check if the king is in check by generating all legal moves for the opponent and then checking if one of these
 * moves is to the king's position. We can also trim down the king's moves by making sure it can not move into one of
 * these moves. We need to use a special legal moves function that allows the opponent to take their own pieces. this will
 * make sure that the king cannot take a piece that is guarded by the opponent.
 * 
 * Now we also need to make sure that we include moves for us that block the check. I think because of discovered checks
 * the only way to do this is to execute the pseudo-legal move we have and make sure that the king is not in check.
 *
 */


/*
 * THIS OPTION IS CURRENTLY IMPLEMENTED:
 * Okay fuck that. I think that will be way too slow because we will have to generate all of the moves of the opponent fresh for
 * every pseudo-legal move we have, all the time. Maybe we should just have a king-centric function that scans diagonals, straights,
 * and knight paths to look for an opponent piece. We would then have to run this function on every pseudo-legal move we have to make
 * sure they don't leave the king in check. This would be a lot faster I think. It also should handle every possible case of check.
 * Also it will be really really fast when the king is close to the edge of the board surrounded by its own pieces. This is often
 * the case.
 * 
 * THIS IS NOT IMPLEMENTED:
 * You could potentially optimize this by caching the squares that the king had to check. If a move is not from these squares then
 * you don't need to check it. However, premature optimization is the root of all evil.
 */


/*
 * TODO make it so that pawns cannot jump pieces when moving forward two squares.
 */
int main() {
    std::cout << "Initializing chess engine..." << std::endl;
    Minimax minimax;
    Board board;

    //board.push_move(board.convert_move_fen("e2e4"));
    //board.turn = false;

    board.import_board_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    //bug:
    //starting from above pos:
    //c5c4 -> a6f6
    //engine evaluates g8f7 as a possible move when it leaves the white king in check from the enemy queen.
    //bug must be caused by is_legal_move function.

    board.print_self();
    board.print_board_metadata();

    Move * best_move = minimax.get_best_move(board, 3);
    std::cout << "Best move: " << best_move->from_y << " " << best_move->from_x << "  " << best_move->to_y << " " << best_move->to_x << std::endl;

    std::cout << "Evaluated " << minimax.positions_evaluated << " positions." << std::endl;
    delete best_move;
    board.free_piece_lists();

    // c5c4: 1410 (+1)
    // d7d5: 1643 (perfect)
    // f6d5: 1687 (perfect)
    // b5c4: 1353 (+1)
    // f8f7: 1634 (+11)
    // g8h8: 1752 (-1 missing)
    // full: 9479 (+12)

    // New "fixed" version:
    // c5c4: 1410 (+1)
    // d7d5: 1643 (perfect)
    // f6d5: 1687 (perfect)
    // b5c4: 1353 (+1)
    // f8f7: 1622 (-1)
    // g8h8: 1752 (-1)

    // New "more fixed" version:
    // c5c4: 1409 (perfect)
    // d7d5: 1642 (-1)
    // f6d5: 1686 (-1)
    // b5c4: 1352 (perfect)
    // f8f7: 1622 (-1)
    // g8h8: 1752 (-1)
 
    /*std::ifstream fen_file;
    fen_file.open("board_file.txt", std::ios::in);
    std::string fen = "";
    std::getline(fen_file, fen);
    fen_file.close();

    std::cout << "FEN: " << fen << std::endl;

    board.import_board_fen(fen);
    std::cout << "Board imported." << std::endl;

    Board board2 = board;

    Move * move_list;
    Move * curr_move;
    std::string move_str;

    while(true) {
        board2.print_self();
        board2.print_board_metadata();

        std::cout << "Generating moves..." << std::endl;
        move_list = board2.get_legal_moves();
        curr_move = move_list;
        int num_moves = 0;

        while (curr_move != nullptr) {
            std::cout << "Move: (" << curr_move->from_y << ", " << curr_move->from_x << ") to (" << curr_move->to_y << ", " << curr_move->to_x << ")" << std::endl;
            curr_move = curr_move->next;
            num_moves++;
        }

        std::cout << "Number of moves: " << num_moves << std::endl;

        // get move from user
        std::cout << "Enter move: ";
        std::cin >> move_str;
        board2.push_move(board2.convert_move_fen(move_str));
    }*/


    /*std::cout << "testing move fen conversion" << std::endl;
    Move * move = board.convert_move_fen("g7a2");
    std::cout << "Move: (" << move->from_y << ", " << move->from_x << ") to (" << move->to_y << ", " << move->to_x << ")" << std::endl;

    board.clear_board();
    board.set_piece(0, 4, 'k');
    board.set_piece(0, 7, 'r');
    board.set_piece(7, 4, 'K');
    board.set_piece(4, 4, 'N');
    board.set_piece(7, 5, 'Q');
    board.set_piece(7, 7, 'R');
    board.print_self();
    
    std::cout << "Is king in check? " << board.is_king_in_check(5, 6) << std::endl;
    Move * move_list = board.get_legal_moves();
    Move * curr_move = move_list;
    Move best_move;
    
    // print out the moves
    while (curr_move->next != nullptr) {
        std::cout << "Move: (" << curr_move->from_y << ", " << curr_move->from_x << ") to (" << curr_move->to_y << ", " << curr_move->to_x << ")" << std::endl;
        curr_move = curr_move->next;

        //if(curr_move->from_x == 4 && curr_move->from_y == 7 && curr_move->to_x == 6 && curr_move->to_y == 7) {
        //   best_move = *curr_move;
        //}
    }

    //board.push_move(&best_move);
    //board.print_self();

    // free the move list
    board.free_move_list(move_list);*/

    return 0;
}