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
                   self.white_knight: self.knight_map,
                   self.black_knight: self.knight_map,
                   self.white_king: self.endgame_king_map,
                   self.black_king: self.endgame_king_map,
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

    def king_in_corner_eval(self):
        idx = np.where(self.cached_piece_list == self.white_king)[0][0]
        key = self.cached_key_list[idx]
        king_loc = self.convert_chesspos_to_gridpos(key)

        col_eval = min(king_loc[0], 7 - king_loc[0])
        row_eval = min(king_loc[1], 7 - king_loc[1])

        idx = np.where(self.cached_piece_list == self.black_king)[0][0]
        key = self.cached_key_list[idx]
        bking_loc = self.convert_chesspos_to_gridpos(key)

        krow_eval = abs(king_loc[1] - bking_loc[1])
        kcol_eval = abs(king_loc[0] - bking_loc[0])

        if self.verbose:
            print("col eval: ", col_eval)
            print("row eval: ", row_eval)
            print("krow eval: ", krow_eval)
            print("kcol eval: ", kcol_eval)
        
        corner_eval = (col_eval + row_eval)
        k_eval = (kcol_eval + krow_eval)

        if corner_eval >= 4:
            k_eval = 0
        else:
            k_eval = k_eval * round(((4 - corner_eval) * 0.5), 0)

        return corner_eval + k_eval
    
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
                score = -100000
                return score
            else:
                score = 100000
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
            k_in_corner = self.king_in_corner_eval()
            score += k_in_corner
            if self.verbose:
                print("King in corner eval: " + str(k_in_corner))
        else:
            score += self.get_piece_map_scores(board)
            if self.verbose:
                print("Piece map score: " + str(score - old_score))
                print("\n")
            
        return score