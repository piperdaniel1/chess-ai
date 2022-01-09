# this will abstract away everything related to pygame
import pygame
import chess
import time
import pygame
import threading

from Minimax import Minimax
from Board_Scorer import Board_Scorer

class ChessWindow:
    def __init__(self):
        # everyone loves magic numbers 
        self.GLOBAL_OFFSET = 25
        self.GRID_SIZE = 900 / 8 - (50 / 8) - 2
        self.LOCAL_OFFSET = -12.5
        self.screen = self.setup_board()
        self.selected_square = None
        self.internal_board = chess.Board()
        self.minimax = Minimax()
        self.legal_move = pygame.image.load("pieces/legal-move.png")
        self.legal_move = pygame.transform.scale(self.legal_move, (90, 90))
        self.moves_made = 0
        self.player_move = True
    
    def get_move_from_minimax(self, is_white):
        curr_time = time.time()
        end_time = 0
        depth = 4
        
        while True:
            self.minimax.max_depth = depth

            educated_move = self.minimax.find_best_move(self.internal_board, depth, False, -1000, 1000, self.moves_made)[0]
            end_time = time.time()
            print(f"Found the move {educated_move.uci()} in {round(end_time - curr_time, 1)} seconds. (d={depth})                   ")
            depth += 1

            if (end_time - curr_time) * 18 > 30:
                break

        return educated_move

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
    
    def convert_gridpos_to_chesspos(self, grid_pos):
        row, col = grid_pos

        return (row * 8) + col
    
    def convert_chesspos_to_gridpos(self, chess_pos):
        row = chess_pos % 8
        col = chess_pos // 8

        return (row, col)

    def trim_moves(self, moves, target_square : int):
        c = 0
        while c < len(moves): 
            if moves[c].from_square != target_square:
                moves.pop(c)
                c -= 1
            
            c += 1
        
    def render_valid_moves(self, moves_to_render):
        for move in moves_to_render:
            key = move.to_square
            gridpos = self.convert_chesspos_to_gridpos(key)
            pixelpos = self.convert_grid_to_pixel(gridpos)
            self.screen.blit(self.legal_move, pixelpos)
    
    def run_game(self):
        self.draw_board()
        pygame.display.flip()

        running = False

        while running == False:
            if not self.player_move:
                self.minimax.tt.successful_uses = 0
                move = self.get_move_from_minimax(True)
                print(f"The transposition table has been used {self.minimax.tt.successful_uses} times. {self.minimax.tt.indexes_used} total entries.")
                self.internal_board.push(move)
                self.draw_board()
                pygame.display.flip()
                self.player_move = True
                self.moves_made += 1
                
            for event in pygame.event.get():
                if event.type == pygame.MOUSEBUTTONUP and self.player_move == True:
                    mouse_pos = pygame.mouse.get_pos()
                    grid_pos = self.convert_pixel_to_grid(mouse_pos)
                    chess_pos = self.convert_gridpos_to_chesspos(grid_pos)
                    piece_at = self.internal_board.piece_at(chess_pos)

                    # case one:
                    # selected_square is None
                    #   if no piece was clicked board is refreshed with no change
                    #   if piece was clicked legal moves are calculated and displayed
                    # selected_square is not None
                    #   - if a piece was clicked and it is the same piece color as the selected_square piece then
                    #     change selected_square to this new piece and rerender the board with those moves
                    #   - if anything else is selected then check if that is a legal move for the piece that is on the selected_square
                    #       - if it is, execute that move on the board and redisplay.
                    #       - if it is not, set selected_square to None and redisplay.

                    if self.selected_square is None:
                        if piece_at is not None:
                            self.selected_square = chess_pos
                            moves = list(self.internal_board.legal_moves)
                            self.trim_moves(moves, chess_pos)
                            if len(moves) == 0:
                                self.selected_square = None
                            self.draw_board()
                            self.render_valid_moves(moves)
                            pygame.display.flip()
                        else:
                            self.draw_board()
                            pygame.display.flip()
                    else:
                        if piece_at is not None and piece_at.color == self.internal_board.piece_at(self.selected_square).color:
                            self.selected_square = chess_pos
                            moves = list(self.internal_board.legal_moves)
                            self.trim_moves(moves, chess_pos)
                            self.draw_board()
                            self.render_valid_moves(moves)
                            pygame.display.flip()
                        else:
                            moves = list(self.internal_board.legal_moves)
                            self.trim_moves(moves, self.selected_square)

                            for move in moves:
                                if move.to_square == chess_pos:
                                    self.internal_board.push(move)
                                    self.player_move = not self.player_move
                                    self.moves_made += 1
                                    break
                            
                            self.selected_square = None
                            self.draw_board()
                            pygame.display.flip()
                            print("potential new move")
                if event.type == pygame.QUIT:
                    running = True
                    pygame.quit()
                    quit()
    
    def convert_pixel_to_grid(self, pixel_pos):
        """
        Converts pixel position to grid position
        """
        x, y = pixel_pos
        y = 900 - y

        if y < self.GLOBAL_OFFSET:
            return None
        if x < self.GLOBAL_OFFSET:
            return None

        row = int((y - self.GLOBAL_OFFSET) / self.GRID_SIZE)
        col = int((x - self.GLOBAL_OFFSET) / self.GRID_SIZE)

        return (row, col)

    def draw_piece(self, key, piece):
        row = key % 8
        col = key // 8

        piece = pygame.image.load(f"pieces/png-versions/{piece}.png")
        piece = pygame.transform.scale(piece, (90, 90))
        self.screen.blit(piece, self.convert_grid_to_pixel((row, col)))

    def convert_grid_to_pixel(self, grid_pos):
        """
        Converts grid position to pixel position on screen
        """
        row, col = grid_pos
        col = 7 - col
        base_row = row * self.GRID_SIZE
        base_col = col * self.GRID_SIZE

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
window.run_game()