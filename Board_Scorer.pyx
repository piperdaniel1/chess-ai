#cython: language_level=3

import chess
knight_map = [[0, 1, 2, 2, 2, 2, 1, 0],
                   [1, 2, 3, 3, 3, 3, 2, 1],
                   [2, 3, 4, 4, 4, 4, 3, 2],
                   [2, 3, 4, 4, 4, 4, 3, 2],
                   [2, 3, 4, 4, 4, 4, 3, 2],
                   [2, 3, 4, 4, 4, 4, 3, 2],
                   [1, 2, 3, 3, 3, 3, 2, 1],
                   [0, 1, 2, 2, 2, 2, 1, 0]]

king_map =   [[3, 4, 2, 1, 1, 2, 4, 3],
                   [3, 3, 2, 1, 1, 2, 3, 3],
                   [2, 1, 1, 1, 1, 1, 1, 2],
                   [1, 0, 0,-1,-1, 0, 0, 1],
                   [0,-1,-1,-2,-2,-1,-1, 0],
                  [-1,-2,-2,-3,-3,-2,-2,-1],
                  [-2,-3,-3,-4,-4,-3,-3,-2],
                  [-3,-4,-4,-5,-5,-4,-4,-3]]

queen_map = [[-3,-2,-2,-1,-1,-2,-2,-3],
                  [-2,-1, 1, 2, 2, 1,-1,-2],
                   [0, 1, 2, 2, 2, 2, 1, 0],
                   [1, 2, 2, 2, 2, 2, 2, 1],
                   [1, 2, 2, 2, 2, 2, 2, 1],
                   [0, 1, 2, 2, 2, 2, 1, 0],
                  [-2,-1, 1, 2, 2, 1,-1,-2],
                  [-3,-2,-2,-1,-1,-2,-2,-3]]

rook_map = [[-1,-1,0, 1, 1, 0,-1,-1],
                  [-1,0, 0, 0, 0, 0, 0,-1],
                  [-1,0, 0, 0, 0, 0, 0,-1],
                  [-1,0, 0, 0, 0, 0, 0,-1],
                  [-1,0, 0, 0, 0, 0, 0,-1],
                  [-1,0, 0, 0, 0, 0, 0,-1],
                  [0, 2, 2, 2, 2, 2, 2, 0],
                  [-1,-1,-1,-1,-1,-1,-1,-1]]

bishop_map = [[1, 0, 0, 0, 0, 0, 0, 1],
                   [0, 3, 1, 1, 1, 1, 3, 0],
                   [0, 1, 1, 1, 1, 1, 1, 0],
                   [0, 1, 2, 1, 1, 2, 1, 0],
                   [0, 1, 1, 2, 2, 1, 1, 0],
                   [0, 1, 1, 2, 2, 1, 1, 0],
                   [0, 1, 1, 1, 1, 1, 1, 0],
                   [-1,0, 0, 0, 0, 0, 0,-1]]

pawn_map = [[0, 0, 0, 0, 0, 0, 0, 0],
                 [0, 1, 1,-2,-2, 1, 1, 0],
                 [0,-1,-1, 0, 0,-1,-1, 0],
                 [0, 0, 0, 1, 1, 0, 0, 0],
                 [0, 0, 0, 1, 1, 0, 0, 0],
                 [0, 0, 1, 2, 2, 1, 0, 0],
                 [4, 4, 4, 4, 4, 4, 4, 4],
                 [4, 4, 4, 4, 4, 4, 4, 4]]

endgame_king_map = [[-2,-2,-2,-2,-2,-2,-2,-2],
                         [-2,-1,-1,-1,-1,-1,-1,-2],
                         [-2,-1, 0, 0, 0, 0,-1,-2],
                         [-2,-1, 0, 1, 1, 0,-1,-2],
                         [-2,-1, 0, 1, 1, 0,-1,-2],
                         [-2,-1, 0, 0, 0, 0,-1,-2],
                         [-2,-1,-1,-1,-1,-1,-1,-2],
                         [-2,-2,-2,-2,-2,-2,-2,-2]]

white_rook = chess.Piece.from_symbol('R')
black_rook = chess.Piece.from_symbol('r')
white_knight = chess.Piece.from_symbol('N')
black_knight = chess.Piece.from_symbol('n')
white_king = chess.Piece.from_symbol('K')
black_king = chess.Piece.from_symbol('k')
white_queen = chess.Piece.from_symbol('Q')
black_queen = chess.Piece.from_symbol('q')
white_bishop = chess.Piece.from_symbol('B')
black_bishop = chess.Piece.from_symbol('b')
white_pawn = chess.Piece.from_symbol('P')
black_pawn = chess.Piece.from_symbol('p')        

cached_piece_map = None
piece_dict = {white_rook: rook_map, 
                   black_rook: rook_map,
                   white_knight: knight_map,
                   black_knight: knight_map,
                   white_king: king_map,
                   black_king: king_map,
                   white_queen: queen_map,
                   black_queen: queen_map,
                   white_bishop: bishop_map,
                   black_bishop: bishop_map,
                   white_pawn: pawn_map,
                   black_pawn: pawn_map}

endgame_piece_dict = {white_rook: rook_map, 
           black_rook: rook_map,
           white_knight: endgame_king_map,
           black_knight: endgame_king_map,
           white_king: king_map,
           black_king: king_map,
           white_queen: queen_map,
           black_queen: queen_map,
           white_bishop: bishop_map,
           black_bishop: bishop_map,
           white_pawn: pawn_map,
           black_pawn: pawn_map}
           
piece_value_dict = {white_rook: 5,
                         black_rook: -5,
                         white_knight: 3,
                         black_knight: -3,
                         white_king: 0,
                         black_king: 0,
                         white_queen: 9,
                         black_queen: -9,
                         white_bishop: 3,
                         black_bishop: -3,
                         white_pawn: 1,
                         black_pawn: -1}

def get_endgame_score_of_piece(piece_type, location):
    col, row = location
    return endgame_piece_dict[piece_type][row][col]

def get_score_of_piece(piece_type, location):
    col, row = location
    return piece_dict[piece_type][row][col]
        
def convert_chesspos_to_gridpos(chess_pos):
    row = chess_pos % 8
    col = chess_pos // 8
    return (row, col)

def get_piece_map_scores(cached_piece_map : dict):
    piece_map = cached_piece_map
    score = 0
    row = 0
    for key in range(64):
        try:
            piece_type = piece_map[key]
            row, col = convert_chesspos_to_gridpos(key)
            score += get_score_of_piece(piece_type, (row, col))
        except KeyError:
            pass
    
    return score

def get_endgame_piece_map_scores(cached_piece_map : dict):
    score = 0
    row = 0
    for key in range(64):
        try:
            piece_type = cached_piece_map[key]
            row, col = convert_chesspos_to_gridpos(key)
            score += get_endgame_score_of_piece(piece_type, (row, col))
        except KeyError:
            pass
    
    return score

def get_base_score(cached_piece_map : dict):            
    score = 0
    for key in range(64):
        try:
            character = cached_piece_map[key]
        except KeyError:
            continue
        score += piece_value_dict[character]
    return score

def is_endgame(cached_piece_map : dict):
    return len(cached_piece_map) <= 10

def get_score_of_board(board : chess.Board, cached_piece_map : dict):
    # score starts at a tie.
    score = 0
    endgame = is_endgame(cached_piece_map)
    # add the base scores of each piece.
    score += get_base_score(cached_piece_map)
    # add scores based on where the pieces are and if it is endgame

    if endgame:
        score += get_endgame_piece_map_scores(cached_piece_map)
    else:
        score += get_piece_map_scores(cached_piece_map)

    # hard set score if one player has won
    if board.is_checkmate() == True:
        if board.outcome().result()[0] == "0":
            score = -1000
        else:
            score = 1000
    elif board.is_stalemate() == True or board.is_repetition():
        score = 0
        
    return score