from ast import In
from typing import Type
import chess
import numpy as np

class Board_Scorer:
    def __init__(self):
        self.knight_map = [[0, 0, 2, 2, 2, 2,-4, 0],
                           [1, 2, 3, 3, 3, 3, 2, 1],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [1, 2, 3, 3, 3, 3, 2, 1],
                           [0, 1, 2, 2, 2, 2, 1, 0]]

        self.king_map =   [[3, 9, 3, 1, 1, 3, 9, 3],
                           [3, 3, 2, 1, 1, 2, 3, 3],
                           [2, 1, 1, 1, 1, 1, 1, 2],
                           [1, 0, 0,-1,-1, 0, 0, 1],
                           [0,-1,-1,-2,-2,-1,-1, 0],
                          [-1,-2,-2,-3,-3,-2,-2,-1],
                          [-2,-3,-3,-4,-4,-3,-3,-2],
                          [-3,-4,-4,-5,-5,-4,-4,-3]]

        self.queen_map = [[-3,-2,-2,-1,-1,-2,-2,-3],
                          [-2,-1, 1, 2, 2, 1,-1,-2],
                           [0, 1, 2, 2, 2, 2, 1, 0],
                           [1, 2, 2, 2, 2, 2, 2, 1],
                           [1, 2, 2, 2, 2, 2, 2, 1],
                           [0, 1, 2, 2, 2, 2, 1, 0],
                          [-2,-1, 1, 2, 2, 1,-1,-2],
                          [-3,-2,-2,-1,-1,-2,-2,-3]]
        
        self.rook_map = [[-1,-1, 0, 1, 1, 0,-1,-3],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [0, 2, 2, 2, 2, 2, 2, 0],
                          [-1,-1,-1,-1,-1,-1,-1,-1]]
        
        self.bishop_map = [[1, 0, 0, 0, 0,-4, 0, 1],
                           [0, 3, 1, 1, 1, 1, 3, 0],
                           [0, 1, 1, 1, 1, 1, 1, 0],
                           [0, 1, 2, 1, 1, 2, 1, 0],
                           [0, 1, 1, 2, 2, 1, 1, 0],
                           [0, 1, 1, 2, 2, 1, 1, 0],
                           [0, 1, 1, 1, 1, 1, 1, 0],
                           [-1,0, 0, 0, 0, 0, 0,-1]]
        
        self.pawn_map = [[0, 0, 0, 0, 0, 0, 0, 0],
                         [1, 1, 1,-2,-2, 4, 4, 4],
                         [0,-1,-1, 0, 0,-1,-1,-1],
                         [0, 0, 0, 1, 1,-1,-1,-1],
                         [0, 0, 0, 1, 1, 0, 0, 0],
                         [0, 0, 1, 2, 2, 1, 0, 0],
                         [4, 4, 4, 4, 4, 4, 4, 4],
                         [4, 4, 4, 4, 4, 4, 4, 4]]
        
        self.endgame_king_map = [[-6,-4,-4,-4,-4,-4,-4,-6],
                                 [-4,-2,-2,-2,-2,-2,-2,-4],
                                 [-4,-2, 0, 0, 0, 0,-2,-4],
                                 [-4,-2, 0, 3, 3, 0,-2,-4],
                                 [-4,-2, 0, 3, 3, 0,-2,-4],
                                 [-4,-2, 0, 0, 0, 0,-2,-4],
                                 [-4,-2,-2,-2,-2,-2,-2,-4],
                                 [-6,-4,-4,-4,-4,-4,-4,-6]]

        self.white_rook = chess.Piece.from_symbol('R')
        self.black_rook = chess.Piece.from_symbol('r')

        self.white_knight = chess.Piece.from_symbol('N')
        self.black_knight = chess.Piece.from_symbol('n')

        self.white_king = chess.Piece.from_symbol('K')
        self.black_king = chess.Piece.from_symbol('k')

        self.white_queen = chess.Piece.from_symbol('Q')
        self.black_queen = chess.Piece.from_symbol('q')

        self.white_bishop = chess.Piece.from_symbol('B')
        self.black_bishop = chess.Piece.from_symbol('b')

        self.white_pawn = chess.Piece.from_symbol('P')
        self.black_pawn = chess.Piece.from_symbol('p')        
        
        self.cached_piece_map = None
        self.cached_piece_list = None

        self.piece_dict = {self.white_rook: self.rook_map, 
                           self.black_rook: self.rook_map,
                           self.white_knight: self.knight_map,
                           self.black_knight: self.knight_map,
                           self.white_king: self.king_map,
                           self.black_king: self.king_map,
                           self.white_queen: self.queen_map,
                           self.black_queen: self.queen_map,
                           self.white_bishop: self.bishop_map,
                           self.black_bishop: self.bishop_map,
                           self.white_pawn: self.pawn_map,
                           self.black_pawn: self.pawn_map}
        
        self.minimax_multiplier = {self.white_rook: 1,
                                   self.black_rook: -1,
                                   self.white_knight: 1,
                                   self.black_knight: -1,
                                   self.white_king: 1,
                                   self.black_king: -1,
                                   self.white_queen: 1,
                                   self.black_queen: -1,
                                   self.white_bishop: 1,
                                   self.black_bishop: -1,
                                   self.white_pawn: 1,
                                   self.black_pawn: -1}

        self.endgame_piece_dict = {self.white_rook: self.rook_map, 
                   self.black_rook: self.rook_map,
                   self.white_knight: self.endgame_king_map,
                   self.black_knight: self.endgame_king_map,
                   self.white_king: self.king_map,
                   self.black_king: self.king_map,
                   self.white_queen: self.queen_map,
                   self.black_queen: self.queen_map,
                   self.white_bishop: self.bishop_map,
                   self.black_bishop: self.bishop_map,
                   self.white_pawn: self.pawn_map,
                   self.black_pawn: self.pawn_map}

        self.piece_value_dict = {self.white_rook: 5,
                                 self.black_rook: -5,
                                 self.white_knight: 3,
                                 self.black_knight: -3,
                                 self.white_king: 0,
                                 self.black_king: 0,
                                 self.white_queen: 9,
                                 self.black_queen: -9,
                                 self.white_bishop: 3,
                                 self.black_bishop: -3,
                                 self.white_pawn: 1,
                                 self.black_pawn: -1}
        
        self.verbose = False

        self.situation0 = np.array([1, 0, 0, 0, 0,  0, 0, 0, 0, 0])
        self.situation1 = np.array([0, 0, 0, 0, 1,  0, 0, 0, 0, 0])
        self.situation2 = np.array([0, 2, 0, 0, 0,  0, 0, 0, 0, 0])
        self.situation3 = np.array([0, 0, 2, 0, 0,  0, 0, 0, 0, 0])
        self.situation4 = np.array([0, 1, 1, 0, 0,  0, 0, 0, 0, 0])
        self.situation5 = np.array([1, 0, 0, 0, 1,  1, 0, 0, 0, 0])
        self.situation6 = np.array([0, 0, 0, 1, 0,  0, 0, 0, 0, 0])

        self.tSituation0 = np.array([0, 0, 0, 0, 0,  1, 0, 0, 0, 0])
        self.tSituation1 = np.array([0, 0, 0, 0, 0,  0, 0, 0, 0, 1])
        self.tSituation2 = np.array([0, 0, 0, 0, 0,  0, 2, 0, 0, 0])
        self.tSituation3 = np.array([0, 0, 0, 0, 0,  0, 0, 2, 0, 0])
        self.tsituation4 = np.array([0, 0, 0, 0, 0,  0, 1, 1, 0, 0])
        self.tSituation5 = np.array([1, 0, 0, 0, 0,  1, 0, 0, 0, 1])
        self.tSituation6 = np.array([0, 0, 0, 0, 0,  0, 0, 0, 1, 0])

        self.endgame_situations = [self.situation0, self.situation1, self.situation2, self.situation3, self.situation4, self.situation5, 
                                   self.tSituation0, self.tSituation1, self.tSituation2, self.tSituation3, self.tsituation4, self.tSituation5]
        

        self.evals = [1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1]





        # rook + king vs king is winning                    0
        # queen + king vs king is winning                   (transformation of 0)
        # pawn + king vs king is winning                    1

        # bishop + king vs king is a draw                   2
        # knight + king vs king is a draw                   3
        # king vs king is a draw                            4
        # rook + king vs knight + king is a draw            5
        # rook + king vs bishop + king is a draw            6
        # rook + king vs pawn + king is a draw              7
        # equal material with no pawns is a draw            8
        
        # equal material with pawns depends on pawns that can promote

        # trade to get to one of these situations
    
    def simplify_situation(self, arr):
        # white could possibly win
        if self.cached_base_score < 0:
            if np.equal(arr[0:5], np.array([0, 0, 0, 0, 0])).all():
                if arr[5] > 0:
                    return self.tSituation0
                elif arr[8] > 0:
                    return self.tSituation6
                elif arr[9] > 0:
                    return self.tSituation5
        elif self.cached_base_score > 0:
            if np.equal(arr[5:10], np.array([0, 0, 0, 0, 0])).all():
                if arr[0] > 0:
                    return self.situation0
                elif arr[3] > 0:
                    return self.situation6
                elif arr[4] > 0:
                    return self.situation5
        
        return arr

    def endgame_strategy_eval(self, situation):
        if self.verbose:
            print(f"Evaulating endgame strategy for situation {situation}")
        # black is winning with at least a rook + king
        # strategy is to minimize distance between the kings and the rook
        if situation == 6:
            b_king_key = np.where(self.cached_piece_list == self.black_king)[0][0]
            b_king_key = self.cached_key_list[b_king_key]
            b_king_pos = self.convert_chesspos_to_gridpos(b_king_key)

            b_rook_key = np.where(self.cached_piece_list == self.black_rook)[0][0]
            b_rook_key = self.cached_key_list[b_rook_key]
            b_rook_pos = self.convert_chesspos_to_gridpos(b_rook_key)

            w_king_key = np.where(self.cached_piece_list == self.white_king)[0][0]
            w_king_key = self.cached_key_list[w_king_key]
            w_king_pos = self.convert_chesspos_to_gridpos(w_king_key)

            king_distance = abs(b_king_pos[1] - w_king_pos[1]) + abs(b_king_pos[0] - w_king_pos[0])
            rook_distance = abs(b_rook_pos[0] - w_king_pos[1]) + abs(b_rook_pos[0] - w_king_pos[0])


            # white king distance from any corner
            tl_distance = abs(w_king_pos[1] - 0) + abs(w_king_pos[0] - 0)
            tr_distance = abs(w_king_pos[1] - 0) + abs(w_king_pos[0] - 7)
            bl_distance = abs(w_king_pos[1] - 7) + abs(w_king_pos[0] - 0)
            br_distance = abs(w_king_pos[1] - 7) + abs(w_king_pos[0] - 7)

            corner_distance = min(tl_distance, tr_distance, bl_distance, br_distance)

            if corner_distance == tl_distance:
                rook_mating_square = (2, 0)
                king_mating_square = (0, 2)

                # get brook distance from rook mating square
                rook_mating_distance = abs(b_rook_pos[0] - rook_mating_square[0]) + abs(b_rook_pos[1] - rook_mating_square[1])
                king_mating_distance = abs(b_king_pos[0] - king_mating_square[0]) + abs(b_king_pos[1] - king_mating_square[1])
            elif corner_distance == tr_distance:
                rook_mating_square = (2, 7)
                king_mating_square = (7, 2)

                # get brook distance from rook mating square
                rook_mating_distance = abs(b_rook_pos[0] - rook_mating_square[0]) + abs(b_rook_pos[1] - rook_mating_square[1])
                king_mating_distance = abs(b_king_pos[0] - king_mating_square[0]) + abs(b_king_pos[1] - king_mating_square[1])
            elif corner_distance == bl_distance:
                rook_mating_square = (7, 0)
                king_mating_square = (0, 7)

                # get brook distance from rook mating square
                rook_mating_distance = abs(b_rook_pos[0] - rook_mating_square[0]) + abs(b_rook_pos[1] - rook_mating_square[1])
                king_mating_distance = abs(b_king_pos[0] - king_mating_square[0]) + abs(b_king_pos[1] - king_mating_square[1])
            elif corner_distance == br_distance:
                rook_mating_square = (7, 7)
                king_mating_square = (7, 7)

                # get brook distance from rook mating square
                rook_mating_distance = abs(b_rook_pos[0] - rook_mating_square[0]) + abs(b_rook_pos[1] - rook_mating_square[1])
                king_mating_distance = abs(b_king_pos[0] - king_mating_square[0]) + abs(b_king_pos[1] - king_mating_square[1])


            if self.verbose:
                print("King distance:", king_distance)
                print("Rook distance:", rook_distance)
                print("Corner distance:", corner_distance)

            # bigger distance = better for white
            return corner_distance * 5 + king_distance * 15 + rook_mating_distance * 3 + king_mating_distance * 3
        
        return 0

    # gets who is expected to win the endgame + eval based on type of endgame
    def get_endgame_eval(self):
        white_rooks = len(np.where(self.cached_piece_list == self.white_rook)[0])
        white_knights = len(np.where(self.cached_piece_list == self.white_knight)[0])
        white_bishops = len(np.where(self.cached_piece_list == self.white_bishop)[0])
        white_queens = len(np.where(self.cached_piece_list == self.white_queen)[0])
        white_pawns = len(np.where(self.cached_piece_list == self.white_pawn)[0])

        black_rooks = len(np.where(self.cached_piece_list == self.black_rook)[0])
        black_knights = len(np.where(self.cached_piece_list == self.black_knight)[0])
        black_bishops = len(np.where(self.cached_piece_list == self.black_bishop)[0])
        black_queens = len(np.where(self.cached_piece_list == self.black_queen)[0])
        black_pawns = len(np.where(self.cached_piece_list == self.black_pawn)[0])

        arr = np.array([white_rooks, white_knights, white_bishops, white_queens, white_pawns, black_rooks, black_knights, black_bishops, black_queens, black_pawns])
        arr = self.simplify_situation(arr)
        eval = 0
        strategy_eval = 0

        for i in range(len(self.endgame_situations)):
            if np.equal(self.endgame_situations[i], arr).all():
                strategy_eval = self.endgame_strategy_eval(i)
                eval = self.evals[i]
                break
        
        return eval * 50 + strategy_eval

    def get_endgame_score_of_piece(self, piece_type, location):
        if (piece_type == self.white_king and self.cached_base_score < 0) or (piece_type == self.black_king and self.cached_base_score > 0):
            col, row = location
            m = self.minimax_multiplier[piece_type]

            if m == 1:
                return self.endgame_piece_dict[piece_type][row][col] * m * 3
            else:
                return self.endgame_piece_dict[piece_type][7-row][col] * m * 3
        else:
            return 0
    
    def get_score_of_piece(self, piece_type, location):
        row, col = location

        m = self.minimax_multiplier[piece_type]
        
        if m == 1:
            return self.piece_dict[piece_type][row][col] * m
        else:
            return self.piece_dict[piece_type][7-row][col] * m
            
    def convert_chesspos_to_gridpos(self, chess_pos):
        row = chess_pos // 8
        col = chess_pos % 8

        return (row, col)

    def get_piece_map_scores(self, board):
        piece_map = self.cached_piece_map
        score = 0
        row = 0

        for key in range(64):
            try:
                piece_type = piece_map[key]
                row, col = self.convert_chesspos_to_gridpos(key)
                score_to_add = self.get_score_of_piece(piece_type, (row, col))
                score += score_to_add
            except KeyError:
                pass
        
        return score
    
    def get_endgame_piece_map_scores(self, board):
        piece_map = self.cached_piece_map
        score = 0
        row = 0

        for key in range(64):
            try:
                piece_type = piece_map[key]
                row, col = self.convert_chesspos_to_gridpos(key)
                score += self.get_endgame_score_of_piece(piece_type, (row, col))
            except KeyError:
                pass
        
        return score

    def get_base_score(self, board):            
        score = 0

        for key in range(64):
            try:
                character = self.cached_piece_map[key]
            except KeyError:
                continue

            if self.is_endgame:
                self.cached_piece_list[self.piece_ind] = character
                self.cached_key_list[self.piece_ind] = key
                self.piece_ind += 1

            score += self.piece_value_dict[character]

        self.cached_base_score = score
        return score
    
    def check_for_endgame(self, board):
        return len(self.cached_piece_map) <= 16
    
    def get_score_of_board(self, board : chess.Board, move2=0, phase = "default", verbose = False):
        self.cached_piece_list = np.empty(64, dtype=object)
        self.cached_key_list = np.empty(64, dtype=object)
        self.piece_ind = 0
        self.verbose = verbose
        self.cached_base_score = 0
        self.is_endgame = None

        # hard set score if one player has won
        if board.is_checkmate() == True:
            if self.verbose:
                print("Evaluating board:")
                print("Checkmate.")
            if board.outcome().result()[0] == "0":
                score = -1000
                return score
            else:
                score = 1000
                return score
        elif board.is_stalemate() == True or board.is_repetition():
            if self.verbose:
                print("Evaluating board:")
                print("Stalemate.")
            score = 0
            return score

        self.cached_piece_map = board.piece_map()
        # score starts at a tie.
        score = 0
        self.is_endgame = self.check_for_endgame(board)

        # add the base scores of each piece.
        score += self.get_base_score(board) * 10
        if self.verbose:
            print("Evaluating board:")
            print("Base score: " + str(score))

        if self.verbose:
            old_score = score
        # add scores based on where the pieces are and if it is endgame
        if self.is_endgame:
            if score < -90:
                score += self.piece_ind * ((abs(score) - 90) / 10)

            score += self.get_endgame_piece_map_scores(board)
            if self.verbose:
                print("Piece map score: " + str(score - old_score))
                print("\n")
                old_score = score
            
            score += self.get_endgame_eval()
            if self.verbose:
                print("Endgame eval score: " + str(score - old_score))
                print("\n")
                old_score = score
        else:
            score += self.get_piece_map_scores(board)
            if self.verbose:
                print("Piece map score: " + str(score - old_score))
                print("\n")
            
        return score