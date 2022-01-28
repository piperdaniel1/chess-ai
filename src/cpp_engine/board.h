#ifndef BOARD_H
#define BOARD_H

#include <iostream>
#include <vector>
#include <string>

struct Move {
    int from_x = -1;
    int from_y = -1;
    int to_x = -1;
    int to_y = -1;

    char promotion = '.';
    Move * next = nullptr;
};

class MovC {
    public:
    MovC();
    MovC(int from_x, int from_y, int to_x, int to_y);
    MovC(int from_x, int from_y, int to_x, int to_y, char promotion);
    MovC(std::string);
    MovC(Move);
    ~MovC();
    MovC(const MovC&);
    MovC& operator=(const MovC&);
    std::string get_fen();
    Move get_old_move();

    int from_x;
    int from_y;
    int to_x;
    int to_y;

    char promotion;
    MovC * next;
};

class Board {
    private:
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
    bool is_legal_move(MovC mov);
    bool is_in_arr(char piece, char arr[]);
    void get_rook_moves(std::vector<MovC>&, int, int);
    void get_king_moves(std::vector<MovC>&, int, int);
    void get_bishop_moves(std::vector<MovC>&, int, int);
    void get_queen_moves(std::vector<MovC>&, int, int);
    void get_knight_moves(std::vector<MovC>&, int, int);
    void get_pawn_moves(std::vector<MovC>&, int, int);
    void get_castling_moves(std::vector<MovC>&);
    void clone_promotion_moves(std::vector <MovC> & moves, int, int, int, int);
    Move * convert_vector_to_linked_list(std::vector<MovC>, Move *);
    char fake_push_move(Move * move);
    int min(int, int);
    int max(int, int);

    public:
    char board[8][8];
    Board();
    ~Board();
    Board(const Board &);
    int * get_king_pos();
    void print_board_metadata();
    void import_board_fen(std::string);
    void clear_board();
    void set_piece(int row, int col, char piece);
    char get_piece(int row, int col);
    void print_self();
    char push_movC(MovC mov);
    void get_pseudo_legal_moves(std::vector<MovC>&);
    void get_legal_movC(std::vector<MovC>&);
    bool is_king_in_check(int, int);
    // half moves since last capture or pawn move
    int halfmove_clock;
    // full moves since game start
    int fullmove_number;
    bool turn = true;
    void free_piece_lists();
    bool verbose = false;

};

#endif