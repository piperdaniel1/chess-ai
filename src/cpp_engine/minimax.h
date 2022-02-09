#ifndef MINIMAX_H
#define MINIMAX_H

#include "tt_table.h"
#include "evaluator.h"
#include "board.h"
#include <iostream>
#include <chrono>

struct Node {
    Board b;
    int depth;
    int score;
    bool tt_table;
    std::string last_fen;
    std::vector<Node> children;
    Node();
    Node(Board, int);
    Node(Board, int, std::string);
    ~Node();
    void print();
};

class Minimax {
    private:
    TT_Table tt_table;
    Evaluator eval;
    bool v2;
    std::uint64_t start_time;
    std::uint64_t max_time;
    bool cut_search_early;

    public:
    Minimax();
    void get_best_move(Board board, int depth, int& num_moves, std::vector<MovC>& sorted_legal_moves, std::uint64_t max);
    int maximize(Board * board, int depth, int alpha, int beta, Node& cache_node, bool verbose);
    int minimize(Board * board, int depth, int alpha, int beta, Node& cache_node, bool verbose);
    std::uint64_t get_time();
    int positions_evaluated;
    int score_of_best_move;
    Node root_node; // root node of the search tree cache
};

#endif