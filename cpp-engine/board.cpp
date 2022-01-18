#include "board.h"

Board::Board() {
    std::cout << "Initializing board..." << std::endl;     
    // init black pieces
    black_pieces[0] = 'r';
    black_pieces[1] = 'n';
    black_pieces[2] = 'b';
    black_pieces[3] = 'q';
    black_pieces[4] = 'k';
    black_pieces[5] = 'p';
    // init white pieces
    white_pieces[0] = 'R';
    white_pieces[1] = 'N';
    white_pieces[2] = 'B';
    white_pieces[3] = 'Q';
    white_pieces[4] = 'K';
    white_pieces[5] = 'P';
}

Board::~Board() {
    std::cout << "Deleting board..." << std::endl;
    free(black_pieces);
    free(white_pieces);
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

// Assumes that arr is length six
bool Board::is_in_arr(char piece, char * arr) {
    for (int i = 0; i < 6; i++) {
        if (piece == arr[i]) {
            return true;
        }
    }
    return false;
}

bool Board::is_king_in_check(int row, int col) {
    char * enemy_pieces = turn ? black_pieces : white_pieces;

    // check for rooks and queens on the same row or column that are not blocked by other pieces
    for (int i = row - 1; i >= 0; i--) {
        if (board[i][col] == enemy_pieces[0] || board[i][col] == enemy_pieces[3]) {
            return true;
        } else if (board[i][col] != '.') {
            break;
        }
    }
    for (int i = row + 1; i < 0; i--) {
        if (board[i][col] == enemy_pieces[0] || board[i][col] == enemy_pieces[3]) {
            return true;
        } else if (board[i][col] != '.') {
            break;
        }
    }
    for (int i = col - 1; i >= 0; i--) {
        if (board[row][i] == enemy_pieces[0] || board[row][i] == enemy_pieces[3]) {
            return true;
        }  else if (board[row][i] != '.') {
            break;
        }
    }
    for (int i = col + 1; i < 8; i++) {
        if (board[row][i] == enemy_pieces[0] || board[row][i] == enemy_pieces[3]) {
            return true;
        } else if (board[row][i] != '.') {
            break;
        }
    }

    // check for bishops and queens on the same diagonal that are not blocked by other pieces
    for (int i = 1; i < 8; i++) {
        if (row + i < 8 && col + i < 8) {
            if (board[row + i][col + i] == enemy_pieces[2] || board[row + i][col + i] == enemy_pieces[3]) {
                return true;
            } else if (board[row + i][col + i] != '.') {
                break;
            }
        } else {
            break;
        }
    }
    for (int i = 1; i < 8; i++) {
        if (row - i >= 0 && col - i >= 0) {
            if (board[row - i][col - i] == enemy_pieces[2] || board[row - i][col - i] == enemy_pieces[3]) {
                return true;
            } else if (board[row - i][col - i] != '.') {
                break;
            }
        } else {
            break;
        }
    }
    for (int i = 1; i < 8; i++) {
        if (row + i < 8 && col - i >= 0) {
            if (board[row + i][col - i] == enemy_pieces[2] || board[row + i][col - i] == enemy_pieces[3]) {
                return true;
            } else if (board[row + i][col - i] != '.') {
                break;
            }
        } else {
            break;
        }
    }
    for (int i = 1; i < 8; i++) {
        if (row - i >= 0 && col + i < 8) {
            if (board[row - i][col + i] == enemy_pieces[2] || board[row - i][col + i] == enemy_pieces[3]) {
                return true;
            } else if (board[row - i][col + i] != '.') {
                break;
            }
        } else {
            break;
        }
    }

    // check for knights that are two and one spaces away from the king
    if (row + 2 < 8 && col + 1 < 8) {
        if (board[row + 2][col + 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row + 2 < 8 && col - 1 >= 0) {
        if (board[row + 2][col - 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 2 >= 0 && col + 1 < 8) {
        if (board[row - 2][col + 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 2 >= 0 && col - 1 >= 0) {
        if (board[row - 2][col - 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row + 1 < 8 && col + 2 < 8) {
        if (board[row + 1][col + 2] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row + 1 < 8 && col - 2 >= 0) {
        if (board[row + 1][col - 2] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 1 >= 0 && col + 2 < 8) {
        if (board[row - 1][col + 2] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 1 >= 0 && col - 2 >= 0) {
        if (board[row - 1][col - 2] == enemy_pieces[1]) {
            return true;
        }
    }

    // check for the enemy king that is one space away from the king
    for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
            if (i == 0 && j == 0) {
                continue;
            }

            if (row + i < 0 || row + i > 7 || col + j < 0 || col + j > 7) {
                continue;
            }

            if (board[row + i][col + j] == enemy_pieces[4]) {
                return true;
            }
        }
    }

    // check for enemy pawn on digonal from king
    // if we are checking for the black king we should be checking for white pawns
    // that are on a lower row than the king
    if (turn == false) {
        if (row - 1 > 0 && col + 1 < 8) {
            if (board[row - 1][col + 1] == enemy_pieces[5]) {
                return true;
            }
        }
        if (row - 1 > 0 && col - 1 >= 0) {
            if (board[row - 1][col - 1] == enemy_pieces[5]) {
                return true;
            }
        }
    // if we are checking for the white king we should be checking for black pawns
    // that are on a higher row than the king
    } else {
        if (row + 1 < 8 && col + 1 < 8) {
            if (board[row + 1][col + 1] == enemy_pieces[5]) {
                return true;
            }
        }
        if (row + 1 < 8 && col - 1 >= 0) {
            if (board[row + 1][col - 1] == enemy_pieces[5]) {
                return true;
            }
        }
    }

    return false;
}

Move * Board::get_king_moves(Move * moves, int row, int col) {
    std::cout << "Getting king moves at (" << col << ", " << row << ")." << std::endl;

    char * pieces = turn ? black_pieces : white_pieces;

    // check for moves in all directions
    for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
            if (i == 0 && j == 0) {
                continue;
            }

            if (row + i < 0 || row + i > 7 || col + j < 0 || col + j > 7) {
                continue;
            }

            if (board[row + i][col + j] == '.' || is_in_arr(board[row + i][col + j], pieces)) {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col + j;
                moves->to_y = row + i;
                moves->next = new Move;
                moves = moves->next;
            }
        }
    }

    return moves;
}

Move * Board::get_rook_moves(Move * moves, int row, int col) {
    std::cout << "Getting rook moves at (" << col << ", " << row << ")." << std::endl;
    char * pieces = turn ? black_pieces : white_pieces;
    
    // check up
    for (int i = row - 1; i >= 0; i--) {
        if (board[i][col] == '.' || is_in_arr(board[i][col], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
        } else {
            break;
        }
    }

    // check down
    for (int i = row + 1; i < 8; i++) {
        if (board[i][col] == '.' || is_in_arr(board[i][col], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
        } else {
            break;
        }
    }

    // check left
    for (int i = col - 1; i >= 0; i--) {
        if (board[row][i] == '.' || is_in_arr(board[row][i], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
        } else {
            break;
        }
    }

    // check right
    for (int i = col + 1; i < 8; i++) {
        if (board[row][i] == '.' || is_in_arr(board[row][i], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
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

Move * Board::get_pseudo_legal_moves() {
    std::cout << "Getting pseudo legal moves..." << std::endl;
    
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
                    list_end = get_rook_moves(list_end, row, col);
                } else if(piece == 'k') {
                    list_end = get_king_moves(list_end, row, col);
                }
            } else if (turn == true) {
                if(piece == 'R') {
                    list_end = get_rook_moves(list_end, row, col);
                } else if(piece == 'K') {
                    list_end = get_king_moves(list_end, row, col);
                }
            }
        }
    }

    return moves;
}

void Board::pull_move(Move * move, int captured_piece = NULL) {
}

bool Board::is_legal_move(Move * move) {
    return;
}

Move * Board::get_legal_moves() {
    std::cout << "Getting legal moves..." << std::endl;

    Move * pseudo_legal_moves = get_pseudo_legal_moves();
    Move * moves = new Move;
    Move * list_end = moves;

    Move * temp;
    while (pseudo_legal_moves != nullptr) {
        temp = pseudo_legal_moves;
        pseudo_legal_moves = pseudo_legal_moves->next;

        if (is_legal_move(temp)) {
            list_end->from_x = temp->from_x;
            list_end->from_y = temp->from_y;
            list_end->to_x = temp->to_x;
            list_end->to_y = temp->to_y;
            list_end->next = new Move;
            list_end = list_end->next;
        }

        delete temp;
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

bool Board::check_on_board() {
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            if(board[row][col] == '.') {
                return false;
            }
        }
    }

    return false;
}