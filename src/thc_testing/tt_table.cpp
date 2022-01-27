//this file will contain the transposition table
#include "tt_table.h"

TT_Table::TT_Table() {
    std::cout << "Initializing transposition table..." << std::endl;
    this->table = new Entry[this->size];
    srand(time(NULL));
    std::random_device rd;
    this->null_entry.depth = 0;
    this->null_entry.eval = 0;
    this->null_entry.hash = 0;

    std::default_random_engine generator(rd());

    std::uniform_int_distribution<long long unsigned> distribution(0,0xFFFFFFFFFFFFFFFF);

    for(int i=0; i<64; i++) {
        for(int k=0; k<12; k++) {
            this->hash_table[i][k] = distribution(generator);
        }
    }

    for(int i=0; i<this->size; i++) {
        this->table[i].depth = 0;
        this->table[i].eval = 0;
        this->table[i].hash = 0;
    }

    std::cout << "Hash: " << this->table[5000].hash << std::endl;
}

TT_Table::~TT_Table() {
    delete [] this->table;
}

int TT_Table::get_corresponding_num(char p) {
    if(p == 'p') {
        return 0;
    } else if (p == 'r') {
        return 1;
    } else if (p == 'n') {
        return 2;
    } else if (p == 'b') {
        return 3;
    } else if (p == 'q') {
        return 4;
    } else if (p == 'k') {
        return 5;
    } else if (p == 'P') {
        return 6;
    } else if (p == 'R') {
        return 7;
    } else if (p == 'N') {
        return 8;
    } else if (p == 'B') {
        return 9;
    } else if (p == 'Q') {
        return 10;
    } else if (p == 'K') {
        return 11;
    } else {
        return -1;
    }
}

std::uint64_t TT_Table::get_hash(char board[64]) {
    std::uint64_t hash = 0;
    int piece = 0;
    for(int i=0; i<64; i++) {
        if(board[i] == ' ') {
            continue;
        }

        piece = this->get_corresponding_num(board[i]);
        hash ^= this->hash_table[i][piece];
    }

    return hash;
}

Entry TT_Table::query_board(char board[64]) {
    std::uint64_t hash = this->get_hash(board);

    int i = hash % this->size;
    if(this->table[i].hash == hash) {
        return this->table[i];
    }

    return this->null_entry;
}

void TT_Table::store_board(char board[64], int depth, int eval) {
    std::uint64_t hash = this->get_hash(board);
    int i = hash % this->size;
    this->table[i].depth = depth;
    this->table[i].eval = eval;
    this->table[i].hash = hash;
}