#include <stdio.h>
#include <string>
#include <iostream>
#include <vector>
#include "thc.h"

void display_position( thc::ChessRules &cr, const std::string &description ) {
    std::string fen = cr.ForsythPublish();
    std::string s = cr.ToDebugStr();
    printf( "%s\n", description.c_str() );
    printf( "FEN (Forsyth Edwards Notation) = %s\n", fen.c_str() );
    printf( "Position = %s\n", s.c_str() );
}

int get_all_positions(thc::ChessRules cr, int d) {
    int evals = 0;

    if(d == 0) {
        return 1;
    }

    thc::Move mv;
    std::vector<thc::Move> moves;
    std::vector<bool> check;
    std::vector<bool> mate;
    std::vector<bool> stalemate;
    cr.GenLegalMoveList(  moves, check, mate, stalemate );

    for( unsigned int i=0; i<moves.size(); i++ ) {
        mv = moves[i];
        cr.PlayMove(mv);
        evals += get_all_positions(cr, d-1);
        cr.PopMove(mv);
    }

    return evals;
}

int main() {
    thc::ChessRules cr;
    thc::Move mv;

    std::cout << "Num pos: " << get_all_positions(cr, 5) << std::endl;

    return 0;
}