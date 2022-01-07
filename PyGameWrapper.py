# this will abstract away everything related to pygame
import pygame
import chess
import time
import pygame
import threading

class ChessWindow:
    def __init__(self):
        # everyone loves magic numbers 
        self.GLOBAL_OFFSET = 25
        self.GRID_SIZE = 900 / 8 - (50 / 8) - 2
        self.LOCAL_OFFSET = -12.5
        self.screen = self.setup_board()
        self.internal_board = chess.Board()

    def draw_board(self):
        picture = pygame.image.load("pieces/png-versions/board.png")
        self.screen.blit(picture, (0,0))
        map = self.internal_board.piece_map()

        for key in range(64):
            try:
                piece = map[key]
                self.draw_piece(key, piece)
            except KeyError:
                pass
        
        pygame.display.flip()
        
        running = False
        while not running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = True
                    pygame.quit()
                    quit()  
    
    def draw_piece(self, key, piece):
        row = key // 8
        col = key % 8

        piece = pygame.image.load(f"pieces/png-versions/{piece}.png")
        piece = pygame.transform.scale(piece, (90, 90))
        self.screen.blit(piece, self.convert_grid_to_pixel((row, col)))

    def convert_grid_to_pixel(self, grid_pos):
        """
        Converts grid position to pixel position on screen
        """
        row, col = grid_pos
        base_row = row * self.GRID_SIZE
        base_col = col * self.GRID_SIZE

        print("base row pos:", base_row)
        print("base col pos:", base_col)

        print("local offset:", self.LOCAL_OFFSET)
        print("global offset:", self.GLOBAL_OFFSET)

        print(f"final calculated row is {base_row} - {self.LOCAL_OFFSET} + {self.GLOBAL_OFFSET} = {base_row - self.LOCAL_OFFSET + self.GLOBAL_OFFSET}")

        return (base_row - self.LOCAL_OFFSET + self.GLOBAL_OFFSET, base_col - self.LOCAL_OFFSET + self.GLOBAL_OFFSET)

    def setup_board(self):
        background_colour = (255,255,255)
        (width, height) = (900, 900)
        screen = pygame.display.set_mode((width, height))
        pygame.display.set_caption('chess-ai')
        screen.fill(background_colour)

        return screen

    def board_test(self):
        for row in range(8):
            for col in range(8):
                picture = pygame.image.load("pieces/png-versions/board.png")
                self.screen.blit(picture, (0,0))

                white_pawn = pygame.image.load("pieces/png-versions/white-pawn.png")
                white_pawn = pygame.transform.scale(white_pawn, (90, 90))
                self.screen.blit(white_pawn, self.convert_grid_to_pixel((row, col)))
                pygame.display.flip()

                time.sleep(500 / 1000)

window = ChessWindow()
window.draw_board()