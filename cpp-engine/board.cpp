#include "board.h"

Board::Board() {
    std::cout << "Initializing board..." << std::endl;     
    // init black pieces
    this->black_pieces[0] = 'r';
    this->black_pieces[1] = 'n';
    this->black_pieces[2] = 'b';
    this->black_pieces[3] = 'q';
    this->black_pieces[4] = 'L';
    this->black_pieces[5] = 'p';
    // init white pieces
    this->white_pieces[0] = 'R';
    this->white_pieces[1] = 'N';
    this->white_pieces[2] = 'B';
    this->white_pieces[3] = 'Q';
    this->white_pieces[4] = 'L';
    this->white_pieces[5] = 'P';

    this->black_king = 'k';
    this->white_king = 'K';

    this->enPassantCol = -1;
    this->enPassantRow = -1;

    this->white_kingside_castling = true;
    this->white_queenside_castling = true;
    this->black_kingside_castling = true;
    this->black_queenside_castling = true;
}

Board::~Board() {
    std::cout << "Deleting board..." << std::endl;
    delete black_pieces;
    delete white_pieces;
}

void Board::clear_board() {
    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            this->board[i][j] = '.';
        }
    }
}

void Board::set_piece(int row, int col, char piece) {
    this->board[row][col] = piece;
}

// assumes that the move is legal does not support castling
char Board::push_move(Move * move) {    
    // execute castling move if applicable
    if(!this->turn) {
        if(this->black_queenside_castling) {
            if(move->from_x == 4 && move->from_y == 0 && move->to_x == 2 && move->to_y == 0) {
                this->board[0][2] = 'k';
                this->board[0][3] = 'r';
                this->board[0][0] = '.';
                this->board[0][4] = '.';
                this->turn = !this->turn;
                this->black_kingside_castling = false;
                this->black_queenside_castling = false;
                return '.';
            }
        }
        if (this->black_kingside_castling) {
            if(move->from_x == 4 && move->from_y == 0 && move->to_x == 6 && move->to_y == 0) {
                // execute black kingside castle
                this->board[0][6] = 'k';
                this->board[0][5] = 'r';
                this->board[0][7] = '.';
                this->board[0][4] = '.';
                this->turn = !this->turn;
                this->black_kingside_castling = false;
                this->black_queenside_castling = false;
                return '.';
            }
        }
    } else {
        if(this->white_queenside_castling) {
            if(move->from_x == 4 && move->from_y == 7 && move->to_x == 2 && move->to_y == 7) {
                // execute white qeenside castle
                this->board[7][2] = 'K';
                this->board[7][3] = 'R';
                this->board[7][0] = '.';
                this->board[7][4] = '.';
                this->turn = !this->turn;
                this->white_kingside_castling = false;
                this->white_queenside_castling = false;
                return '.';
            }
        } 
        if (this->white_kingside_castling) {
            if(move->from_x == 4 && move->from_y == 7 && move->to_x == 6 && move->to_y == 7) {
                // execute white kingside castle
                std::cout << "pushing white kingside castle move" << std::endl;
                this->board[7][6] = 'K';
                this->board[7][5] = 'R';
                this->board[7][7] = '.';
                this->board[7][4] = '.';
                this->turn = !this->turn;
                this->white_kingside_castling = false;
                this->white_queenside_castling = false;
                return '.';
            }
        }
    }

    // remove castle rights if applicable
    if(!this->turn) {
        if(this->black_kingside_castling) {
            if(move->from_x == 7 && move->from_y == 0) {
                this->black_kingside_castling = false;
            } else if (move->from_x == 4 && move->from_y == 0) {
                this->black_kingside_castling = false;
                this->black_queenside_castling = false;
            }
        } else if (this->black_queenside_castling) {
            if(move->from_x == 0 && move->from_y == 0) {
                this->black_kingside_castling = false;
            } else if (move->from_x == 4 && move->from_y == 0) {
                this->black_kingside_castling = false;
                this->black_queenside_castling = false;
            }
        }
    } else {
        if(this->white_kingside_castling) {
            if(move->from_x == 7 && move->from_y == 7) {
                this->white_kingside_castling = false;
            } else if (move->from_x == 4 && move->from_y == 7) {
                this->white_kingside_castling = false;
                this->white_queenside_castling = false;
            }
        } else if (this->white_queenside_castling) {
            if(move->from_x == 0 && move->from_y == 0) {
                this->white_kingside_castling = false;
            } else if (move->from_x == 7 && move->from_y == 7) {
                this->white_kingside_castling = false;
                this->white_queenside_castling = false;
            }
        }
    }

    // TODO remove en passant if applicable

    // TODO add en passant rights if applicable

    // execute move
    char captured_piece = this->board[move->to_y][move->to_x];
    this->board[move->to_y][move->to_x] = this->board[move->from_y][move->from_x];
    this->board[move->from_y][move->from_x] = '.';

    if (captured_piece != '.') {
        return captured_piece;
    }

    this->turn = !this->turn;
    return '.';
}

// for internal use
char Board::fake_push_move(Move * move) {
    char captured_piece = this->board[move->to_y][move->to_x];
    this->board[move->to_y][move->to_x] = this->board[move->from_y][move->from_x];
    this->board[move->from_y][move->from_x] = '.';

    if (captured_piece != '.') {
        return captured_piece;
    }

    return '.';
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
    char * enemy_pieces = this->turn ? black_pieces : white_pieces;
    char enemy_king = this->turn ? this->black_king : this->white_king;

    // check for rooks and queens on the same row or column that are not blocked by other pieces
    for (int i = row - 1; i >= 0; i--) {
        if (this->board[i][col] == enemy_pieces[0] || this->board[i][col] == enemy_pieces[3]) {
            return true;
        } else if (this->board[i][col] != '.') {
            break;
        }
    }
    for (int i = row + 1; i < 0; i--) {
        if (this->board[i][col] == enemy_pieces[0] || this->board[i][col] == enemy_pieces[3]) {
            return true;
        } else if (this->board[i][col] != '.') {
            break;
        }
    }
    for (int i = col - 1; i >= 0; i--) {
        if (this->board[row][i] == enemy_pieces[0] || this->board[row][i] == enemy_pieces[3]) {
            return true;
        }  else if (this->board[row][i] != '.') {
            break;
        }
    }
    for (int i = col + 1; i < 8; i++) {
        if (this->board[row][i] == enemy_pieces[0] || this->board[row][i] == enemy_pieces[3]) {
            return true;
        } else if (this->board[row][i] != '.') {
            break;
        }
    }

    // check for bishops and queens on the same diagonal that are not blocked by other pieces
    for (int i = 1; i < 8; i++) {
        if (row + i < 8 && col + i < 8) {
            if (this->board[row + i][col + i] == enemy_pieces[2] || this->board[row + i][col + i] == enemy_pieces[3]) {
                return true;
            } else if (this->board[row + i][col + i] != '.') {
                break;
            }
        } else {
            break;
        }
    }
    for (int i = 1; i < 8; i++) {
        if (row - i >= 0 && col - i >= 0) {
            if (this->board[row - i][col - i] == enemy_pieces[2] || this->board[row - i][col - i] == enemy_pieces[3]) {
                return true;
            } else if (this->board[row - i][col - i] != '.') {
                break;
            }
        } else {
            break;
        }
    }
    for (int i = 1; i < 8; i++) {
        if (row + i < 8 && col - i >= 0) {
            if (this->board[row + i][col - i] == enemy_pieces[2] || this->board[row + i][col - i] == enemy_pieces[3]) {
                return true;
            } else if (this->board[row + i][col - i] != '.') {
                break;
            }
        } else {
            break;
        }
    }
    for (int i = 1; i < 8; i++) {
        if (row - i >= 0 && col + i < 8) {
            if (this->board[row - i][col + i] == enemy_pieces[2] || this->board[row - i][col + i] == enemy_pieces[3]) {
                return true;
            } else if (this->board[row - i][col + i] != '.') {
                break;
            }
        } else {
            break;
        }
    }

    // check for knights that are two and one spaces away from the king
    if (row + 2 < 8 && col + 1 < 8) {
        if (this->board[row + 2][col + 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row + 2 < 8 && col - 1 >= 0) {
        if (this->board[row + 2][col - 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 2 >= 0 && col + 1 < 8) {
        if (this->board[row - 2][col + 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 2 >= 0 && col - 1 >= 0) {
        if (this->board[row - 2][col - 1] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row + 1 < 8 && col + 2 < 8) {
        if (this->board[row + 1][col + 2] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row + 1 < 8 && col - 2 >= 0) {
        if (this->board[row + 1][col - 2] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 1 >= 0 && col + 2 < 8) {
        if (this->board[row - 1][col + 2] == enemy_pieces[1]) {
            return true;
        }
    }
    if (row - 1 >= 0 && col - 2 >= 0) {
        if (this->board[row - 1][col - 2] == enemy_pieces[1]) {
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

            if (this->board[row + i][col + j] == enemy_king) {
                return true;
            }
        }
    }

    // check for enemy pawn on digonal from king
    // if we are checking for the black king we should be checking for white pawns
    // that are on a lower row than the king
    if (turn == false) {
        if (row - 1 > 0 && col + 1 < 8) {
            if (this->board[row - 1][col + 1] == enemy_pieces[5]) {
                return true;
            }
        }
        if (row - 1 > 0 && col - 1 >= 0) {
            if (this->board[row - 1][col - 1] == enemy_pieces[5]) {
                return true;
            }
        }
    // if we are checking for the white king we should be checking for black pawns
    // that are on a higher row than the king
    } else {
        if (row + 1 < 8 && col + 1 < 8) {
            if (this->board[row + 1][col + 1] == enemy_pieces[5]) {
                return true;
            }
        }
        if (row + 1 < 8 && col - 1 >= 0) {
            if (this->board[row + 1][col - 1] == enemy_pieces[5]) {
                return true;
            }
        }
    }

    return false;
}

int Board::min(int a, int b) {
    if (a < b) {
        return a;
    }

    return b;
}

int Board::max(int a, int b) {
    if (a > b) {
        return a;
    }

    return b;
}

Move * Board::get_pawn_moves(Move * moves, int row, int col) {
    std::cout << "Getting pawn moves at " << row << " " << col << std::endl;
    char * pieces = turn ? black_pieces : white_pieces;

    if(this->turn) {
        // check the square right in front of the pawn
        if(board[row-1][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = row - 1;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square two in front of the pawn (if it is the first move)
        if(row == 6 && board[row-2][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = row - 2;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square to the diagonal left of the pawn
        if(col - 1 >= 0 && (this->is_in_arr(board[row-1][col-1], pieces) || row-1 == this->enPassantRow && col-1 == this->enPassantCol)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col - 1;
            moves->to_y = row - 1;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square to the diagonal right of the pawn
        if(col + 1 < 8 && (this->is_in_arr(board[row-1][col+1], pieces) || row-1 == this->enPassantRow && col+1 == this->enPassantCol)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col + 1;
            moves->to_y = row - 1;
            moves->next = new Move;
            moves = moves->next;
        }
    } else {
        // check the square right in front of the pawn
        if(board[row+1][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = row + 1;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square two in front of the pawn (if it is the first move)
        if(row == 1 && board[row+2][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = row + 2;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square to the diagonal left of the pawn
        if(col - 1 >= 0 && (this->is_in_arr(board[row+1][col-1], pieces) || row+1 == this->enPassantRow && col-1 == this->enPassantCol)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col - 1;
            moves->to_y = row + 1;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square to the diagonal right of the pawn
        if(col + 1 < 8 && (this->is_in_arr(board[row+1][col+1], pieces) || row+1 == this->enPassantRow && col+1 == this->enPassantCol)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col + 1;
            moves->to_y = row + 1;
            moves->next = new Move;
            moves = moves->next;
        }
    }

    return moves;
}

Move * Board::get_bishop_moves(Move * moves, int row, int col) {
    std::cout << "Getting bishop moves at " << row << " " << col << std::endl;
    char * pieces = turn ? black_pieces : white_pieces;

    // check towards the top left
    for (int i=1; i<=min(row, col); i++) {
        if (this->board[row-i][col-i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col - i;
            moves->to_y = row - i;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[row-i][col-i], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col - i;
            moves->to_y = row - i;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    // check towards the top right
    for (int i=1; i<=min(row, 7-col); i++) {
        if (this->board[row-i][col+i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col + i;
            moves->to_y = row - i;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[row-i][col+i], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col + i;
            moves->to_y = row - i;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    // check towards the bottom left
    for (int i=1; i<=min(7-row, col); i++) {
        if (this->board[row+i][col-i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col - i;
            moves->to_y = row + i;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[row+i][col-i], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col - i;
            moves->to_y = row + i;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    // check towards the bottom right
    for (int i=1; i<=min(7-row, 7-col); i++) {
        if (this->board[row+i][col+i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col + i;
            moves->to_y = row + i;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[row+i][col+i], pieces)) {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col + i;
            moves->to_y = row + i;
            moves->next = new Move;
            moves = moves->next;
            break;
        } else {
            break;
        }
    }

    return moves;
}

Move * Board::get_queen_moves(Move * moves, int row, int col) {
    std::cout << "Getting queen moves at " << row << " " << col << std::endl;

    moves = this->get_bishop_moves(moves, row, col);
    moves = this->get_rook_moves(moves, row, col);

    return moves;
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

            if (this->board[row + i][col + j] == '.' || is_in_arr(this->board[row + i][col + j], pieces)) {
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

Move * Board::get_knight_moves(Move * moves, int row, int col) {
    std::cout << "Getting knight moves at " << row << " " << col << std::endl;

    if (row - 2 >= 0 && col - 1 >= 0) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 1;
        moves->to_y = row - 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row - 2 >= 0 && col + 1 <= 7) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 1;
        moves->to_y = row - 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row - 1 >= 0 && col - 2 >= 0) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 2;
        moves->to_y = row - 1;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row - 1 >= 0 && col + 2 <= 7) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 2;
        moves->to_y = row - 1;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 2 <= 7 && col - 1 >= 0) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 1;
        moves->to_y = row + 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 2 <= 7 && col + 1 <= 7) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 1;
        moves->to_y = row + 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 1 <= 7 && col - 2 >= 0) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 2;
        moves->to_y = row + 1;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 1 <= 7 && col + 2 <= 7) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 2;
        moves->to_y = row + 1;
        moves->next = new Move;
        moves = moves->next;
    }

    return moves;
}

Move * Board::get_rook_moves(Move * moves, int row, int col) {
    std::cout << "Getting rook moves at (" << col << ", " << row << ")." << std::endl;
    char * pieces = turn ? black_pieces : white_pieces;
    
    // check up
    for (int i = row - 1; i >= 0; i--) {
        if (this->board[i][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
        } else if ( is_in_arr(this->board[i][col], pieces)) {
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
        if (this->board[i][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = i;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[i][col], pieces)) {
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
        if (this->board[row][i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[row][i], pieces)) {
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
        if (this->board[row][i] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = i;
            moves->to_y = row;
            moves->next = new Move;
            moves = moves->next;
        } else if (is_in_arr(this->board[row][i], pieces)) {
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

// we don't deal with checking castling rights here, we assume
// that the Board::castling_rights booleans are correct and up to date
// those booleans get changed in push_move.
Move * Board::get_castling_moves(Move * moves) {
    if(!this->turn) {
        // check for black castling
        if(this->black_kingside_castling && 
          !this->is_king_in_check(0, 5) && !this->is_king_in_check(0,4)
          && this->board[0][5] == '.' && this->board[0][6] == '.') {
            // TODO check if there is a check along the way
            moves->from_x = 4;
            moves->from_y = 0;
            moves->to_x = 6;
            moves->to_y = 0;
            moves->next = new Move;
            moves = moves->next;
        }

        if(this->black_queenside_castling && 
          !this->is_king_in_check(0,3) && !this->is_king_in_check(0,4)
          && this->board[0][1] == '.' && this->board[0][2] == '.' && this->board[0][3] == '.') {
            moves->from_x = 4;
            moves->from_y = 0;
            moves->to_x = 2;
            moves->to_y = 0;
            moves->next = new Move;
            moves = moves->next;
        }
    } else {
        // check for white castling
        if(this->white_kingside_castling && 
        !this->is_king_in_check(7, 5) && !this->is_king_in_check(7,4)
        && this->board[7][5] == '.' && this->board[7][6] == '.') {
            moves->from_x = 4;
            moves->from_y = 7;
            moves->to_x = 6;
            moves->to_y = 7;
            moves->next = new Move;
            moves = moves->next;
        }

        if(this->white_queenside_castling && 
        !this->is_king_in_check(7, 3) && !this->is_king_in_check(7,4)
        && this->board[7][1] && this->board[7][2] == '.' && this->board[7][3] == '.') {
            moves->from_x = 4;
            moves->from_y = 7;
            moves->to_x = 2;
            moves->to_y = 7;
            moves->next = new Move;
            moves = moves->next;
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
            piece = this->board[row][col];

            if(piece == '.') {
                continue;
            }

            if (turn == false) {
                if(piece == 'r') {
                    list_end = this->get_rook_moves(list_end, row, col);
                } else if(piece == 'k') {
                    list_end = this->get_king_moves(list_end, row, col);
                } else if(piece == 'b') {
                    list_end = this->get_bishop_moves(list_end, row, col);
                } else if(piece == 'q') {
                    list_end = this->get_queen_moves(list_end, row, col);
                } else if(piece == 'n') {
                    list_end = this->get_knight_moves(list_end, row, col);
                }
            } else if (turn == true) {
                if(piece == 'R') {
                    list_end = this->get_rook_moves(list_end, row, col);
                } else if(piece == 'K') {
                    list_end = this->get_king_moves(list_end, row, col);
                } else if(piece == 'B') {
                    list_end = this->get_bishop_moves(list_end, row, col);
                } else if(piece == 'Q') {
                    list_end = this->get_queen_moves(list_end, row, col);
                } else if(piece == 'N') {
                    list_end = this->get_knight_moves(list_end, row, col);
                }
            }
        }
    }

    this->get_castling_moves(list_end);

    return moves;
}

// does not support castling
void Board::pull_move(Move * move, int captured_piece = '.') {
    char piece = this->board[move->to_y][move->to_x];

    if (captured_piece != '.') {
        this->board[move->to_y][move->to_x] = captured_piece;
    } else {
        this->board[move->to_y][move->to_x] = '.';
    }
    
    this->board[move->from_y][move->from_x] = piece;
}

int * Board::get_king_pos() {
    for(int i=0; i<8; i++) {
        for(int j=0; j<8; j++) {
            if(this->board[i][j] == 'k' and this->turn == false) {
                int * king_pos = new int[2];
                king_pos[0] = i;
                king_pos[1] = j;
                return king_pos;
            } else if (this->board[i][j] == 'K' and this->turn == true) {
                int * king_pos = new int[2];
                king_pos[0] = i;
                king_pos[1] = j;
                return king_pos;
            }
        }
    }

    return nullptr;
}

bool Board::is_legal_move(Move * move) {
    char captured = this->fake_push_move(move);

    int * king_pos = this->get_king_pos();

    int row = king_pos[0];
    int col = king_pos[1];

    if(this->is_king_in_check(row, col) == true) {
        this->pull_move(move, captured);

        delete king_pos;
        return false;
    }

    this->pull_move(move, captured);

    delete king_pos;
    return true;
}

Move * Board::get_legal_moves() {
    Move * pseudo_legal_moves = this->get_pseudo_legal_moves();

    /*std::cout << "ALL PSEUDO LEGAL MOVES:" << std::endl;
    Move * curr_move = pseudo_legal_moves;
    // print out the moves
    while (curr_move->next != nullptr) {
        std::cout << "Move: (" << curr_move->from_x << ", " << curr_move->from_y << ") to (" << curr_move->to_x << ", " << curr_move->to_y << ")" << std::endl;
        curr_move = curr_move->next;
    }
    std::cout << std::endl;*/

    Move * moves = new Move;
    Move * list_end = moves;
    Move * prev_end;

    Move * temp;
    while (pseudo_legal_moves != nullptr) {
        temp = pseudo_legal_moves;
        pseudo_legal_moves = pseudo_legal_moves->next;

        if (this->is_legal_move(temp)) {
            list_end->from_x = temp->from_x;
            list_end->from_y = temp->from_y;
            list_end->to_x = temp->to_x;
            list_end->to_y = temp->to_y;
            list_end->next = new Move;
            prev_end = list_end;
            list_end = list_end->next;
        }

        delete temp;
    }

    delete list_end;
    prev_end->next = nullptr;
    return moves;
}

void Board::print_self() {
    std::cout << "Printing this->board..." << std::endl;
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            std::cout << this->board[row][col] << " ";
        }
        std::cout << std::endl;
    }
}

bool Board::check_on_board() {
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            if(this->board[row][col] == '.') {
                return false;
            }
        }
    }

    return false;
}