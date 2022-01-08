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

    def get_score_of_piece(self, piece_type, location):
        score = 0
        col, row = location
        if piece_type.upper() == piece_type:
            # white scores should be inversed, as well as map.
            if piece_type == 'P':
                score += self.pawn_map[row][col]
            elif piece_type == 'R':
                score += self.rook_map[row][col]
            elif piece_type == 'N':
                score += self.knight_map[row][col]
            elif piece_type == 'B':
                score += self.bishop_map[row][col]
            elif piece_type == 'Q':
                score += self.queen_map[row][col]
            elif piece_type == 'K':
                score += self.king_map[row][col]
        else:
            # white scores should be inversed. the map should be as well.
            if piece_type == 'p':
                score -= self.pawn_map[7-row][col]
            elif piece_type == 'r':
                score -= self.rook_map[7-row][col]
            elif piece_type == 'n':
                score -= self.knight_map[7-row][col]
            elif piece_type == 'b':
                score -= self.bishop_map[7-row][col]
            elif piece_type == 'q':
                score -= self.queen_map[7-row][col]
            elif piece_type == 'k':
                score -= self.king_map[7-row][col]

        #print(f"Scored piece {piece_type} as {score} at the location {location}.")
        return score
            
    def convert_chesspos_to_gridpos(self, chess_pos):
        row = chess_pos % 8
        col = chess_pos // 8

        return (row, col)

    def get_piece_map_scores(self, board):
        piece_map = board.piece_map()
        score = 0
        row = 0

        for key in range(64):
            try:
                piece = piece_map[key]
                piece_type = piece.symbol()
                row, col = self.convert_chesspos_to_gridpos(key)
                score += self.get_score_of_piece(piece_type, (row, col))
            except KeyError:
                pass
        
        return score

    def get_base_score(self, board):
        fen = board.board_fen()
        score = 0

        for character in fen:
            if character.isnumeric() or character == "/":
                continue

            if character.islower():
                if character == 'p':
                    score -= 1
                elif character == 'n':
                    score -= 3
                elif character == 'b':
                    score -= 3
                elif character == 'r':
                    score -= 5
                elif character == 'q':
                    score -= 8
            else:
                if character == 'P':
                    score += 1
                elif character == 'N':
                    score += 3
                elif character == 'B':
                    score += 3
                elif character == 'R':
                    score += 5
                elif character == 'Q':
                    score += 8

        return score
    
    def get_score_of_board(self, board : chess.Board, move2, phase = "default"):
        score = 0
        row = 0

        score += self.get_base_score(board)
        score += self.get_piece_map_scores(board)
        
        og_color = board.turn
        
        board.turn = chess.WHITE
        white_legal_moves = board.legal_moves.count()

        board.turn = chess.BLACK
        black_legal_moves = board.legal_moves.count()

        board.turn = og_color

        score += white_legal_moves / 15
        score -= black_legal_moves / 15

        if board.is_checkmate() == True:
            if board.outcome().result()[0] == "0":
                score = -1000
            else:
                score = 1000
        elif board.is_stalemate() == True or board.is_repetition():
            score = 0
            
        return score