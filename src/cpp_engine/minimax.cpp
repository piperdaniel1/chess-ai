// this file will contain the minimax algorithm
#include "minimax.h"

Node::Node() {
    this->b = Board();
    this->depth = 0;
    this->tt_table = false;
}

Node::~Node() {
    this->children.clear();
}

Node::Node(Board b, int d) {
    this->b = b;
    this->depth = d;
    this->tt_table = false;
}

Node::Node(Board b, int d, std::string fen) {
    this->b = b;
    this->depth = d;
    this->tt_table = false;
    this->last_fen = fen;
}

void Node::print() {
    this->b.print_self();
    std::cout << "Depth: " << this->depth << std::endl;
    std::cout << "Score: " << this->score << std::endl;
}

Minimax::Minimax() {
    std::cout << "Initializing minimax..." << std::endl;
    this->v2 = false;
}

std::uint64_t Minimax::get_time() {
    return std::chrono::duration_cast<std::chrono::milliseconds>(
    std::chrono::system_clock::now().time_since_epoch()).count();
}

int Minimax::minimize(Board * board, int depth, int alpha, int beta, Node& cache_node, bool verbose=false) {
    if (this->get_time() - this->start_time > this->max_time) {
        this->cut_search_early = true;
        return 0;
    }

    std::vector<MovC> movesC;
    board->get_legal_movC(movesC);

    int game_over = this->eval.is_game_overC(*board, movesC);
    if (game_over != 0) {
        this->positions_evaluated++;
        if (game_over == 1) {
            return 100000;
        } else if (game_over == 2) {
            return -100000;
        } else if (game_over == 3) {
            return 0;
        }
    }

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluateC(*board, movesC, false);
        return final_eval;
    }

    int best_score = 100000;
    int score = 0;
    for (MovC mov : movesC) {
        board->push_movC(mov);

        cache_node.children.push_back(Node(*board, depth, mov.get_fen()));

        Entry tt_entry = this->tt_table.query_board(board->board);
        if (tt_entry.depth >= depth) {
            score = tt_entry.eval;
            cache_node.children.back().tt_table = true;
        } else {
            score = this->maximize(board, depth - 1, alpha, beta, cache_node.children.back(), false);

            this->tt_table.store_board(board->board, depth, score);
        }

        board->pull_movC(mov);

        if(score < -10000) {
            score += 10;
        }

        if (score < best_score) {
            cache_node.score = score;
            best_score = score;
        }

        if (best_score < beta) {
            beta = best_score;
        }

        if (alpha > beta) {
            break;
        }
    }

    return best_score;
}

int Minimax::maximize(Board * board, int depth, int alpha, int beta, Node& cache_node, bool verbose = false) {
    if (this->get_time() - this->start_time > this->max_time) {
        this->cut_search_early = true;
        return 0;
    }

    std::vector<MovC> movesC;
    board->get_legal_movC(movesC);

    int game_over = this->eval.is_game_overC(*board, movesC);
    if (game_over != 0) {
        this->positions_evaluated++;
        if (game_over == 1) {
            return 100000;
        } else if (game_over == 2) {
            return -100000;
        } else if (game_over == 3) {
            return 0;
        }
    }

    if (depth == 0) {
        this->positions_evaluated++;
        int final_eval = this->eval.evaluateC(*board, movesC, false);
        return final_eval;
    }

    int best_score = -100000;
    int score = 0;
    for(MovC mov : movesC) {
        board->push_movC(mov);

        cache_node.children.push_back(Node(*board, depth, mov.get_fen()));

        Entry tt_entry = this->tt_table.query_board(board->board);
        if (tt_entry.depth >= depth) {
            score = tt_entry.eval;
            cache_node.children.back().tt_table = true;
        } else {
            score = this->minimize(board, depth - 1, alpha, beta, cache_node.children.back(), false);

            this->tt_table.store_board(board->board, depth, score);
        }

        board->pull_movC(mov);

        if(score < -10000) {
            score += 10;
        }

        if (score > best_score) {
            cache_node.score = score;
            best_score = score;
        }

        if (best_score > alpha) {
            alpha = best_score;
        }

        if (alpha > beta) {
            break;
        }
    }

    return best_score;
}

void Minimax::get_best_move(Board board, int depth, int& num_moves, std::vector<MovC>& sorted_legal_moves, std::uint64_t max) {
    this->root_node.b = board;
    this->root_node.depth = depth;
    this->root_node.children.clear();
    this->root_node.last_fen = "root";

    this->eval.evaluateC(board, sorted_legal_moves, true);
    this->start_time = get_time();
    this->cut_search_early = false;
    this->max_time = max;
    this->positions_evaluated = 0;
    int alpha = -100000;
    int beta = 100000;

    int game_over = this->eval.is_game_overC(board, sorted_legal_moves);
    if (game_over != 0) {
        this->positions_evaluated++;
    }

    int best_score = 0;

    if (board.turn) {
        best_score = -100000;
    } else {
        best_score = 100000;
    }
    Move * best_move = nullptr;
    int score = 0;
    int scores[100]; // i really don't think there will ever be more than 100 moves
    num_moves = 0;

    for(MovC mov : sorted_legal_moves) {
        Board * next_board = new Board(board);
        next_board->push_movC(mov);
        
        this->root_node.children.push_back(Node(*next_board, depth, mov.get_fen()));

        Entry tt_entry = this->tt_table.query_board(next_board->board);
        if (tt_entry.depth >= depth) {
            score = tt_entry.eval;
            this->root_node.children.back().tt_table = true;
        } else {
            if(!board.turn) {
                score = this->maximize(next_board, depth - 1, alpha, beta, root_node.children.back(), false);
            } else {
                score = this->minimize(next_board, depth - 1, alpha, beta, root_node.children.back(), false);
            }

            this->tt_table.store_board(next_board->board, depth, score);
        }

        scores[num_moves] = score;
        num_moves++;

        // minimizing
        if(!board.turn) {
            if (best_score < beta) {
                this->root_node.score = score;
                beta = best_score;
            }
        // maximizing
        } else {
            if (best_score > alpha) {
                this->root_node.score = score;
                alpha = best_score;
            }
        }
    }
    
    if(board.turn) {
        // rank the moves by score highest to lowest
        int swaps = 0;
        while(1) {
            for(int i=0; i<num_moves-1; i++) {
                if(scores[i] < scores[i+1]) {
                    std::swap(scores[i], scores[i+1]);
                    
                    std::swap(sorted_legal_moves[i], sorted_legal_moves[i+1]);
                    swaps++;
                }
            }
            if(swaps == 0) {
                break;
            }
            swaps = 0;
        }
    } else {
        // rank the moves by score lowest to highest
        int swaps = 0;
        while(1) {
            for(int i=0; i<num_moves-1; i++) {
                if(scores[i] > scores[i+1]) {
                    std::swap(scores[i], scores[i+1]);

                    std::swap(sorted_legal_moves[i], sorted_legal_moves[i+1]);
                    swaps++;
                }
            }
            if(swaps == 0) {
                break;
            }
            swaps = 0;
        }
    }
    std::cout << "Current best move is evaluated at " << scores[0] << std::endl;
    this->score_of_best_move = scores[0];
}