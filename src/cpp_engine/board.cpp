#include "board.h"

MovC::MovC() {
    this->from_x = 0;
    this->from_y = 0;
    this->to_x = 0;
    this->to_y = 0;

    this->promotion = '.';
    this->next = nullptr;
}
MovC::MovC(int from_x, int from_y, int to_x, int to_y) {
    this->from_x = from_x;
    this->from_y = from_y;
    this->to_x = to_x;
    this->to_y = to_y;

    this->promotion = '.';
    this->next = nullptr;
}
MovC::MovC(int from_x, int from_y, int to_x, int to_y, char promotion) {
    this->from_x = from_x;
    this->from_y = from_y;
    this->to_x = to_x;
    this->to_y = to_y;

    this->promotion = promotion;
    this->next = nullptr;
}
MovC::MovC(std::string fen) {
    this->from_x = fen[0] - 'a';
    this->from_y = 7 -(fen[1] - '1');
    this->to_x = fen[2] - 'a';
    this->to_y = 7 - (fen[3] - '1');
    if (fen.length() == 5) {
        this->promotion = fen[4];
    } else {
        this->promotion = '.';
    }
}
MovC::~MovC() {
    // nothing to do
}
MovC::MovC(const MovC& other) {
    this->from_x = other.from_x;
    this->from_y = other.from_y;
    this->to_x = other.to_x;
    this->to_y = other.to_y;

    this->promotion = other.promotion;
    this->next = other.next;
}
MovC& MovC::operator=(const MovC& other) {
    this->from_x = other.from_x;
    this->from_y = other.from_y;
    this->to_x = other.to_x;
    this->to_y = other.to_y;

    this->promotion = other.promotion;
    this->next = other.next;
    
    return *this;
}
std::string MovC::get_fen() {
    std::string fen = "";
    fen += (char)(this->from_x + 'a');
    fen += (char)(7 - this->from_y + '1');
    fen += (char)(this->to_x + 'a');
    fen += (char)(7 - this->to_y + '1');

    if(this->promotion != '.') {
        fen += this->promotion;
    }

    return fen;
}

Board::Board() {
    //std::cout << "Initializing board..." << std::endl;
    this->black_pieces = new char[6];
    this->white_pieces = new char[6];

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

    this->fullmove_number = 0;
    this->halfmove_clock = 0;

    this->black_king = 'k';
    this->white_king = 'K';

    this->enPassantCol = -1;
    this->enPassantRow = -1;

    this->white_kingside_castling = true;
    this->white_queenside_castling = true;
    this->black_kingside_castling = true;
    this->black_queenside_castling = true;

    this->import_board_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}

Board::~Board() {
    //std::cout << "Deleting board..." << std::endl;
}

Board::Board(const Board & b) {
    //std::cout << "Copying board..." << std::endl;
    this->black_pieces = b.black_pieces;
    this->white_pieces = b.white_pieces;
    this->black_king = b.black_king;
    this->white_king = b.white_king;

    this->fullmove_number = b.fullmove_number;
    this->halfmove_clock = b.halfmove_clock;

    this->enPassantCol = b.enPassantCol;
    this->enPassantRow = b.enPassantRow;

    this->white_kingside_castling = b.white_kingside_castling;
    this->white_queenside_castling = b.white_queenside_castling;
    this->black_kingside_castling = b.black_kingside_castling;
    this->black_queenside_castling = b.black_queenside_castling;

    this->turn = b.turn;

    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            this->board[i][j] = b.board[i][j];
        }
    }
}

void Board::free_piece_lists() {
    delete [] this->black_pieces;
    delete [] this->white_pieces;
}

void Board::clear_board() {
    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            this->board[i][j] = '.';
        }
    }
}

char Board::get_piece(int row, int col) {
    return this->board[row][col];
}

void Board::set_piece(int row, int col, char piece) {
    this->board[row][col] = piece;
}

void Board::print_board_metadata() {
    std::cout << "Printing board metadata..." << std::endl;
    std::cout << "Turn: " << this->turn << std::endl;
    std::cout << "En Passant: " << this->enPassantCol << " " << this->enPassantRow << std::endl;
    std::cout << "White Kingside Castling: " << this->white_kingside_castling << std::endl;
    std::cout << "White Queenside Castling: " << this->white_queenside_castling << std::endl;
    std::cout << "Black Kingside Castling: " << this->black_kingside_castling << std::endl;
    std::cout << "Black Queenside Castling: " << this->black_queenside_castling << std::endl;
    std::cout << "Halfmove Clock: " << this->halfmove_clock << std::endl;
    std::cout << "Fullmove Number: " << this->fullmove_number << std::endl;
}

void Board::import_board_fen(std::string fen) {
    this->clear_board();

    std::string row = "";
    int row_num = 0;
    int col_num = 0;
    int cursor = 0;
    for (int i = 0; i < fen.length(); i++) {
        if (fen[i] == '/') {
            row_num++;
            col_num = 0;
            continue;
        }
        if (fen[i] == ' ') {
            cursor = i;
            break;
        }
        if (fen[i] == '1') {
            col_num++;
            continue;
        }
        if (fen[i] == '2') {
            col_num += 2;
            continue;
        }
        if (fen[i] == '3') {
            col_num += 3;
            continue;
        }
        if (fen[i] == '4') {
            col_num += 4;
            continue;
        }
        if (fen[i] == '5') {
            col_num += 5;
            continue;
        }
        if (fen[i] == '6') {
            col_num += 6;
            continue;
        }
        if (fen[i] == '7') {
            col_num += 7;
            continue;
        }
        if (fen[i] == '8') {
            col_num += 8;
            continue;
        }
        this->board[row_num][col_num] = fen[i];
        col_num++;
    }

    cursor++;
    if(fen[cursor] == 'w') {
        this->turn = true;
    } else {
        this->turn = false;
    }

    cursor += 2;
    bool skip = false;

    this->white_kingside_castling = false;
    this->white_queenside_castling = false;
    this->black_kingside_castling = false;
    this->black_queenside_castling = false;

    if(fen[cursor] == '-') {
        skip = true;
        cursor++;
    } else if (fen[cursor] == 'K') {
        this->white_kingside_castling = true;
    } else if (fen[cursor] == 'Q') {
        this->white_queenside_castling = true;
    } else if (fen[cursor] == 'k') {
        this->black_kingside_castling = true;
    } else if (fen[cursor] == 'q') {
        this->black_queenside_castling = true;
    }

    if(!skip) {
        cursor++;
    }

    if(!skip) {
        if (fen[cursor] == 'K') {
            this->white_kingside_castling = true;
        } else if (fen[cursor] == 'Q') {
            this->white_queenside_castling = true;
        } else if (fen[cursor] == 'k') {
            this->black_kingside_castling = true;
        } else if (fen[cursor] == 'q') {
            this->black_queenside_castling = true;
        } else if (fen[cursor] == ' ') {
            skip = true;
        }
    }

    if(!skip) {
        cursor++;
    }

    if(!skip) {
        if (fen[cursor] == 'K') {
            this->white_kingside_castling = true;
        } else if (fen[cursor] == 'Q') {
            this->white_queenside_castling = true;
        } else if (fen[cursor] == 'k') {
            this->black_kingside_castling = true;
        } else if (fen[cursor] == 'q') {
            this->black_queenside_castling = true;
        } else if (fen[cursor] == ' ') {
            skip = true;
        }
    }

    if(!skip) {
        cursor++;
    }

    if(!skip) {
        if (fen[cursor] == 'K') {
            this->white_kingside_castling = true;
        } else if (fen[cursor] == 'Q') {
            this->white_queenside_castling = true;
        } else if (fen[cursor] == 'k') {
            this->black_kingside_castling = true;
        } else if (fen[cursor] == 'q') {
            this->black_queenside_castling = true;
        } else if (fen[cursor] == ' ') {
            skip = true;
        }
    }

    cursor++;
    if(!skip) {
        cursor++;
    }    

    skip = false;

    if(fen[cursor] == '-') {
        cursor++;
    } else {
        this->enPassantCol = 7 - (fen[cursor] - 'a');
        cursor++;
        this->enPassantRow = 7 - (fen[cursor] - '1');
        cursor++;
    }

    cursor++;

    if(fen[cursor] == '-') {
        cursor++;
    } else {
        if(fen[cursor+1] == ' ') {
            this->halfmove_clock = fen[cursor] - '0';
            cursor++;
        } else {
            this->halfmove_clock = (fen[cursor] - '0') * 10 + (fen[cursor+1] - '0');
            cursor += 2;
        }
    }

    cursor++;

    if(fen[cursor] == '-') {
        cursor++;
    } else {
        // one digit num
        if(cursor+1 == fen.length()) {
            this->fullmove_number = fen[cursor] - '0';
        // two digit num
        } else if (cursor+2 == fen.length()) {
            this->fullmove_number = (fen[cursor] - '0') * 10 + (fen[cursor+1] - '0');
        // three digit num
        } else {
            this->fullmove_number = (fen[cursor] - '0') * 100 + (fen[cursor+1] - '0') * 10 + (fen[cursor+2] - '0');
        }
    }
}

Move * Board::convert_move_fen(std::string fen) {
    // should convert something like "e2e4" to a Move struct with from_x = 4, from_y = 1, to_x = 4, to_y = 3
    Move * move = new Move();
    move->from_x = fen[0] - 'a';
    move->from_y = 7 -(fen[1] - '1');
    move->to_x = fen[2] - 'a';
    move->to_y = 7 - (fen[3] - '1');
    if (fen.length() == 5) {
        move->promotion = fen[4];
    }

    return move;
}

std::string Board::get_move_fen(Move * move) {
    std::string fen = "";
    fen += (char)(move->from_x + 'a');
    fen += (char)(7 - move->from_y + '1');
    fen += (char)(move->to_x + 'a');
    fen += (char)(7 - move->to_y + '1');

    if(move->promotion != '.') {
        fen += move->promotion;
    }

    return fen;
}


// assumes that the move is legal does not support castling
char Board::push_move(Move * move) {
    MovC mov = MovC(move->from_x, move->from_y, move->to_x, move->to_y, move->promotion);
    if(mov.from_x < 0 || mov.from_x > 7) {
        std::cout << "push_move Error: " << mov.from_x << " " << mov.from_y << " " << mov.to_x << " " << mov.to_y << std::endl;
        this->print_self();
        return '.';
    }    

    if(this->verbose) {
        std::cout << "before" << std::endl;
        this->print_self();
    }

    // Remove en passant
    this->enPassantRow = -1;
    this->enPassantCol = -1;

    // execute castling move if applicable
    if(!this->turn) {
        if(this->black_queenside_castling) {
            if(mov.from_x == 4 && mov.from_y == 0 && mov.to_x == 2 && mov.to_y == 0) {
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
            if(mov.from_x == 4 && mov.from_y == 0 && mov.to_x == 6 && mov.to_y == 0) {
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
            if(mov.from_x == 4 && mov.from_y == 7 && mov.to_x == 2 && mov.to_y == 7) {
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
            if(mov.from_x == 4 && mov.from_y == 7 && mov.to_x == 6 && mov.to_y == 7) {
                // execute white kingside castle
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
            // kingside rook moves
            if(mov.from_x == 7 && mov.from_y == 0) {
                this->black_kingside_castling = false;
            // king moves
            } else if (mov.from_x == 4 && mov.from_y == 0) {
                this->black_kingside_castling = false;
                this->black_queenside_castling = false;
            }
        }

        if (this->black_queenside_castling) {
            if(mov.from_x == 0 && mov.from_y == 0) {
                this->black_queenside_castling = false;
            } else if (mov.from_x == 4 && mov.from_y == 0) {
                this->black_kingside_castling = false;
                this->black_queenside_castling = false;
            }
        }

        if (mov.to_x == 7 && mov.to_y == 7) {
            this->white_kingside_castling = false;
        } else if (mov.to_x == 0 && mov.to_y == 7) {
            this->white_queenside_castling = false;
        }
    } else {
        if(this->white_kingside_castling) {
            if(mov.from_x == 7 && mov.from_y == 7) {
                this->white_kingside_castling = false;
            } else if (mov.from_x == 4 && mov.from_y == 7) {
                this->white_kingside_castling = false;
                this->white_queenside_castling = false;
            }
        }

        if (this->white_queenside_castling) {
            if(mov.from_x == 0 && mov.from_y == 7) {
                this->white_queenside_castling = false;
            } else if (mov.from_x == 4 && mov.from_y == 7) {
                this->white_kingside_castling = false;
                this->white_queenside_castling = false;
            }
        }

        if (mov.to_x == 7 && mov.to_y == 0) {
            this->black_kingside_castling = false;
        } else if (mov.to_x == 0 && mov.to_y == 0) {
            this->black_queenside_castling = false;
        }
    }

    // Add en passant rights if applicable
    if(!this->turn) {
        // if the piece is a pawn
        if(this->board[mov.from_y][mov.from_x] == 'p') {
            // if the pawn is moving two spaces
            if(mov.from_y == 1 && mov.to_y == 3) {
                this->enPassantCol = mov.from_x;
                this->enPassantRow = 2;
            }
        }
    } else {
        if(this->board[mov.from_y][mov.from_x] == 'P') {
            if(mov.from_y == 6 && mov.to_y == 4) {
                this->enPassantCol = mov.from_x;
                this->enPassantRow = 5;
            }
        }
    }

    this->halfmove_clock++;

    // if the piece is a pawn
    if(this->board[mov.from_y][mov.from_x] == 'p' || this->board[mov.from_y][mov.from_x] == 'P') {
        this->halfmove_clock = 0;
    }

    //if(mov.from_x == 1 && mov.from_y == 1 && mov.to_x == 1 && mov.to_y == 0) {
    //    std::cout << "before:" << std::endl;
    //    this->print_self();
    //}
    char captured_piece = '.';
    // execute move for en passant
    if(mov.to_x != mov.from_x && (this->board[mov.from_y][mov.from_x] == 'P' || this->board[mov.from_y][mov.from_x] == 'p') && this->board[mov.to_y][mov.to_x] == '.') {
        if(this->verbose) {
            std::cout << "Taking with en passant" << std::endl;
            // from piece
            std::cout << "From piece is " << this->board[mov.from_y][mov.from_x] << std::endl;
        }
        //take with en passant
        if(!this->turn) {
            captured_piece = this->board[mov.to_y-1][mov.to_x];
            this->board[mov.to_y-1][mov.to_x] = '.';
        } else {
            captured_piece = this->board[mov.to_y+1][mov.to_x];
            this->board[mov.to_y+1][mov.to_x] = '.';
        }
        this->board[mov.to_y][mov.to_x] = this->board[mov.from_y][mov.from_x];
        this->board[mov.from_y][mov.from_x] = '.';
    } else {
        captured_piece = this->board[mov.to_y][mov.to_x];
        this->board[mov.to_y][mov.to_x] = this->board[mov.from_y][mov.from_x];
        this->board[mov.from_y][mov.from_x] = '.';
    }

    if(mov.promotion != '.') {
        this->board[mov.to_y][mov.to_x] = mov.promotion;
    }

    /*if(mov.from_x == 1 && mov.from_y == 1 && mov.to_x == 1 && mov.to_y == 0) {
        std::cout << "after:" << std::endl;
        this->print_self();
    }*/

    if(!this->turn) {
        this->fullmove_number++;
    }

    this->turn = !this->turn;

    if(this->verbose) {
        std::cout << "after" << std::endl;
        this->print_self();
    }

    if (captured_piece != '.') {
        this->halfmove_clock = 0;
        return captured_piece;
    }

    return '.';
}

char Board::fake_push_move(Move * move) {
    if(move->from_x < 0 || move->from_x > 7) {
        std::cout << "fake_push_move Error: " << move->from_x << " " << move->from_y << " " << move->to_x << " " << move->to_y << std::endl;
        this->print_self();
        return '.';
    }    
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

    char captured_piece = '.';
    // execute move for en passant
    if(move->to_x != move->from_x && (this->board[move->from_y][move->from_x] == 'P' || this->board[move->from_y][move->from_x] == 'p') && this->board[move->to_y][move->to_x] == '.') {
        //take with en passant
        if(!this->turn) {
            captured_piece = this->board[move->to_y-1][move->to_x];
            this->board[move->to_y-1][move->to_x] = '.';
        } else {
            captured_piece = this->board[move->to_y+1][move->to_x];
            this->board[move->to_y+1][move->to_x] = '.';
        }
        this->board[move->to_y][move->to_x] = this->board[move->from_y][move->from_x];
        this->board[move->from_y][move->from_x] = '.';
    } else {
        captured_piece = this->board[move->to_y][move->to_x];
        this->board[move->to_y][move->to_x] = this->board[move->from_y][move->from_x];
        this->board[move->from_y][move->from_x] = '.';
    }

    if(move->promotion != '.') {
        this->board[move->to_y][move->to_x] = move->promotion;
    }

    if (captured_piece != '.') {
        return captured_piece;
    }

    return '.';
}

// for internal use
/*char Board::fake_push_move(Move * move) {
    char captured_piece = this->board[move->to_y][move->to_x];
    this->board[move->to_y][move->to_x] = this->board[move->from_y][move->from_x];
    this->board[move->from_y][move->from_x] = '.';

    if (captured_piece != '.') {
        return captured_piece;
    }

    return '.';
}*/

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
    for (int i = row + 1; i < 8; i++) {
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
    // that are on a higher row than the king
    if (this->turn == false) {
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
    // if we are checking for the white king we should be checking for black pawns
    // that are on a lower row than the king
    } else {
        if (row - 1 >= 0 && col + 1 < 8) {
            if (this->board[row - 1][col + 1] == enemy_pieces[5]) {
                return true;
            }
        }
        if (row - 1 >= 0 && col - 1 >= 0) {
            if (this->board[row - 1][col - 1] == enemy_pieces[5]) {
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

Move * Board::clone_promotion_moves(Move * moves, int from_y, int from_x, int to_y, int to_x) {
    char * promo_pieces;
    if(this->turn) {
        promo_pieces = this->white_pieces;
    } else {
        promo_pieces = this->black_pieces;
    }

    for(int i=0; i<4; i++) {
        moves->from_x = from_x;
        moves->from_y = from_y;
        moves->to_x = to_x;
        moves->to_y = to_y;
        moves->promotion = promo_pieces[i];

        moves->next = new Move;
        moves = moves->next;
    }

    return moves;
}

Move * Board::get_pawn_moves(Move * moves, int row, int col) {
    char * pieces = this->turn ? black_pieces : white_pieces;

    if(this->turn) {
        // check the square right in front of the pawn
        if(board[row-1][col] == '.') {
            if(row-1 == 0) {
                moves = this->clone_promotion_moves(moves, row, col, row-1, col);
            } else {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col;
                moves->to_y = row - 1;
                moves->next = new Move;
                moves = moves->next;
            }
        }

        // check the square two in front of the pawn (if it is the first move)
        if(row == 6 && board[row-2][col] == '.' && board[row-1][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = row - 2;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square to the diagonal left of the pawn
        if(col - 1 >= 0 && (this->is_in_arr(board[row-1][col-1], pieces) || 
        (row-1 == this->enPassantRow && col-1 == this->enPassantCol && this->enPassantRow == 2))) {
            if(row-1 == 0) {
                moves = this->clone_promotion_moves(moves, row, col, row-1, col-1);
            } else {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col - 1;
                moves->to_y = row - 1;
                moves->next = new Move;
                moves = moves->next;
            }
        }

        // check the square to the diagonal right of the pawn
        if(col + 1 < 8 && (this->is_in_arr(board[row-1][col+1], pieces) || 
        (row-1 == this->enPassantRow && col+1 == this->enPassantCol && this->enPassantRow == 2))) {
            if(row-1 == 0) {
                moves = this->clone_promotion_moves(moves, row, col, row-1, col+1);
            } else {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col + 1;
                moves->to_y = row - 1;
                moves->next = new Move;
                moves = moves->next;
            }
        }
    } else {
        // check the square right in front of the pawn
        if(board[row+1][col] == '.') {
            if(row+1 == 7) {
                moves = this->clone_promotion_moves(moves, row, col, row+1, col);
            } else {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col;
                moves->to_y = row + 1;
                moves->next = new Move;
                moves = moves->next;
            }
        }

        // check the square two in front of the pawn (if it is the first move)
        if(row == 1 && board[row+2][col] == '.' && board[row+1][col] == '.') {
            moves->from_x = col;
            moves->from_y = row;
            moves->to_x = col;
            moves->to_y = row + 2;
            moves->next = new Move;
            moves = moves->next;
        }

        // check the square to the diagonal "left" of the pawn
        if(col - 1 >= 0 && (this->is_in_arr(board[row+1][col-1], pieces) || 
        (row+1 == this->enPassantRow && col-1 == this->enPassantCol && this->enPassantRow == 5))) {
            if(row+1 == 7) {
                moves = this->clone_promotion_moves(moves, row, col, row+1, col-1);
            } else {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col - 1;
                moves->to_y = row + 1;
                moves->next = new Move;
                moves = moves->next;
            }
        }

        // check the square to the diagonal "right" of the pawn
        if(col + 1 < 8 && (this->is_in_arr(board[row+1][col+1], pieces) ||
        (row+1 == this->enPassantRow && col+1 == this->enPassantCol && this->enPassantRow == 5))) {
            if(row+1 == 7) {
                moves = this->clone_promotion_moves(moves, row, col, row+1, col+1);
            } else {
                moves->from_x = col;
                moves->from_y = row;
                moves->to_x = col + 1;
                moves->to_y = row + 1;
                moves->next = new Move;
                moves = moves->next;
            }
        }
    }

    return moves;
}

Move * Board::get_bishop_moves(Move * moves, int row, int col) {
    char * pieces = this->turn ? black_pieces : white_pieces;

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

    moves = this->get_bishop_moves(moves, row, col);
    moves = this->get_rook_moves(moves, row, col);

    return moves;
}

Move * Board::get_king_moves(Move * moves, int row, int col) {
    char * pieces = this->turn ? black_pieces : white_pieces;

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
    if (row - 2 >= 0 && col - 1 >= 0 && (this->board[row - 2][col - 1] == '.' || is_in_arr(this->board[row - 2][col - 1], this->turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 1;
        moves->to_y = row - 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row - 2 >= 0 && col + 1 <= 7 && (this->board[row - 2][col + 1] == '.' || is_in_arr(this->board[row - 2][col + 1], this->turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 1;
        moves->to_y = row - 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row - 1 >= 0 && col - 2 >= 0 && (this->board[row - 1][col - 2] == '.' || is_in_arr(this->board[row - 1][col - 2], this->turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 2;
        moves->to_y = row - 1;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row - 1 >= 0 && col + 2 <= 7 && (this->board[row - 1][col + 2] == '.' || is_in_arr(this->board[row - 1][col + 2], this->turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 2;
        moves->to_y = row - 1;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 2 <= 7 && col - 1 >= 0 && (this->board[row + 2][col - 1] == '.' || is_in_arr(this->board[row + 2][col - 1], turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 1;
        moves->to_y = row + 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 2 <= 7 && col + 1 <= 7 && (this->board[row + 2][col + 1] == '.' || is_in_arr(this->board[row + 2][col + 1], this->turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col + 1;
        moves->to_y = row + 2;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 1 <= 7 && col - 2 >= 0 && (this->board[row + 1][col - 2] == '.' || is_in_arr(this->board[row + 1][col - 2], this->turn ? black_pieces : white_pieces))) {
        moves->from_x = col;
        moves->from_y = row;
        moves->to_x = col - 2;
        moves->to_y = row + 1;
        moves->next = new Move;
        moves = moves->next;
    }

    if (row + 1 <= 7 && col + 2 <= 7 && (this->board[row + 1][col + 2] == '.' || is_in_arr(this->board[row + 1][col + 2], this->turn ? black_pieces : white_pieces))) {
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
    std::vector <MovC> movesC;

    char * pieces = this->turn ? black_pieces : white_pieces;
    
    // check up
    for (int i = row - 1; i >= 0; i--) {
        if (this->board[i][col] == '.') {
            movesC.push_back(MovC(col, row, col, i));
        } else if ( is_in_arr(this->board[i][col], pieces)) {
            movesC.push_back(MovC(col, row, col, i));
            break;
        } else {
            break;
        }
    }

    // check down
    for (int i = row + 1; i < 8; i++) {
        if (this->board[i][col] == '.') {
            movesC.push_back(MovC(col, row, col, i));
        } else if (is_in_arr(this->board[i][col], pieces)) {
            movesC.push_back(MovC(col, row, col, i));
            break;
        } else {
            break;
        }
    }

    // check left
    for (int i = col - 1; i >= 0; i--) {
        if (this->board[row][i] == '.') {
            movesC.push_back(MovC(col, row, i, row));
        } else if (is_in_arr(this->board[row][i], pieces)) {
            movesC.push_back(MovC(col, row, i, row));
            break;
        } else {
            break;
        }
    }

    // check right
    for (int i = col + 1; i < 8; i++) {
        if (this->board[row][i] == '.') {
            movesC.push_back(MovC(col, row, i, row));
        } else if (is_in_arr(this->board[row][i], pieces)) {
            movesC.push_back(MovC(col, row, i, row));
            break;
        } else {
            break;
        }
    }

    return convert_vector_to_linked_list(movesC, moves);
}

Move * Board::convert_vector_to_linked_list(std::vector<MovC> movesC, Move * moves) {
    for (int i = 0; i < movesC.size(); i++) {
        moves->from_x = movesC[i].from_x;
        moves->from_y = movesC[i].from_y;
        moves->to_x = movesC[i].to_x;
        moves->to_y = movesC[i].to_y;
        moves->next = new Move;
        moves = moves->next;
    }

    return moves;
}

// we don't deal with checking castling rights here, we assume
// that the Board::castling_rights booleans are correct and up to date
// those booleans get changed in push_move.
Move * Board::get_castling_moves(Move * moves) {
    std::vector <MovC> movesC;
    
    if(!this->turn) {
        // check for black castling
        if(this->black_kingside_castling && 
          !this->is_king_in_check(0, 5) && !this->is_king_in_check(0,4)
          && this->board[0][5] == '.' && this->board[0][6] == '.') {
            movesC.push_back(MovC(4, 0, 6, 0));
        }

        if(this->black_queenside_castling && 
          !this->is_king_in_check(0,3) && !this->is_king_in_check(0,4)
          && this->board[0][1] == '.' && this->board[0][2] == '.' && this->board[0][3] == '.') {
            movesC.push_back(MovC(4, 0, 2, 0));
        }
    } else {
        // check for white castling
        if(this->white_kingside_castling && 
        !this->is_king_in_check(7, 5) && !this->is_king_in_check(7,4)
        && this->board[7][5] == '.' && this->board[7][6] == '.') {
            movesC.push_back(MovC(4, 7, 6, 7));
        }

        if(this->white_queenside_castling && 
        !this->is_king_in_check(7, 3) && !this->is_king_in_check(7,4)
        && this->board[7][1] == '.' && this->board[7][2] == '.' && this->board[7][3] == '.') {
            movesC.push_back(MovC(4, 7, 2, 7));
        }
    }

    return convert_vector_to_linked_list(movesC, moves);
}

void Board::free_move_list(Move * moves) {
    Move * temp;
    while (moves != nullptr) {
        temp = moves;
        moves = moves->next;
        if(temp->from_x >= 0 and temp->from_x <=7) {
            delete temp;
        }
    }
}

Move * Board::get_pseudo_legal_moves() {
    //std::cout << "Getting pseudo legal moves..." << std::endl;
    
    Move * moves = new Move;
    Move * list_end = moves;

    char piece;
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            piece = this->board[row][col];

            if(piece == '.') {
                continue;
            }

            if (this->turn == false) {
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
                } else if(piece == 'p') {
                    list_end = this->get_pawn_moves(list_end, row, col);
                }
            } else if (this->turn == true) {
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
                } else if(piece == 'P') {
                    list_end = this->get_pawn_moves(list_end, row, col);
                }
            }
        }
    }

    this->get_castling_moves(list_end);

    if(moves == list_end) {
        // no moves were found
        delete moves;
        return nullptr;
    }

    return moves;
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
    if(move->from_x == -1) {
        return false;
    }

    Board * dummy_board = new Board(*this);

    dummy_board->push_move(move);
    dummy_board->turn = !dummy_board->turn;

    int * king_pos = dummy_board->get_king_pos();

    int row = king_pos[0];
    int col = king_pos[1];

    bool result = dummy_board->is_king_in_check(row, col);

    delete dummy_board;
    delete [] king_pos;

    return !result;
}

Move * Board::get_legal_moves(bool v) {
    Move * pseudo_legal_moves = this->get_pseudo_legal_moves();

    if(pseudo_legal_moves == nullptr) {
        // there are no legal moves in this position
        return nullptr;
    }

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
    int legal_moves = 0;

    Move * temp;

    while (pseudo_legal_moves != nullptr) {
        temp = pseudo_legal_moves;
        pseudo_legal_moves = pseudo_legal_moves->next;

        if (this->is_legal_move(temp)) {
            list_end->from_x = temp->from_x;
            list_end->from_y = temp->from_y;
            list_end->to_x = temp->to_x;
            list_end->to_y = temp->to_y;
            list_end->promotion = temp->promotion;
            list_end->next = new Move;
            prev_end = list_end;
            list_end = list_end->next;
            legal_moves++;
        }
        delete temp;
    }

    if(legal_moves == 0) {
        delete moves;
        return nullptr;
    }

    if (prev_end != nullptr) {
        prev_end->next = nullptr;
    }

    return moves;
}

void Board::print_self() {
    std::cout << "Printing this->board..." << std::endl;
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            std::cout << this->board[row][col] << " ";
        }
        std::cout << 8 - row << std::endl;
    }

    std::cout << "a b c d e f g h" << std::endl;
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