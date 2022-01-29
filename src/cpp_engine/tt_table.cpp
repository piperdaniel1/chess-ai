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

    for(int i=0; i<8; i++) {
        for(int j=0; j<8; j++) {
            for(int k=0; k<12; k++) {
                this->hash_table[i][j][k] = distribution(generator);
            }
        }
    }

    for(int i=0; i<this->size; i++) {
        this->table[i].depth = 0;
        this->table[i].eval = 0;
        this->table[i].hash = 0;
    }
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

std::uint64_t TT_Table::get_hash(char board[8][8]) {
    std::uint64_t hash = 0;
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

    return hash;
}

std::uint64_t TT_Table::test_thing(char board[8][8]) {
    for(int row=0; row<8; row++) {
        for(int col=0; col<8; col++) {
            std::cout << board[row][col] << " ";
        }
        std::cout << 8 - row << std::endl;
    }

    std::cout << "a b c d e f g h" << std::endl;

    return this->get_hash(board);
}

Entry TT_Table::query_board(char board[8][8]) {
    std::uint64_t hash = this->get_hash(board);

    int i = hash % this->size;
    if(this->table[i].hash == hash) {
        return this->table[i];
    }

    return this->null_entry;
}

void TT_Table::store_board(char board[8][8], int depth, int eval) {
    std::uint64_t hash = this->get_hash(board);
    int i = hash % this->size;
    this->table[i].depth = depth;
    this->table[i].eval = eval;
    this->table[i].hash = hash;
}