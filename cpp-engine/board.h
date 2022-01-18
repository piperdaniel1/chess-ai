#ifndef BOARD_H
#define BOARD_H

#include <iostream>

struct Move {
    int from_x = -1;
    int from_y = -1;
    int to_x = -1;
    int to_y = -1;

    Move * next = nullptr;
};

class Board {
    private:
    char board[8][8] = {{'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'},
                        {'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'},
                        {'.', '.', '.', '.', '.', '.', '.', '.'},
                        {'.', '.', '.', '.', '.', '.', '.', '.'},
                        {'.', '.', '.', '.', '.', '.', '.', '.'},
                        {'.', '.', '.', '.', '.', '.', '.', '.'},
                        {'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'},
                        {'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'}};

    char * black_pieces = new char[6];
    char * white_pieces = new char[6];

    bool turn = true;

    bool check_on_board();
    void pull_move(Move * move, int captured_piece = NULL);
    bool is_legal_move(Move * move);
    Move * get_pseudo_legal_moves();
    bool is_in_arr(char piece, char arr[]);
    Move * get_rook_moves(Move *, int, int);
    Move * get_king_moves(Move *, int, int);

    public:
    Board();
    ~Board();
    void free_move_list(Move * moves);
    void clear_board();
    void set_piece(int row, int col, char piece);
    void print_self();
    void push_move(Move * move);
    Move * get_legal_moves();
    bool is_king_in_check(int, int);
};

#endif