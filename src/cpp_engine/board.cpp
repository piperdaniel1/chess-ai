#include "board.h"

Square::Square() {
    row = -1;
    col = -1;
}

Square::Square(int row, int col) {
    this->row = row;
    this->col = col;
}

void MovC::init_cues() {
    this->whiteKingSideCue = true;
    this->whiteQueenSideCue = true;
    this->blackKingSideCue = true;
    this->whiteQueenSideCue = true;

    this->is_castling = false;
    this->is_enpassant = false;
    this->captured_piece = '.';
    this->halfMoveCount = 0;
    this->fullMoveCount = 0;
}
MovC::MovC() {
    this->from_x = 0;
    this->from_y = 0;
    this->to_x = 0;
    this->to_y = 0;

    this->promotion = '.';
    this->next = nullptr;

    this->init_cues();
}
MovC::MovC(int from_x, int from_y, int to_x, int to_y) {
    this->from_x = from_x;
    this->from_y = from_y;
    this->to_x = to_x;
    this->to_y = to_y;

    this->promotion = '.';
    this->next = nullptr;

    this->init_cues();
}
MovC::MovC(int from_x, int from_y, int to_x, int to_y, char promotion) {
    this->from_x = from_x;
    this->from_y = from_y;
    this->to_x = to_x;
    this->to_y = to_y;

    this->promotion = promotion;
    this->next = nullptr;

    this->init_cues();
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

    this->init_cues();
}
MovC::MovC(Move move) {
    this->from_x = move.from_x;
    this->from_y = move.from_y;
    this->to_x = move.to_x;
    this->to_y = move.to_y;

    this->promotion = move.promotion;
    this->next = nullptr;

    this->init_cues();
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

    this->enPassantCue = other.enPassantCue;
    this->whiteKingSideCue = other.whiteKingSideCue;
    this->whiteQueenSideCue = other.whiteQueenSideCue;
    this->blackKingSideCue = other.blackKingSideCue;
    this->blackQueenSideCue = other.blackQueenSideCue;

    this->is_castling = other.is_castling;
    this->is_enpassant = other.is_enpassant;
    this->captured_piece = other.captured_piece;
    this->halfMoveCount = other.halfMoveCount;
    this->fullMoveCount = other.fullMoveCount;
}
MovC& MovC::operator=(const MovC& other) {
    this->from_x = other.from_x;
    this->from_y = other.from_y;
    this->to_x = other.to_x;
    this->to_y = other.to_y;

    this->promotion = other.promotion;
    this->next = other.next;

    this->enPassantCue = other.enPassantCue;
    this->whiteKingSideCue = other.whiteKingSideCue;
    this->whiteQueenSideCue = other.whiteQueenSideCue;
    this->blackKingSideCue = other.blackKingSideCue;
    this->blackQueenSideCue = other.blackQueenSideCue;

    this->is_castling = other.is_castling;
    this->is_enpassant = other.is_enpassant;
    this->captured_piece = other.captured_piece;
    this->halfMoveCount = other.halfMoveCount;
    this->fullMoveCount = other.fullMoveCount;
    
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
Move MovC::get_old_move() {
    Move move;
    move.from_x = this->from_x;
    move.from_y = this->from_y;
    move.to_x = this->to_x;
    move.to_y = this->to_y;
    move.promotion = this->promotion;

    return move;
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

void Board::pull_movC(MovC& mov) {
    // * 1. If the mov is not a valid move, print an error message.
    if(mov.from_x < 0 || mov.from_x > 7) {
        std::cout << "pull_movC Error: from_x out of bounds" <<  std::endl;
        std::cout << "MovC: " << mov.from_x << " " << mov.from_y << " " << mov.to_x << " " << mov.to_y << std::endl;
        return;
    }

    // * 2. Set enPassantRow and enPassantCol to the enPassant square
    this->enPassantCol = mov.enPassantCue.col;
    this->enPassantRow = mov.enPassantCue.row;

    // * 3. Set the castling flags to the values stored inside the mov.
    this->white_kingside_castling = mov.whiteKingSideCue;
    this->white_queenside_castling = mov.whiteQueenSideCue;
    this->black_kingside_castling = mov.blackKingSideCue;
    this->black_queenside_castling = mov.blackQueenSideCue;

    // * 4. Set the move counts to the values stored inside the mov.
    this->halfmove_clock = mov.halfMoveCount;
    this->fullmove_number = mov.fullMoveCount;

    // * 5. Reverse whose turn it is.
    this->turn = !this->turn;

    // * 6. Check if the castling flag is true.
    // *    - If so, reverse the move as it if is a castling move and exit.
    if(mov.is_castling) {
        // This means that black is castling
        if(mov.from_y == 0) {
            // This means that it is a queenside castling move
            if(mov.to_x == 2) {
                // 0 1 2 3 4
                // . . k r . . . .
                // to
                // 0 1 2 3 4
                // r . . . k . . .
                this->board[mov.from_y][0] = 'r';
                this->board[mov.from_y][4] = 'k';
                this->board[mov.from_y][3] = '.';
                this->board[mov.from_y][2] = '.';
            // This means that it is a kingside castling move
            } else if (mov.to_x == 6) {
                // 0 1 2 3 4 5 6 7
                // . . . . . r k .
                // to
                // 0 1 2 3 4 5 6 7
                // . . . . k . . r
                this->board[mov.from_y][7] = 'r';
                this->board[mov.from_y][4] = 'k';
                this->board[mov.from_y][5] = '.';
                this->board[mov.from_y][6] = '.';
            }
        // This means that white is castling
        } else {
            // This means that it is a queenside castling move
            if(mov.to_x == 2) {
                // 0 1 2 3 4
                // . . k r . . . .
                // to
                // 0 1 2 3 4
                // r . . . k . . .
                this->board[mov.from_y][0] = 'r';
                this->board[mov.from_y][4] = 'k';
                this->board[mov.from_y][3] = '.';
                this->board[mov.from_y][2] = '.';
            // This means that it is a kingside castling move
            } else if (mov.to_x == 6) {
                // 0 1 2 3 4 5 6 7
                // . . . . . r k .
                // to
                // 0 1 2 3 4 5 6 7
                // . . . . k . . r
                this->board[mov.from_y][7] = 'r';
                this->board[mov.from_y][4] = 'k';
                this->board[mov.from_y][5] = '.';
                this->board[mov.from_y][6] = '.';
            }
        }
        return;
    // * 7. Check if the en passant flag is true.
    // *    - If so, reverse the move as if it is an en passant move and exit.
    } else if (mov.is_enpassant) {
        // black is taking white with en passant
        // . . P p . . . . 4
        // . . ! . . . . . 5
        // . . . . . . . . 6
        // . . . . K . . . 7
        // 0 1 2 3 4 5 6 7
        if(mov.to_y == 5) {
            // Swap the from and to squares
            this->board[mov.from_y][mov.from_x] = this->board[mov.to_y][mov.to_x];
            this->board[mov.to_y][mov.to_x] = '.';

            // Replace the pawn that was taken
            this->board[4][mov.to_x] = 'P';
        // white is taking black with en passant
        // . . . . k . . . 0
        // . . . . . . . . 1
        // . . ! . . . . . 2
        // . . p P . . . . 3
        // 0 1 2 3 4 5 6 7
        } else if (mov.to_y == 2) {
            // Swap the from and to squares
            this->board[mov.from_y][mov.from_x] = this->board[mov.to_y][mov.to_x];
            this->board[mov.to_y][mov.to_x] = '.';

            // Replace the pawn that was taken
            this->board[3][mov.to_x] = 'p';
        }

    } else {
        
    }

}

// Design for void Board::pull_movC(MovC mov);
/*
 * 1. If the mov is not a valid move, print an error message.
 * 2. Set enPassantRow and enPassantCol to the enPassant square
 *    that is stored inside the mov.
 * 3. Set the castling flags to the values stored inside the mov.
 * 4. Set the move counts to the values stored inside the mov.
 * 5. Reverse whose turn it is.
 * 6. Check if the castling flag is true.
 *    - If so, reverse the move as it if is a castling move and exit.
 * 7. Check if the en passant flag is true.
 *    - If so, reverse the move as if it is an en passant move and exit.
 * 8. Reverse the move normally, remembering to respawn the captured piece.
 * 9. If the promotion field is not blank revert the piece that moved back to
 *    a pawn of the correct color. This can be determined by looking at the
 *    to_y of the move.
 * 10. Exit.
 */

void Board::push_movC(MovC& mov) {
    if(mov.from_x < 0 || mov.from_x > 7) {
        std::cout << "push_movC Error: " << mov.from_x << " " << mov.from_y << " " << mov.to_x << " " << mov.to_y << std::endl;
        this->print_self();
        return;
    }

    // Set cues in the mov to be used in pull_movC later.
    mov.blackKingSideCue = this->black_kingside_castling;
    mov.blackQueenSideCue = this->black_queenside_castling;
    mov.whiteKingSideCue = this->white_kingside_castling;
    mov.whiteQueenSideCue = this->white_queenside_castling;
    mov.enPassantCue = Square(this->enPassantRow, this->enPassantCol);
    mov.halfMoveCount = this->halfmove_clock;
    mov.fullMoveCount = this->fullmove_number;

    /*
     * The current enPassant settings will have to be stored into the mov at this point,
     * for use later in PullMove so we can revert to the correct state.
     * 
     * The current castling flags will also have to be stored into the mov at this point,
     * for use later in PullMove so we can revert them to the correct state.
     * 
     * The current halfMoveCount and fullMoveCount will also have to be stored into the mov
     * at this point, for use later in PullMove.
     */

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
                /*
                 * We need to set the castling field in mov to true at this point so we
                 * know how to revert the move.
                 */
                mov.is_castling = true;
                return;
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
                /*
                 * We need to set the castling field in mov to true at this point so we
                 * know how to revert the move.
                 */
                mov.is_castling = true;
                return;
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
                /*
                 * We need to set the castling field in mov to true at this point so we
                 * know how to revert the move.
                 */
                mov.is_castling = true;
                return;
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
                /*
                 * We need to set the castling field in mov to true at this point so we
                 * know how to revert the move.
                 */
                mov.is_castling = true;
                return;
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

    if(this->board[mov.from_y][mov.from_x] == 'p' || this->board[mov.from_y][mov.from_x] == 'P') {
        this->halfmove_clock = 0;
    }

    char captured_piece = '.';

    if(mov.to_x != mov.from_x && (this->board[mov.from_y][mov.from_x] == 'P' || this->board[mov.from_y][mov.from_x] == 'p') && this->board[mov.to_y][mov.to_x] == '.') {
        /*
         * The en passant flag will have to be set
         * inside the mov at this point for later use
         * inside Pull_Move.
         */
        mov.is_enpassant = true;
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

    if(!this->turn) {
        this->fullmove_number++;
    }

    this->turn = !this->turn;

    if (captured_piece != '.') {
        this->halfmove_clock = 0;
        mov.captured_piece = captured_piece;
        /*
         * The captured piece will have to be stored in the mov at this point
         * for use in the pull_move function later.
         */
    }

    return;
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

void Board::clone_promotion_moves(std::vector <MovC> & movesC, int from_y, int from_x, int to_y, int to_x) {
    char * promo_pieces;
    if(this->turn) {
        promo_pieces = this->white_pieces;
    } else {
        promo_pieces = this->black_pieces;
    }

    for(int i=0; i<4; i++) {
        movesC.push_back(MovC(from_x, from_y, to_x, to_y, promo_pieces[i]));
    }
}

void Board::get_pawn_moves(std::vector<MovC>& movesC, int row, int col) {
    char * pieces = this->turn ? black_pieces : white_pieces;

    if(this->turn) {
        // check the square right in front of the pawn
        if(board[row-1][col] == '.') {
            if(row-1 == 0) {
                this->clone_promotion_moves(movesC, row, col, row-1, col);
            } else {
                movesC.push_back(MovC(col, row, col, row-1));
            }
        }

        // check the square two in front of the pawn (if it is the first move)
        if(row == 6 && board[row-2][col] == '.' && board[row-1][col] == '.') {
            movesC.push_back(MovC(col, row, col, row-2));
        }

        // check the square to the diagonal left of the pawn
        if(col - 1 >= 0 && (this->is_in_arr(board[row-1][col-1], pieces) || 
        (row-1 == this->enPassantRow && col-1 == this->enPassantCol && this->enPassantRow == 2))) {
            if(row-1 == 0) {
                this->clone_promotion_moves(movesC, row, col, row-1, col-1);
            } else {
                movesC.push_back(MovC(col, row, col-1, row-1));
            }
        }

        // check the square to the diagonal right of the pawn
        if(col + 1 < 8 && (this->is_in_arr(board[row-1][col+1], pieces) || 
        (row-1 == this->enPassantRow && col+1 == this->enPassantCol && this->enPassantRow == 2))) {
            if(row-1 == 0) {
                this->clone_promotion_moves(movesC, row, col, row-1, col+1);
            } else {
                movesC.push_back(MovC(col, row, col+1, row-1));
            }
        }
    } else {
        // check the square right in front of the pawn
        if(board[row+1][col] == '.') {
            if(row+1 == 7) {
                this->clone_promotion_moves(movesC, row, col, row+1, col);
            } else {
                movesC.push_back(MovC(col, row, col, row+1));
            }
        }

        // check the square two in front of the pawn (if it is the first move)
        if(row == 1 && board[row+2][col] == '.' && board[row+1][col] == '.') {
            movesC.push_back(MovC(col, row, col, row+2));
        }

        // check the square to the diagonal "left" of the pawn
        if(col - 1 >= 0 && (this->is_in_arr(board[row+1][col-1], pieces) || 
        (row+1 == this->enPassantRow && col-1 == this->enPassantCol && this->enPassantRow == 5))) {
            if(row+1 == 7) {
                this->clone_promotion_moves(movesC, row, col, row+1, col-1);
            } else {
                movesC.push_back(MovC(col, row, col-1, row+1));
            }
        }

        // check the square to the diagonal "right" of the pawn
        if(col + 1 < 8 && (this->is_in_arr(board[row+1][col+1], pieces) ||
        (row+1 == this->enPassantRow && col+1 == this->enPassantCol && this->enPassantRow == 5))) {
            if(row+1 == 7) {
                this->clone_promotion_moves(movesC, row, col, row+1, col+1);
            } else {
                movesC.push_back(MovC(col, row, col+1, row+1));
            }
        }
    }
}

void Board::get_bishop_moves(std::vector<MovC>& movesC, int row, int col) {
    char * pieces = this->turn ? black_pieces : white_pieces;

    // check towards the top left
    for (int i=1; i<=min(row, col); i++) {
        if (this->board[row-i][col-i] == '.') {
            movesC.push_back(MovC(col, row, col-i, row-i));
        } else if (is_in_arr(this->board[row-i][col-i], pieces)) {
            movesC.push_back(MovC(col, row, col-i, row-i));
            break;
        } else {
            break;
        }
    }

    // check towards the top right
    for (int i=1; i<=min(row, 7-col); i++) {
        if (this->board[row-i][col+i] == '.') {
            movesC.push_back(MovC(col, row, col+i, row-i));
        } else if (is_in_arr(this->board[row-i][col+i], pieces)) {
            movesC.push_back(MovC(col, row, col+i, row-i));
            break;
        } else {
            break;
        }
    }

    // check towards the bottom left
    for (int i=1; i<=min(7-row, col); i++) {
        if (this->board[row+i][col-i] == '.') {
            movesC.push_back(MovC(col, row, col-i, row+i));
        } else if (is_in_arr(this->board[row+i][col-i], pieces)) {
            movesC.push_back(MovC(col, row, col-i, row+i));
            break;
        } else {
            break;
        }
    }

    // check towards the bottom right
    for (int i=1; i<=min(7-row, 7-col); i++) {
        if (this->board[row+i][col+i] == '.') {
            movesC.push_back(MovC(col, row, col+i, row+i));
        } else if (is_in_arr(this->board[row+i][col+i], pieces)) {
            movesC.push_back(MovC(col, row, col+i, row+i));
            break;
        } else {
            break;
        }
    }
}

void Board::get_queen_moves(std::vector<MovC>& movesC, int row, int col) {
    this->get_bishop_moves(movesC, row, col);
    this->get_rook_moves(movesC, row, col);
}

void Board::get_king_moves(std::vector<MovC>& movesC, int row, int col) {
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
                movesC.push_back(MovC(col, row, col + j, row + i));
            }
        }
    }
}

void Board::get_knight_moves(std::vector<MovC>& movesC, int row, int col) {
    if (row - 2 >= 0 && col - 1 >= 0 && (this->board[row - 2][col - 1] == '.' || is_in_arr(this->board[row - 2][col - 1], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col - 1, row - 2));
    }

    if (row - 2 >= 0 && col + 1 <= 7 && (this->board[row - 2][col + 1] == '.' || is_in_arr(this->board[row - 2][col + 1], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col + 1, row - 2));
    }

    if (row - 1 >= 0 && col - 2 >= 0 && (this->board[row - 1][col - 2] == '.' || is_in_arr(this->board[row - 1][col - 2], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col - 2, row - 1));
    }

    if (row - 1 >= 0 && col + 2 <= 7 && (this->board[row - 1][col + 2] == '.' || is_in_arr(this->board[row - 1][col + 2], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col + 2, row - 1));
    }

    if (row + 2 <= 7 && col - 1 >= 0 && (this->board[row + 2][col - 1] == '.' || is_in_arr(this->board[row + 2][col - 1], turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col - 1, row + 2));
    }

    if (row + 2 <= 7 && col + 1 <= 7 && (this->board[row + 2][col + 1] == '.' || is_in_arr(this->board[row + 2][col + 1], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col + 1, row + 2));
    }

    if (row + 1 <= 7 && col - 2 >= 0 && (this->board[row + 1][col - 2] == '.' || is_in_arr(this->board[row + 1][col - 2], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col - 2, row + 1));
    }

    if (row + 1 <= 7 && col + 2 <= 7 && (this->board[row + 1][col + 2] == '.' || is_in_arr(this->board[row + 1][col + 2], this->turn ? black_pieces : white_pieces))) {
        movesC.push_back(MovC(col, row, col + 2, row + 1));
    }
}

void Board::get_rook_moves(std::vector<MovC>& movesC, int row, int col) {
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
}

// we don't deal with checking castling rights here, we assume
// that the Board::castling_rights booleans are correct and up to date
// those booleans get changed in push_move.
void Board::get_castling_moves(std::vector<MovC>& movesC) {
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
}

void Board::get_pseudo_legal_moves(std::vector<MovC>& movesC) {    
    char piece;
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            piece = this->board[row][col];

            if(piece == '.') {
                continue;
            }

            if (this->turn == false) {
                if(piece == 'r') {
                    this->get_rook_moves(movesC, row, col);
                } else if(piece == 'k') {
                    this->get_king_moves(movesC, row, col);
                } else if(piece == 'b') {
                    this->get_bishop_moves(movesC, row, col);
                } else if(piece == 'q') {
                    this->get_queen_moves(movesC, row, col);
                } else if(piece == 'n') {
                    this->get_knight_moves(movesC, row, col);
                } else if(piece == 'p') {
                    this->get_pawn_moves(movesC, row, col);
                }
            } else if (this->turn == true) {
                if(piece == 'R') {
                    this->get_rook_moves(movesC, row, col);
                } else if(piece == 'K') {
                    this->get_king_moves(movesC, row, col);
                } else if(piece == 'B') {
                    this->get_bishop_moves(movesC, row, col);
                } else if(piece == 'Q') {
                    this->get_queen_moves(movesC, row, col);
                } else if(piece == 'N') {
                    this->get_knight_moves(movesC, row, col);
                } else if(piece == 'P') {
                    this->get_pawn_moves(movesC, row, col);
                }
            }
        }
    }

    this->get_castling_moves(movesC);
}

Square Board::get_king_pos() {
    Square pos;
    for(int i=0; i<8; i++) {
        for(int j=0; j<8; j++) {
            if(this->board[i][j] == 'k' and this->turn == false) {
                pos.row = i;
                pos.col = j;
                return pos;
            } else if (this->board[i][j] == 'K' and this->turn == true) {
                pos.row = i;
                pos.col = j;
                return pos;
            }
        }
    }

    return pos;
}

bool Board::is_legal_move(MovC mov) {
    if(mov.from_x == -1) {
        return false;
    }

    Board * dummy_board = new Board(*this);

    dummy_board->push_movC(mov);
    dummy_board->turn = !dummy_board->turn;

    Square king_pos = dummy_board->get_king_pos();
    bool result = dummy_board->is_king_in_check(king_pos.row, king_pos.col);

    delete dummy_board;

    return !result;
}

void Board::get_legal_movC(std::vector<MovC>& legal_moves) {
    std::vector <MovC> movesC;
    this->get_pseudo_legal_moves(movesC);

    for(MovC mov : movesC) {
        if (this->is_legal_move(mov)) {
            legal_moves.push_back(mov);    
        }
    }
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