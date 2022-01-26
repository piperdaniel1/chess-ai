#ifndef TT_TABLE
#define TT_TABLE

#include <iostream>
#include <time.h>
#include <random>

struct Entry {
    int depth;
    int eval;
    int key;
};

class TT_Table {
    private:
    Entry * table;
    std::uint64_t hash_table[8][8][6];

    public:
    TT_Table();
    std::uint64_t get_hash(char board[8][8]);
};

#endif