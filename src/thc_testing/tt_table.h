#ifndef TT_TABLE
#define TT_TABLE

#include <iostream>
#include <time.h>
#include <random>

struct Entry {
    int depth;
    int eval;
    std::uint64_t hash;
};

class TT_Table {
    private:
    int size =  20000000;
    Entry null_entry;
    Entry * table;
    std::uint64_t hash_table[64][6];
    int get_corresponding_num(char piece);

    public:
    TT_Table();
    ~TT_Table();
    std::uint64_t get_hash(char board[64]);
    Entry query_board(char board[64]);
    void store_board(char board[64], int depth, int eval);
};

#endif