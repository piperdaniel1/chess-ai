//this file will contain the transposition table
#include "tt_table.h"

TT_Table::TT_Table() {
    std::cout << "Initializing transposition table..." << std::endl;
    srand(time(NULL));
    std::random_device rd;

    std::default_random_engine generator(rd());

    std::uniform_int_distribution<long long unsigned> distribution(0,0xFFFFFFFFFFFFFFFF);

    for(int i=0; i<8; i++) {
        for(int j=0; j<8; j++) {
            for(int k=0; k<6; k++) {
                this->hash_table[i][j][k] = distribution(generator);
            }
        }
    }
}