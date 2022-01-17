#include "board.h"

Board::Board() {
    std::cout << "Initializing board..." << std::endl;     
}

void Board::clear_board() {
    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            board[i][j] = '.';
        }
    }
}

void Board::set_piece(int row, int col, char piece) {
    board[row][col] = piece;
}

// assumes that the move is legal
void Board::push_move(Move * move) {
    turn = !turn;
    board[move->to_y][move->to_x] = board[move->from_y][move->from_x];
    board[move->from_y][move->from_x] = '.';
}

Move * Board::get_rook_moves(Move * moves, int row, int col) {
    std::cout << "Getting rook moves at (" << col << ", " << row << ")." << std::endl;
    // check up
    for (int i = row - 1; i >= 0; i--) {
        if (board[i][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
        } else if (board[i][col] == 'P' || board[i][col] == 'N' || board[i][col] == 'B' || board[i][col] == 'R' || board[i][col] == 'Q' || board[i][col] == 'K') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    // check down
    for (int i = row + 1; i < 8; i++) {
        if (board[i][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
        } else if (board[i][col] == 'P' || board[i][col] == 'N' || board[i][col] == 'B' || board[i][col] == 'R' || board[i][col] == 'Q' || board[i][col] == 'K') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    // check left
    for (int i = col - 1; i >= 0; i--) {
        if (board[row][i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
        } else if (board[row][i] == 'P' || board[row][i] == 'N' || board[row][i] == 'B' || board[row][i] == 'R' || board[row][i] == 'Q' || board[row][i] == 'K') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    // check right
    for (int i = col + 1; i < 8; i++) {
        if (board[row][i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
        } else if (board[row][i] == 'P' || board[row][i] == 'N' || board[row][i] == 'B' || board[row][i] == 'R' || board[row][i] == 'Q' || board[row][i] == 'K') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    return moves;
}

void Board::free_move_list(Move * moves) {
    std::cout << "Freeing move list..." << std::endl;
    Move * temp;
    while (moves != nullptr) {
        temp = moves;
        moves = moves->next;
        delete temp;
    }
}

Move * Board::get_legal_moves() {
    std::cout << "Getting legal moves..." << std::endl;
    
    Move * moves = new Move;
    Move * list_end = moves;

    char piece;
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            piece = board[row][col];

            if(piece == '.') {
                continue;
            }

            if (turn == false) {
                if(piece == 'r') {
                    list_end = get_rook_moves(moves, row, col);
                }
            } else if (turn == true) {
                if(piece == 'R') {
                    list_end = get_rook_moves(moves, row, col);
                }
            }
        }
    }

    return moves;
}

void Board::print_self() {
    std::cout << "Printing board..." << std::endl;
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            std::cout << board[row][col] << " ";
        }
        std::cout << std::endl;
    }
}