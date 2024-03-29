# this will abstract away everything related to pygame
import pygame
import chess
import time
from threading import Thread, Event
from rust_minimax import RustMinimax

# The ChessTimer class is used to keep track of the time left for one player
# Abstracts away the details of keeping track of minutes and seconds
class ChessTimer:
    def __init__(self, minutes : float = 5, seconds : float = 0):
        self.minutes = minutes
        self.seconds = seconds
    
    def __str__(self):
        self.minutes = int(self.minutes)
        self.seconds = round(self.seconds, 1)
        if self.minutes >= 10 and self.seconds >= 10:
            return "{}:{}".format(self.minutes, self.seconds)
        elif self.minutes < 10 and self.seconds >= 10:
            return "0{}:{}".format(self.minutes, self.seconds)
        elif self.minutes >= 10 and self.seconds < 10:
            return "{}:0{}".format(self.minutes, self.seconds)
        elif self.minutes < 10 and self.seconds < 10:
            return "0{}:0{}".format(self.minutes, self.seconds)

    def get_seconds_remaining(self):
        return int(self.minutes * 60 + self.seconds)
    
    def tick(self):
        self.seconds -= 1
        if self.seconds == -1:
            self.seconds = 59
            self.minutes -= 1
    
    def tick_accurately(self):
        self.seconds -= 0.1
        if self.seconds < 0:
            self.seconds = 59
            self.minutes -= 1

    def apply_move_bonus(self, move_bonus : float):
        self.seconds += move_bonus
        if self.seconds > 59:
            self.seconds -= 60
            self.minutes += 1

# This class runs two timers, one for each player
# Makes sure to keep track of whose turn it is and to switch the timers
# when the turn changes
class TimerThread(Thread):
    def __init__(self, event, board: chess.Board, mins = 5, secs = 0):
        Thread.__init__(self)
        self.stopped = False
        self.board = board
        self.turn = True
        self.white_clock = ChessTimer(mins, secs)
        self.black_clock = ChessTimer(mins, secs)
        self.move_bonus = 0

    def run(self):
        while self.stopped == False:
            time.sleep(0.1)

            if self.turn == True:
                self.white_clock.tick_accurately()
            else:
                self.black_clock.tick_accurately()

# Master class to print the game to the screen and handle user input / minimax
class ChessWindow:
    def __init__(self):
        # everyone loves magic numbers 
        pygame.init()
        self.GLOBAL_OFFSET = 25
        self.GRID_SIZE = 900 / 8 - (50 / 8) - 2
        self.LOCAL_OFFSET = -12.5

        # Setup the pygame window and other assets
        self.screen = self.setup_board()
        self.legal_move = pygame.image.load("pieces/legal-move.png")
        self.legal_move = pygame.transform.scale(self.legal_move, (90, 90))

        # Initialize the internal board state self.selected_square = None
        self.internal_board = chess.Board()
        self.moves_made = 0
        self.player_move = True

        # Initialize the minimax algorithm
        self.rust_minimax = RustMinimax()

        # Initialize the thread for the timer
        self.stop_timer = Event()
        self.timer = TimerThread(self.stop_timer, self.internal_board)
        self.timer.start()
    
    # Returns the move that the minimax algorithm has determined is the best move
    def get_move_from_minimax(self, is_white):
        # Capture the current starting time
        curr_time = time.time()
        end_time = 0
        depth = 4
        
        # Run the minimax algorithm on the current board
        move = self.rust_minimax.get_move(self.internal_board)

        # Capture the end time
        end_time = time.time()

        # This code displays the move chain that the minimax algorithm thinks is
        # going to happen
        print(f"Found the move {move} in {round(end_time - curr_time, 1)} seconds.                 ")

        # Debug mode that allows us to see the entire minimax tree
        return move

    # Blits the board to the screen using the pygame library
    def draw_board(self):
        # This seems horrifically inefficient in retrospect, especially loading
        # the image for the board every time
        self.screen.fill((255, 255, 255))
        picture = pygame.image.load("pieces/png-versions/board.png")

        self.screen.blit(picture, (0,0))
        map = self.internal_board.piece_map()

        # Draw the pieces on the board
        for key in range(64):
            try:
                piece = map[key]
                self.draw_piece(key, piece)
            except KeyError:
                pass

        # Draw the chess clock
        font = pygame.font.SysFont('Consolas', 100)

        black_clock_str = self.timer.black_clock.__str__()
        white_clock_str = self.timer.white_clock.__str__()

        assert(black_clock_str is not None)
        assert(white_clock_str is not None)

        if self.internal_board.turn == True:
            self.screen.blit(font.render(black_clock_str, True, (0, 0, 0)), (920, 300))
            pygame.draw.line(self.screen, (0, 0, 0), (920, 450), (1330, 450), 5)
            self.screen.blit(font.render(white_clock_str, True, (0, 125, 0)), (920, 500))
        else:
            self.screen.blit(font.render(black_clock_str, True, (0, 125, 0)), (920, 300))
            pygame.draw.line(self.screen, (0, 0, 0), (920, 450), (1330, 450), 5)
            self.screen.blit(font.render(white_clock_str, True, (0, 0, 0)), (920, 500))

    # Simple helper functions to convert between chess and grid positions
    def convert_gridpos_to_chesspos(self, grid_pos):
        row, col = grid_pos

        return (row * 8) + col
    def convert_chesspos_to_gridpos(self, chess_pos):
        row = chess_pos % 8
        col = chess_pos // 8

        return (row, col)

    # R
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
        self.player_move = True
        # rn1qkbnr/ppp2p1p/3pp1p1/8/2PP2b1/1P3N1P/P3PPP1/RNBQKB1R bishop does not avoid capture? (fixed)
        # 4k2r/p4ppp/n1p1pn2/q2pN3/P2P4/b1P1P3/3N1PPP/1Q3RK1 king does not castle?
        #self.internal_board.set_fen("4k2r/p1q2ppp/n1pbpn2/P2pN3/3P4/1QP1P3/3N1PPP/R5K1 b k - 0 1")
        #self.internal_board.set_castling_fen("k")
        #self.internal_board.turn = chess.BLACK

        self.draw_board()
        pygame.display.flip()

        while running == False:
            # Run the minimax algorithm if it is the computer's turn
            if not self.player_move:
                move = self.get_move_from_minimax(True)

                if self.timer.black_clock.get_seconds_remaining() <= 0:
                    running = True
                    pygame.quit()
                    self.timer.stopped = True
                    print("Game over. You won because black ran out of time!")
                    quit()

                assert(move is not None)

                self.internal_board.push(move)
                self.timer.black_clock.apply_move_bonus(self.timer.move_bonus)
                self.timer.turn = True
                self.draw_board()
                pygame.display.flip()
                self.player_move = True
                self.moves_made += 1

                print("There are " + str(len(list(self.internal_board.legal_moves))) + " legal moves in this position.")
                
                print("current FEN:", self.internal_board.fen())

                print("")

                if self.internal_board.is_checkmate() \
                        or self.internal_board.is_stalemate() or \
                        self.internal_board.is_insufficient_material():
                    print("Looks like the game is over.")
                    print("Here's the move stack in case you want to look back:")
                    print(self.internal_board.move_stack)
                    print(self.internal_board)
                    print(self.internal_board.fen())
            
            # Exit the game if the player has run out of time
            if self.timer.white_clock.get_seconds_remaining() <= 0:
                running = True
                pygame.quit()
                self.timer.stopped = True
                print("Game over. Black won because you ran out of time!")
                quit()

            if self.selected_square != None:
                moves = list(self.internal_board.legal_moves)
                self.trim_moves(moves, self.selected_square)
                if len(moves) == 0:
                    self.selected_square = None
                self.draw_board()
                self.render_valid_moves(moves)
                pygame.display.flip()
            else:
                self.draw_board()
                pygame.display.flip()
   
            for event in pygame.event.get():
                if event.type == pygame.MOUSEBUTTONUP and self.player_move == True:
                    mouse_pos = pygame.mouse.get_pos()
                    try:
                        grid_pos = self.convert_pixel_to_grid(mouse_pos)
                        chess_pos = self.convert_gridpos_to_chesspos(grid_pos)
                    except TypeError:
                        continue
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

                                    if self.internal_board.is_checkmate():
                                        running = True
                                        pygame.quit()
                                        self.timer.stopped = True
                                        print("Game over. You won!")
                                        quit()
                                    self.timer.turn = False
                                    self.timer.white_clock.apply_move_bonus(self.timer.move_bonus)
                                    self.player_move = not self.player_move
                                    self.moves_made += 1
                                    break
                            
                            self.selected_square = None
                            self.draw_board()
                            pygame.display.flip()
                if event.type == pygame.QUIT:
                    running = True
                    pygame.quit()
                    self.timer.stopped = True
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

        if str(piece).upper() == str(piece):
            piece = pygame.image.load(f"pieces/png-versions/{piece}-white.png")
        else:
            piece = pygame.image.load(f"pieces/png-versions/{piece}-black.png")
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
        (width, height) = (1350, 900)
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

if __name__ == "__main__":
    window = ChessWindow()
    window.internal_board.set_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    window.timer.white_clock.minutes = 10
    window.timer.black_clock.minutes = 10
    window.timer.white_clock.seconds = 0
    window.timer.black_clock.seconds = 0
    window.timer.move_bonus = 0

    window.selected_square = None

    try:
        window.run_game()
    except Exception:
        print("Oh no... looks like we hit an unhandled exception.")
        print("Current board:")
        print(window.internal_board)
        print("Current move stack:")
        print(window.internal_board.move_stack)
        print("FEN:", window.internal_board.fen())

        raise Exception

    pygame.quit()
    window.timer.stopped = True
    quit()
