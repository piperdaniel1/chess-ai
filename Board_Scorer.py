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

    def get_score_of_board(self, board : chess.Board, move2, phase = "default"):
        score = 0
        row = 0
                       #q  r  b  n  p
        black_pieces = [0, 0, 0, 0, 0]
        white_pieces = [0, 0, 0, 0, 0]
        fen = board.board_fen()

        for character in fen:
            if character.isnumeric():
                continue

            if character == "/":
                row += 1
                continue

            if character.islower():
                if move2 < 15:
                    row_bonus = (row - 1) * 0.075
                else:
                    row_bonus = 0

                if character == 'p':
                    score -= 1 + row_bonus * 2
                    black_pieces[4] += 1
                elif character == 'n':
                    score -= 3 + row_bonus
                    black_pieces[3] += 1
                elif character == 'b':
                    score -= 3 + row_bonus
                    black_pieces[2] += 1
                elif character == 'r':
                    score -= 5 + row_bonus
                    black_pieces[1] += 1
                elif character == 'q':
                    score -= 8 + row_bonus
                    black_pieces[0] += 1
            else:
                if move2 < 15:
                    row_bonus = (row - 7) * -0.15
                else:
                    row_bonus = 0

                if character == 'P':
                    score += 1 + row_bonus * 2
                    white_pieces[4] += 1
                elif character == 'N':
                    score += 3 + row_bonus
                    white_pieces[3] += 1
                elif character == 'B':
                    score += 3 + row_bonus
                    white_pieces[2] += 1
                elif character == 'R':
                    score += 5 + row_bonus
                    white_pieces[1] += 1
                elif character == 'Q':
                    score += 8 + row_bonus
                    white_pieces[0] += 1  

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