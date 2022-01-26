#ifndef TT_TABLE
#define TT_TABLE

#include <iostream>
#include <time.h>
#include <random>

struct Entry {
    int depth;
    int eval;
    int hash;
};

class TT_Table {
    private:
    int size =  20000000;
    Entry null_entry;
    Entry table[20000000];
    std::uint64_t hash_table[8][8][6];
    int get_corresponding_num(char piece);

    public:
    TT_Table();
    std::uint64_t get_hash(char board[8][8]);
    Entry query_hash(std::uint64_t hash);
    void TT_Table::store_hash(Entry entry);
};

#endif