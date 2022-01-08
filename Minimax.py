from Board_Scorer import Board_Scorer
import chess
import numpy as np

'''
TODO for Minimax:
    - Rework board scoring to include a lot of stuff
        - |x| add piece maps to try to make the computer put its pieces in the best places at the beginning
    - Add iterative deepening
    - Add a transposition table
    - Sort the moves way more efficiently so that the best moves are always at the top
    - Add opening book
    - Add endgame strategies
    - Dynamically increase search depth using iterative deepening
'''

class Minimax:
    def __init__(self):
        self.eval = Board_Scorer()
        self.max_depth = 4
        self.starting_eval = 0
        pass

    def get_score_of_move(self, move_to_rate : chess.Move, board : chess.Board):
        uci = move_to_rate.uci()
        ending_file = ord(uci[2]) - 97
        ending_rank = uci[3]

        score = abs(float(ending_file) - 3.5) + abs(float(ending_rank) - 3.5)
        score = 20 - score

        if board.piece_at(move_to_rate.to_square) != None:
            score = score + 10
        
        return score

    def sort_moves_ideally(self, moves_to_sort, score_of_moves, verbose = False):
        score_of_moves_np = np.array(score_of_moves)
        moves_to_sort_np = np.array(moves_to_sort)

        moves_to_sort = moves_to_sort_np[score_of_moves_np.argsort()].tolist()

    def find_best_move(self, board : chess.Board, depth, maximize, alpha, beta, move2):        
        if depth == 0 or board.is_checkmate() or board.is_stalemate():
            return None, self.eval.get_score_of_board(board, move2)
        
        best_move = None

        if maximize:
            max_eval = -1000

            if depth == self.max_depth:
                total = board.legal_moves.count()

                anaylsis = 0

            for move in board.legal_moves:
                if depth == self.max_depth:
                    anaylsis += 1
                    print(f"Anaylzing move {anaylsis} out of {total}", end="\r")

                board.push(move)
                eval_of_branch = self.find_best_move(board, depth-1, False, alpha, beta, move2)[1]
                board.pop()

                if eval_of_branch > max_eval:
                    max_eval = eval_of_branch
                    best_move = move
                elif eval_of_branch == max_eval and depth == self.max_depth:
                    current_eval = self.eval.get_score_of_board(board, move2)
                    board.push(move)
                    next_eval = self.eval.get_score_of_board(board, move2)
                    board.pop()

                    if next_eval > current_eval:
                        best_move = move
                        max_eval = eval_of_branch

                alpha = max(alpha, eval_of_branch)
                if beta < alpha:
                    break

            if depth == self.max_depth:
                print(f"Returning {best_move} after checking {anaylsis} out of {total} moves")
                
            return best_move, max_eval
        else:
            min_eval = 1000

            if depth == self.max_depth:
                total = board.legal_moves.count()

                anaylsis = 0

            last_move = board.pop()
            board.push(last_move)

            for move in board.legal_moves:
                if depth == self.max_depth:
                    anaylsis += 1
                    print(f"Anaylzing move {anaylsis} out of {total} with depth {depth}", end="\r")

                board.push(move)
                eval_of_branch = self.find_best_move(board, depth-1, True, alpha, beta, move2)[1]
                board.pop()

                if eval_of_branch < min_eval:
                    min_eval = eval_of_branch
                    best_move = move
                elif eval_of_branch == min_eval and depth == self.max_depth:
                    current_eval = self.eval.get_score_of_board(board, move2)
                    board.push(move)
                    next_eval = self.eval.get_score_of_board(board, move2)
                    board.pop()
                    
                    if next_eval < current_eval:
                        best_move = move
                        min_eval = eval_of_branch
                
                beta = min(beta, eval_of_branch)
                if beta < alpha:
                    break

            if depth == self.max_depth:
                print(f"Returning {best_move} after checking {anaylsis} out of {total} moves")

            return best_move, min_eval