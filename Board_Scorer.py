import chess

class Board_Scorer:
    def __init__(self):
        self.knight_map = [[0, 1, 2, 2, 2, 2, 1, 0],
                           [1, 2, 3, 3, 3, 3, 2, 1],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [2, 3, 4, 4, 4, 4, 3, 2],
                           [1, 2, 3, 3, 3, 3, 2, 1],
                           [0, 1, 2, 2, 2, 2, 1, 0]]

        self.king_map =   [[3, 4, 2, 1, 1, 2, 4, 3],
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
        
        self.rook_map = [[-1,-1,0, 1, 1, 0,-1,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [-1,0, 0, 0, 0, 0, 0,-1],
                          [0, 2, 2, 2, 2, 2, 2, 0],
                          [-1,-1,-1,-1,-1,-1,-1,-1]]
        
        self.bishop_map = [[1, 0, 0, 0, 0, 0, 0, 1],
                           [0, 3, 1, 1, 1, 1, 3, 0],
                           [0, 1, 1, 1, 1, 1, 1, 0],
                           [0, 1, 2, 1, 1, 2, 1, 0],
                           [0, 1, 1, 2, 2, 1, 1, 0],
                           [0, 1, 1, 2, 2, 1, 1, 0],
                           [0, 1, 1, 1, 1, 1, 1, 0],
                           [-1,0, 0, 0, 0, 0, 0,-1]]
        
        self.pawn_map = [[0, 0, 0, 0, 0, 0, 0, 0],
                         [0, 1, 1,-2,-2, 1, 1, 0],
                         [0,-1,-1, 0, 0,-1,-1, 0],
                         [0, 0, 0, 1, 1, 0, 0, 0],
                         [0, 0, 0, 1, 1, 0, 0, 0],
                         [0, 0, 1, 2, 2, 1, 0, 0],
                         [4, 4, 4, 4, 4, 4, 4, 4],
                         [4, 4, 4, 4, 4, 4, 4, 4]]
        
        self.endgame_king_map = [[-2,-2,-2,-2,-2,-2,-2,-2],
                                 [-2,-1,-1,-1,-1,-1,-1,-2],
                                 [-2,-1, 0, 0, 0, 0,-1,-2],
                                 [-2,-1, 0, 1, 1, 0,-1,-2],
                                 [-2,-1, 0, 1, 1, 0,-1,-2],
                                 [-2,-1, 0, 0, 0, 0,-1,-2],
                                 [-2,-1,-1,-1,-1,-1,-1,-2],
                                 [-2,-2,-2,-2,-2,-2,-2,-2]]


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

    def get_endgame_score_of_piece(self, piece_type, location):
        col, row = location

        m = self.minimax_multiplier[piece_type]

        if m == 1:
            return self.endgame_piece_dict[piece_type][row][col] * m
        else:
            return self.endgame_piece_dict[piece_type][7-row][col] * m
    
    def get_score_of_piece(self, piece_type, location):
        col, row = location

        m = self.minimax_multiplier[piece_type]

        if m == 1:
            return self.piece_dict[piece_type][row][col] * m
        else:
            return self.piece_dict[piece_type][7-row][col] * m
            
    def convert_chesspos_to_gridpos(self, chess_pos):
        row = chess_pos % 8
        col = chess_pos // 8

        return (row, col)

    def get_piece_map_scores(self, board):
        piece_map = self.cached_piece_map
        score = 0
        row = 0

        for key in range(64):
            try:
                piece_type = piece_map[key]
                row, col = self.convert_chesspos_to_gridpos(key)
                score += self.get_score_of_piece(piece_type, (row, col))
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

            score += self.piece_value_dict[character]

        return score
    
    def is_endgame(self, board):
        return len(self.cached_piece_map) <= 10
    
    def get_score_of_board(self, board : chess.Board, move2=0, phase = "default", verbose = False):
        self.verbose = verbose
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
            print("Evaluating board:")
            print("Stalemate.")
            score = 0
            return score

        self.cached_piece_map = board.piece_map()
        # score starts at a tie.
        score = 0
        is_endgame = self.is_endgame(board)

        # add the base scores of each piece.
        score += self.get_base_score(board) * 10
        if self.verbose:
            print("Evaluating board:")
            print("Base score: " + str(score))

        if self.verbose:
            old_score = score
        # add scores based on where the pieces are and if it is endgame
        if is_endgame:
            score += self.get_endgame_piece_map_scores(board)
        else:
            score += self.get_piece_map_scores(board)
        
        if self.verbose:
            print("Piece map score: " + str(score - old_score))
            print("\n")
            
        return score