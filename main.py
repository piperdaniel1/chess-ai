import pygame
import chess
import chess.pgn
import collections
from socket_interface import Connection
from typing import List, Tuple, Union
import sys

# Colors
BLACK_SQUARE_COLOR = (118, 150, 86)
BLACK_SEL_SQUARE_COLOR = (186, 202, 43)
BLACK_MOVE_OPTION_COLOR = (170, 155, 97)
BLACK_SQUARE_IN_CHECK = (150, 50, 50)
BLACK_CAPTURE_OPTION_COLOR = (210, 120, 80)

WHITE_SQUARE_COLOR = (238, 238, 210)
WHITE_SEL_SQUARE_COLOR = (246, 246, 105)
WHITE_MOVE_OPTION_COLOR = (255, 200, 180)
WHITE_SQUARE_IN_CHECK = (225, 100, 100)
WHITE_CAPTURE_OPTION_COLOR = (255, 150, 130)

BORDER_COLOR = (50, 50, 50)

# Pixel Sizes
BORDER_WIDTH = 5
BOARD_SIZE = 960
assert(BOARD_SIZE % 8 == 0)
SQUARE_SIZE = BOARD_SIZE // 8
TIMER_AREA_WIDTH = 0

# Sprites
WHITE_PAWN = pygame.image.load("pieces/png-versions/P-white.png")
WHITE_PAWN = pygame.transform.scale(WHITE_PAWN, (SQUARE_SIZE, SQUARE_SIZE))

BLACK_PAWN = pygame.image.load("pieces/png-versions/p-black.png")
BLACK_PAWN = pygame.transform.scale(BLACK_PAWN, (SQUARE_SIZE, SQUARE_SIZE))

WHITE_ROOK = pygame.image.load("pieces/png-versions/R-white.png")
WHITE_ROOK = pygame.transform.scale(WHITE_ROOK, (SQUARE_SIZE, SQUARE_SIZE))

BLACK_ROOK = pygame.image.load("pieces/png-versions/r-black.png")
BLACK_ROOK = pygame.transform.scale(BLACK_ROOK, (SQUARE_SIZE, SQUARE_SIZE))

WHITE_KNIGHT = pygame.image.load("pieces/png-versions/N-white.png")
WHITE_KNIGHT = pygame.transform.scale(WHITE_KNIGHT, (SQUARE_SIZE, SQUARE_SIZE))

BLACK_KNIGHT = pygame.image.load("pieces/png-versions/n-black.png")
BLACK_KNIGHT = pygame.transform.scale(BLACK_KNIGHT, (SQUARE_SIZE, SQUARE_SIZE))

WHITE_BISHOP = pygame.image.load("pieces/png-versions/B-white.png")
WHITE_BISHOP = pygame.transform.scale(WHITE_BISHOP, (SQUARE_SIZE, SQUARE_SIZE))

BLACK_BISHOP = pygame.image.load("pieces/png-versions/b-black.png")
BLACK_BISHOP = pygame.transform.scale(BLACK_BISHOP, (SQUARE_SIZE, SQUARE_SIZE))

WHITE_QUEEN = pygame.image.load("pieces/png-versions/Q-white.png")
WHITE_QUEEN = pygame.transform.scale(WHITE_QUEEN, (SQUARE_SIZE, SQUARE_SIZE))

BLACK_QUEEN = pygame.image.load("pieces/png-versions/q-black.png")
BLACK_QUEEN = pygame.transform.scale(BLACK_QUEEN, (SQUARE_SIZE, SQUARE_SIZE))

WHITE_KING = pygame.image.load("pieces/png-versions/K-white.png")
WHITE_KING = pygame.transform.scale(WHITE_KING, (SQUARE_SIZE, SQUARE_SIZE))

BLACK_KING = pygame.image.load("pieces/png-versions/k-black.png")
BLACK_KING = pygame.transform.scale(BLACK_KING, (SQUARE_SIZE, SQUARE_SIZE))

PIECE_MAP = {
    'p': BLACK_PAWN,
    'r': BLACK_ROOK,
    'n': BLACK_KNIGHT,
    'b': BLACK_BISHOP,
    'q': BLACK_QUEEN,
    'k': BLACK_KING,
    'P': WHITE_PAWN,
    'R': WHITE_ROOK,
    'N': WHITE_KNIGHT,
    'B': WHITE_BISHOP,
    'Q': WHITE_QUEEN,
    'K': WHITE_KING,
}

# Initializes pygame and returns the screen object
def init_pygame(width, height):
    pygame.init()
    pygame.display.set_caption("Chess vs. AI")

    return pygame.display.set_mode((width, height))

def render_board_squares(screen):
    for i in range(8):
        for j in range(8):
            if (i + j) % 2 == 0:
                color = WHITE_SQUARE_COLOR
            else:
                color = BLACK_SQUARE_COLOR

            pygame.draw.rect(screen, color, (i * SQUARE_SIZE + BORDER_WIDTH, j * SQUARE_SIZE + BORDER_WIDTH, SQUARE_SIZE, SQUARE_SIZE))

def render_board_border(screen):
    pygame.draw.rect(screen, BORDER_COLOR, (0, 0, BOARD_SIZE + BORDER_WIDTH * 2, BOARD_SIZE + BORDER_WIDTH * 2), BORDER_WIDTH)

def render_selected_square(screen, square):
    if square is None:
        return

    i, j = square
    if (i + j) % 2 == 0:
        color = WHITE_SEL_SQUARE_COLOR
    else:
        color = BLACK_SEL_SQUARE_COLOR

    pygame.draw.rect(screen, color, (i * SQUARE_SIZE + BORDER_WIDTH, j * SQUARE_SIZE + BORDER_WIDTH, SQUARE_SIZE, SQUARE_SIZE))

def render_move_option(screen, square):
    i, j = square
    if (i + j) % 2 == 0:
        color = WHITE_MOVE_OPTION_COLOR
    else:
        color = BLACK_MOVE_OPTION_COLOR

    pygame.draw.rect(screen, color, (i * SQUARE_SIZE + BORDER_WIDTH, j * SQUARE_SIZE + BORDER_WIDTH, SQUARE_SIZE, SQUARE_SIZE))

def render_check(screen, square):
    i, j = square
    if (i + j) % 2 == 0:
        color = WHITE_SQUARE_IN_CHECK
    else:
        color = BLACK_SQUARE_IN_CHECK

    pygame.draw.rect(screen, color, (i * SQUARE_SIZE + BORDER_WIDTH, j * SQUARE_SIZE + BORDER_WIDTH, SQUARE_SIZE, SQUARE_SIZE))

def render_capture_option(screen, square):
    i, j = square
    if (i + j) % 2 == 0:
        color = WHITE_CAPTURE_OPTION_COLOR
    else:
        color = BLACK_CAPTURE_OPTION_COLOR

    pygame.draw.rect(screen, color, (i * SQUARE_SIZE + BORDER_WIDTH, j * SQUARE_SIZE + BORDER_WIDTH, SQUARE_SIZE, SQUARE_SIZE))

def render_piece(screen, piece, square):
    screen.blit(piece, (square[0] * SQUARE_SIZE + BORDER_WIDTH, square[1] * SQUARE_SIZE + BORDER_WIDTH))

def rerender(screen, state: 'State'):
    screen.fill((255, 255, 255))

    render_board_squares(screen)
    render_board_border(screen)

    render_selected_square(screen, state.get_selected_square())

    for square in state.get_move_options():
        render_move_option(screen, square)

    # render_check(screen, (4, 0))
    # render_check(screen, (4, 7))

    for square in state.get_attack_options():
        render_capture_option(screen, square)

    for piece, square in state.get_piece_list():
        render_piece(screen, PIECE_MAP[piece], square)

    pygame.display.flip()

def convert_pixels_to_square(x, y) -> Union[Tuple[int, int], None]:
    x -= BORDER_WIDTH
    y -= BORDER_WIDTH
    x //= SQUARE_SIZE
    y //= SQUARE_SIZE

    if x < 0 or x > 7 or y < 0 or y > 7:
        return None

    return (x, y)

class Game:
    def __init__(self):
        self.board = chess.Board()
        pass

    def __conv_square(self, square):
        return (square % 8, (63 - square) // 8)
    
    def __reverse_conv_square(self, square):
        return square[0] + (7 - square[1]) * 8
    
    def push_move(self, from_square, to_square):
        """
        Pushes a move to the board.
        Takes care of promotion (auto picks queen).
        """
        derived_move = chess.Move(self.__reverse_conv_square(from_square), self.__reverse_conv_square(to_square))

        if self.board.piece_type_at(self.__reverse_conv_square(from_square)) == chess.PAWN and (to_square[1] == 0 or to_square[1] == 7):
            derived_move.promotion = chess.QUEEN

        self.board.push(derived_move)
        return derived_move
    
    def push_uci(self, uci):
        self.board.push_uci(uci)

        return chess.Move.from_uci(uci)

    def get_standard_moves_from_square(self, square):
        # yayayay readibility
        return [self.__conv_square(move.to_square) for move in self.board.legal_moves if self.__conv_square(move.from_square) == square and self.board.color_at(move.to_square) == None]

    def get_attack_moves_from_square(self, square):
        # yayayay readibility pt. 2
        return [self.__conv_square(move.to_square) for move in self.board.legal_moves if self.__conv_square(move.from_square) == square and self.board.color_at(move.to_square) != None]
    
    def get_color_at(self, square):
        return self.board.color_at(self.__reverse_conv_square(square))

    def get_piece_list(self):
        pieces = list(self.board.piece_map().values())
        pieces = [piece.symbol() for piece in pieces]
        squares = list(self.board.piece_map().keys())
        squares = [self.__conv_square(square) for square in squares]

        return list(zip(pieces, squares))

# Represents the complete state of the UI
# Also has functions for progressing the UI state based on input events
class State:
    def __init__(self, color, ai_only):
        self.player_color = color
        self.ai_only = ai_only
        self.__game = Game()
        self.__selected_square = None
        self.__move_options = []
        self.__capture_options = []
        self.__check_squares = []
        self.__move_buffer = []
    
    def get_piece_list(self):
        return self.__game.get_piece_list()
    
    def get_selected_square(self):
        return self.__selected_square
    
    def get_move_options(self):
        return self.__move_options
    
    def get_attack_options(self):
        return self.__capture_options
    
    def pop_move_buffer(self):
        if len(self.__move_buffer) == 0:
            return None
        else:
            return self.__move_buffer.pop(0)
    
    def is_players_turn(self):
        if self.ai_only:
            return False
        
        return self.__game.board.turn == self.player_color
    
    def push_move(self, uci):
        self.__move_buffer.append(self.__game.push_uci(uci))
    
    def get_board(self):
        return self.__game.board

    # Mutate the state based on a Pygame event
    def next(self, event):
        if event.type == pygame.MOUSEBUTTONDOWN:
            x, y = event.pos
            result = convert_pixels_to_square(x, y)

            if result is None:
                return
            
            self.__handle_selection_change(result)
    
    def __handle_selection_change(self, square):
        # We are moving a piece
        if square in self.__move_options or square in self.__capture_options:
            self.__move_buffer.append(self.__game.push_move(self.__selected_square, square))
            self.__set_selected_square(None)
            return

        self.__set_selected_square(square)
    
    def __set_standard_moves(self, square):
        self.__move_options = self.__game.get_standard_moves_from_square(square)
    
    def __set_attack_moves(self, square):
        self.__capture_options = self.__game.get_attack_moves_from_square(square)
    
    def __clear_selected_square(self):
        self.__selected_square = None
        self.__move_options = []
        self.__capture_options = []
    
    def __is_square_friendly(self, square):
        return False if square == None else self.__game.get_color_at(square) == self.player_color

    def __set_selected_square(self, square):
        # We can only select a square if it is friendly to us (the human player)
        if not self.__is_square_friendly(square):
            self.__clear_selected_square()
            return

        # Selecting the same square twice deselects it
        if square == self.__selected_square or square is None:
            self.__clear_selected_square()
            return

        self.__selected_square = square
        self.__set_standard_moves(square)
        self.__set_attack_moves(square)

def board_to_game(board):
    game = chess.pgn.Game()

    # Undo all moves.
    switchyard = collections.deque()
    while board.move_stack:
        switchyard.append(board.pop())

    game.setup(board)
    node = game

    # Replay all moves.
    while switchyard:
        move = switchyard.pop()
        node = node.add_variation(move)
        board.push(move)

    game.headers["Result"] = board.result()
    return game

def main():
    if len(sys.argv) > 1:
        if sys.argv[1] == "white":
            state = State(chess.WHITE, False)
        elif sys.argv[1] == "black":
            state = State(chess.BLACK, False)
        elif sys.argv[1] == "ai":
            state = State(chess.WHITE, True)
        else:
            print("Invalid argument")
            return
    else:
        state = State(chess.WHITE, False)

    state.get_board().set_fen("8/8/3k4/4r3/8/5K2/8/8 w - - 0 1")
    width = BORDER_WIDTH * 2 + BOARD_SIZE + TIMER_AREA_WIDTH
    height = BORDER_WIDTH * 2 + BOARD_SIZE
    screen = init_pygame(width, height)

    if len(sys.argv) > 2:
        ip = sys.argv[2]
        port = int(sys.argv[3])
    else:
        ip = '127.0.0.1'
        port = 4321

    minimax_conn = Connection(ip, port)

    '''
    if state.is_players_turn():
        minimax_conn.push_to_queue("init b")
    else:
        minimax_conn.push_to_queue("init w")
    '''

    minimax_conn.push_to_queue("init b fen " + state.get_board().fen())

    waiting_on_ai = False

    try:
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    break
                else:
                    state.next(event)
            
            pmove = state.pop_move_buffer()
            if pmove is not None:
                minimax_conn.push_to_queue('push ' + pmove.uci())
            
            # We need to ask the AI for a move
            if not state.is_players_turn() and not waiting_on_ai:
                minimax_conn.push_to_queue(message='query')
                waiting_on_ai = True
            
            minimax_conn.handle_queue()

            status, result = minimax_conn.receive()

            if not state.is_players_turn() and waiting_on_ai:
                if status:
                    assert(result is not None and result != -1)
                    result = result.decode('utf-8')

                    if "bestmove" in result:
                        result = result.split(' ')[1]
                        state.push_move(result)
                        waiting_on_ai = False

            rerender(screen, state)
    except:
        pgn_game = board_to_game(state.get_board())

        # print pgn game to stdout
        print(pgn_game)
        


# This code will always run as main
main()
