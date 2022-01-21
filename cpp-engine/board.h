#ifndef BOARD_H
#define BOARD_H

#include <iostream>
#include <string>

struct Move {
    int from_x = -1;
    int from_y = -1;
    int to_x = -1;
    int to_y = -1;

    char promotion = '.';
    Move * next = nullptr;
};

class Board {
    private:
    char board[8][8];
    char * black_pieces;
    char * white_pieces;
    char black_king;
    char white_king;
    bool white_kingside_castling;
    bool white_queenside_castling;
    bool black_kingside_castling;
    bool black_queenside_castling;
    int enPassantCol;
    int enPassantRow;

    bool check_on_board();
    void pull_move(Move *, int);
    bool is_legal_move(Move * move);
    bool is_in_arr(char piece, char arr[]);
    Move * get_rook_moves(Move *, int, int);
    Move * get_king_moves(Move *, int, int);
    Move * get_bishop_moves(Move *, int, int);
    Move * get_queen_moves(Move *, int, int);
    Move * get_knight_moves(Move *, int, int);
    Move * get_pawn_moves(Move *, int, int);
    Move * get_castling_moves(Move *);
    Move * clone_promotion_moves(Move *, int, int, int, int);
    char fake_push_move(Move * move);
    int min(int, int);
    int max(int, int);

    public:
    Board();
    ~Board();
    Board(const Board &);
    int * get_king_pos();
    void print_board_metadata();
    void import_board_fen(std::string);
    Move * convert_move_fen(std::string);
    std::string get_move_fen(Move *);
    void free_move_list(Move * moves);
    void clear_board();
    void set_piece(int row, int col, char piece);
    char get_piece(int row, int col);
    void print_self();
    char push_move(Move * move);
    Move * get_pseudo_legal_moves();
    Move * get_legal_moves();
    bool is_king_in_check(int, int);
    // half moves since last capture or pawn move
    int halfmove_clock;
    // full moves since game start
    int fullmove_number;
    bool turn = true;
    void free_piece_lists();

};

#endif