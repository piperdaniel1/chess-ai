//this file will contain the transposition table
#include "tt_table.h"

TT_Table::TT_Table() {
    std::cout << "Initializing transposition table..." << std::endl;
    srand(time(NULL));
    std::random_device rd;
    this->null_entry.depth = -1;
    this->null_entry.eval = -1;
    this->null_entry.hash = -1;

    std::default_random_engine generator(rd());

    std::uniform_int_distribution<long long unsigned> distribution(0,0xFFFFFFFFFFFFFFFF);

    for(int i=0; i<8; i++) {
        for(int j=0; j<8; j++) {
            for(int k=0; k<12; k++) {
                this->hash_table[i][j][k] = distribution(generator);
            }
        }
    }

    for(int i=0; i<this->size; i++) {
        this->table[i].depth = -1;
        this->table[i].eval = -1;
        this->table[i].hash = -1;
    }
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

std::uint64_t TT_Table::get_hash(char board[8][8]) {
    std::uint64_t hash;
    int piece = 0;
    for(int i=0; i<8; i++) {
        for(int j=0; j<8; j++) {
            if(board[i][j] == '.') {
                continue;
            }

            piece = this->get_corresponding_num(board[i][j]);
            hash ^= this->hash_table[i][j][piece];
        }
    }
}

Entry TT_Table::query_hash(std::uint64_t hash) {
    int i = hash % this->size;
    if(this->table[i].hash == hash) {
        return this->table[i];
    }

    return this->null_entry;
}

void TT_Table::store_hash(Entry entry) {
    int i = entry.hash % this->size;
    this->table[i] = entry;
}