import pygame
import chess
from socket_interface import send_message
from typing import List, Union

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
TIMER_AREA_WIDTH = 500

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

def rerender(screen, piece_list : Union[List[tuple], None] = None, sel_squares : Union[List[tuple], None] = None, move_options : Union[List[tuple], None] = None):
    screen.fill((255, 255, 255))

    render_board_squares(screen)
    render_board_border(screen)

    render_selected_square(screen, (1, 4))
    render_selected_square(screen, (2, 4))

    render_move_option(screen, (4, 4))
    render_move_option(screen, (4, 5))

    render_check(screen, (4, 0))
    render_check(screen, (4, 7))

    render_capture_option(screen, (6, 6))
    render_capture_option(screen, (7, 6))

    if piece_list is not None:
        for piece, square in piece_list:
            render_piece(screen, PIECE_MAP[piece], square)

    pygame.display.flip()

def handle_event(event):
    print(event)

class Game:
    def __init__(self):
        self.board = chess.Board()
        pass

    def __conv_square(self, square):
        return (square % 8, (63 - square) // 8)

    def get_piece_list(self):
        pieces = list(self.board.piece_map().values())
        pieces = [piece.symbol() for piece in pieces]
        squares = list(self.board.piece_map().keys())
        squares = [self.__conv_square(square) for square in squares]

        return list(zip(pieces, squares))

def main():
    game = Game()
    print(game.get_piece_list())
    width = BORDER_WIDTH * 2 + BOARD_SIZE + TIMER_AREA_WIDTH
    height = BORDER_WIDTH * 2 + BOARD_SIZE
    screen = init_pygame(width, height)

    while True:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                break
            else:
                handle_event(event)

        rerender(screen, game.get_piece_list())

# This code will always run as main
main()