from Board_Scorer import Board_Scorer
from transposition_table import Transposition_Table, Entry
import chess
import numpy as np
import random
import time
from Opening_Book import Opening_Book, Opening_Entry

'''
TODO for Minimax:
    - Rework board scoring to include a lot of stuff
        - |x| add piece maps to try to make the computer put its pieces in the best places at the beginning
    - Make sure the eval doesn't end on a capture
    - |x| Add iterative deepening
    - |x| Add a transposition table
    - |x| Sort the moves way more efficiently so that the best moves are always at the top
    - Add opening book
    - Add endgame strategies
    - |x| Dynamically increase search depth using iterative deepening
'''


class Minimax:
    def __init__(self):
        self.eval = Board_Scorer()
        self.tt = Transposition_Table()
        self.opening_book = Opening_Book()
        self.max_depth = 4
        self.starting_eval = 0
        self.zorbist_table = [[random.randint(1,2**64 - 1) for i in range(12)] for j in range(64)]
        self.positions_searched = 0
        pass

    def get_index_of_piece(self, piece):
        if (piece=='P'):
            return 0
        if (piece=='N'):
            return 1
        if (piece=='B'):
            return 2
        if (piece=='R'):
            return 3
        if (piece=='Q'):
            return 4
        if (piece=='K'):
            return 5
        if (piece=='p'):
            return 6
        if (piece=='n'):
            return 7
        if (piece=='b'):
            return 8
        if (piece=='r'):
            return 9
        if (piece=='q'):
            return 10
        if (piece=='k'):
            return 11
        else:
            return -1
    #
    # steps to simulate this hash
    # 1. get prev hash of board
    # 2. run xor of value of where piece used to be and where it is now
    # 3. you end up with a new hash value that is the new board state
    #
    def get_zorbist_hash(self, board):
        piece_map = board.piece_map()
        zorbist_hash = 0

        for key in range(64):
            try:
                piece = piece_map[key]
                piece_index = self.get_index_of_piece(piece.symbol())
                zorbist_hash ^= self.zorbist_table[key][piece_index]
            except KeyError:
                pass
        
        return zorbist_hash
    
    def transpose_zorbist_hash(self, og_hash, board, captured_piece=None):
        last_move = board.move_stack[-1]
        from_square = last_move.from_square
        to_square = last_move.to_square
        piece = board.piece_at(to_square)
        piece_index = self.get_index_of_piece(piece.symbol())

        if captured_piece != None:
            captured_index = self.get_index_of_piece(captured_piece)
            og_hash ^= self.zorbist_table[to_square][captured_index]

        og_hash ^= self.zorbist_table[from_square][piece_index]
        og_hash ^= self.zorbist_table[to_square][piece_index]

        return og_hash

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

    def find_best_move(self, board : chess.Board, maximizing_player : bool, alpha : int, beta : int, move2):
        if move2 < 10:
            book_move = self.opening_book.decode(self.opening_book.get_zorbist_hash(board))
            if book_move != None:
                print("Returning book move " + str(book_move) + " from opening book")
                return book_move

        start_time = time.time()
        depth = 1
        self.max_depth = depth
        while True:
            best_move = self.rec_minimax(board, depth, maximizing_player, alpha, beta, move2)[0]

            if time.time() - start_time > 1:
                break
                
            depth += 1
            self.max_depth = depth
        
        print(f"Returning {best_move} after searching {self.positions_searched} positions")
        
        return best_move

    def sort(self, moves_to_sort, score_of_moves):
        score_of_moves_np = np.array(score_of_moves)
        moves_to_sort_np = np.array(moves_to_sort)

        return moves_to_sort_np[score_of_moves_np.argsort()].tolist()

    def sort_moves_by_probable_score(self, board, current_hash, depth):
        moves_to_sort = list(board.legal_moves)
        score_of_moves = []
        for move in moves_to_sort:
            captured_piece = board.piece_at(move.to_square)
            board.push(move)
            new_hash = self.transpose_zorbist_hash(current_hash, board, captured_piece)
            tt_entry = self.tt.decode(new_hash)
            if tt_entry != None:
                score_of_moves.append(tt_entry.eval)
            else:
                score_of_moves.append(0)
            board.pop()
        
        moves_to_sort = self.sort(moves_to_sort, score_of_moves)
        return moves_to_sort

    def rec_minimax(self, board : chess.Board, depth, maximize, alpha, beta, move2, current_hash=None):
        if current_hash == None:
            current_hash = self.get_zorbist_hash(board)
        
        if depth == 0 or board.is_checkmate() or board.is_stalemate():
            return None, self.eval.get_score_of_board(board, move2)
        
        best_move = None
        captured_piece = None

        # computer is trying to maximize the eval with this move
        if maximize:
            # init eval to -inf
            max_eval = -1000

            # init debug values for printing (only on highest level)
            if depth == self.max_depth:
                total = board.legal_moves.count()

                anaylsis = 0

            if depth == self.max_depth:
                legal_moves = self.sort_moves_by_probable_score(board, current_hash, depth)
            else:
                legal_moves = list(board.legal_moves)

            # loop through all legal moves
            for move in legal_moves:
                # again print debug values on highest level
                if depth == self.max_depth:
                    anaylsis += 1
                    print(f"Anaylzing move {anaylsis} out of {total}", end="\r")

                captured_piece = board.piece_at(move.to_square)

                # push move onto board
                board.push(move)
                self.positions_searched += 1
                # check score of move
                new_hash = self.transpose_zorbist_hash(current_hash, board, captured_piece)
                tt_entry = self.tt.decode(new_hash)
                if tt_entry != None:
                    if tt_entry.depth >= depth:
                        eval_of_branch = tt_entry.eval
                    else:
                        eval_of_branch = self.rec_minimax(board, depth-1, False, alpha, beta, move2, new_hash)[1]
                        new_entry = Entry(new_hash, eval_of_branch, depth)
                        self.tt.encode(new_entry)
                else:
                    eval_of_branch = self.rec_minimax(board, depth-1, False, alpha, beta, move2, new_hash)[1]
                    new_entry = Entry(new_hash, eval_of_branch, depth)
                    self.tt.encode(new_entry)
                    
                # pop move off board to make room for the next move
                board.pop()

                # check if that move is better than the current best move
                if eval_of_branch > max_eval:
                    max_eval = eval_of_branch
                    best_move = move
                '''
                # i think this is some kind of tie breaker system but I don't remember making it at all
                elif eval_of_branch == max_eval and depth == self.max_depth:
                    current_eval = self.eval.get_score_of_board(board, move2)
                    board.push(move)
                    next_eval = self.eval.get_score_of_board(board, move2)
                    board.pop()

                    if next_eval > current_eval:
                        best_move = move
                        max_eval = eval_of_branch
                '''

                alpha = max(alpha, eval_of_branch)
                if beta < alpha:
                    break
                
            return best_move, max_eval
        else:
            min_eval = 1000

            if depth == self.max_depth:
                total = board.legal_moves.count()

                anaylsis = 0

            
            if depth == self.max_depth:
                legal_moves = self.sort_moves_by_probable_score(board, current_hash, depth)
            else:
                legal_moves = list(board.legal_moves)
            #last_move = board.pop()
            #board.push(last_move)

            for move in legal_moves:
                if depth == self.max_depth:
                    anaylsis += 1
                    print(f"Anaylzing move {anaylsis} out of {total} with depth {depth}", end="\r")

                board.push(move)
                self.positions_searched += 1
                new_hash = self.transpose_zorbist_hash(current_hash, board, captured_piece)
                tt_entry = self.tt.decode(new_hash)
                if tt_entry != None:
                    if tt_entry.depth >= depth:
                        eval_of_branch = tt_entry.eval
                    else:
                        eval_of_branch = self.rec_minimax(board, depth-1, True, alpha, beta, move2, new_hash)[1]
                        
                        new_entry = Entry(new_hash, eval_of_branch, depth)
                        self.tt.encode(new_entry)
                else:
                    eval_of_branch = self.rec_minimax(board, depth-1, True, alpha, beta, move2, new_hash)[1]
                    new_entry = Entry(new_hash, eval_of_branch, depth)
                    self.tt.encode(new_entry)

                board.pop()

                if eval_of_branch < min_eval:
                    min_eval = eval_of_branch
                    best_move = move
                '''
                elif eval_of_branch == min_eval and depth == self.max_depth:
                    current_eval = self.eval.get_score_of_board(board, move2)
                    board.push(move)
                    next_eval = self.eval.get_score_of_board(board, move2)
                    board.pop()
                    
                    if next_eval < current_eval:
                        best_move = move
                        min_eval = eval_of_branch
                '''
                
                beta = min(beta, eval_of_branch)
                if beta < alpha:
                    break

            return best_move, min_eval