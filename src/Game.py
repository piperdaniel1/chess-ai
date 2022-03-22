# this will abstract away everything related to pygame
from copy import deepcopy
import pygame
import chess
import time
import pygame
from threading import Thread, Event
import os
import pickle

from python_engine.Minimax import Minimax
from python_engine.Opening_Book import Opening_Book

'''
ChessTimer class

This class will be used to keep track of the time left on a given
player's clock.
'''

# Used for the opening book
class Opening_Entry:
    # Key = zorbist hash of position
    # Next_move = the move the GM used in this position
    def __init__(self, key=0, next_move=0):
        self.key = key
        self.next_move = next_move

'''
Used in the TimerThread class. Represents one of the chess
clocks. Supports adding move bonus, and subtracting individual seconds / tenth of a seconds.
'''
class ChessTimer:
    def __init__(self, minutes : float = 5, seconds : float = 0):
        self.minutes = minutes
        self.seconds = seconds
    
    '''
    Returns a string representation of the clock.
    Used for printing the clock to the GUI.
    '''
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
        return self.minutes * 60 + self.seconds
    
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

'''
Wrapper for two ChessTimer classes (one for black, one for white).
Automatically ticks the active timer every second in the background.
There is only ever one instance of the TimerThread in a given game,
it isn't really the most efficient use of a class.
'''
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

'''
ChessWindow class
An event driven loop that handles all the pygame events.
Lets user take their turn, shows them their legal moves,
and queries the chess engines for their move.
'''
class ChessWindow:
    def __init__(self):
        # Compile the CPP backend of the chess engine
        os.system("g++ -o cpp_engine/a.out cpp_engine/engine.cpp cpp_engine/minimax.cpp cpp_engine/tt_table.cpp cpp_engine/evaluator.cpp cpp_engine/board.cpp cpp_engine/perft_test.cpp")
        with open("output_file.txt", "w") as f:
            f.write("")
        try:
            os.remove("input_file.txt")
        except FileNotFoundError:
            pass
        
        # Spawn a thread to start the CPP engine
        Thread(target=self.start_cpp_engine).start()

        # Initializing the GUI and constants
        pygame.init()
        self.GLOBAL_OFFSET = 25
        self.GRID_SIZE = 900 / 8 - (50 / 8) - 2
        self.LOCAL_OFFSET = -12.5
        self.legal_move = pygame.image.load("assets/pieces/legal-move.png")
        self.legal_move = pygame.transform.scale(self.legal_move, (90, 90))

        # Put the pieces on the board
        self.screen = self.setup_board()
        self.selected_square = None

        # Initialize base classes / start timer
        self.internal_board = chess.Board()
        self.minimax = Minimax()
        self.stop_timer = Event()
        self.timer = TimerThread(self.stop_timer, self.internal_board)
        self.timer.start()


        
        self.moves_made = 0
        self.player_move = True
        self.moves_to_render = []
        self.sent_to_engine = False
        self.opening_book = Opening_Book()

        pickle_book = open('hashed_book.p', 'rb')
        pickle_table = open('hashed_table.p', 'rb')

        self.opening_book.book_tt = pickle.load(pickle_book)
        self.opening_book.zorbist_table = pickle.load(pickle_table)
    
    def start_cpp_engine(self):
        os.system(f'./cpp_engine/a.out')

    '''
    This function gets a move from the python minimax.
    It is called every time it is the minimax's turn to move.
    It returns the move in the from of a Chess.Move object.
    '''
    def send_board_to_engine(self):
        move = self.opening_book.decode(self.opening_book.get_zorbist_hash(self.internal_board))
        print(move)
        
        if move is not None:
            with open("output_file.txt", "w") as f:
                f.write(move.uci())
        else:
            with open("input_file.txt", "w") as f:
                f.write(str(self.internal_board.fen()))

    def check_engine_status(self):
        # check contents of output file
        with open("output_file.txt", "r") as f:
            content = f.read()
        
        return content

    '''
    This function updates the chess board with the current timers.
    This function does not draw the legal moves.
    '''
    def draw_board(self):
        self.screen.fill((255, 255, 255))
        picture = pygame.image.load("assets/pieces/png-versions/board.png")

        self.screen.blit(picture, (0,0))
        map = self.internal_board.piece_map()

        for key in range(64):
            try:
                piece = map[key]
                self.draw_piece(key, piece)
            except KeyError:
                pass

        font = pygame.font.SysFont('Consolas', 100)

        if self.internal_board.turn == True:
            self.screen.blit(font.render(self.timer.black_clock.__str__(), True, (0, 0, 0)), (920, 300))
            pygame.draw.line(self.screen, (0, 0, 0), (920, 450), (1330, 450), 5)
            self.screen.blit(font.render(self.timer.white_clock.__str__(), True, (0, 125, 0)), (920, 500))
        else:
            self.screen.blit(font.render(self.timer.black_clock.__str__(), True, (0, 125, 0)), (920, 300))
            pygame.draw.line(self.screen, (0, 0, 0), (920, 450), (1330, 450), 5)
            self.screen.blit(font.render(self.timer.white_clock.__str__(), True, (0, 0, 0)), (920, 500))

    '''
    Converts from my weird coordinate system to chess-python's even weirder system.
    '''
    def convert_gridpos_to_chesspos(self, grid_pos):
        row, col = grid_pos

        return (row * 8) + col
    
    '''
    Converts from chess-python's weird coordinate system to my slightly less weird coordinate system.
    '''
    def convert_chesspos_to_gridpos(self, chess_pos):
        row = chess_pos % 8
        col = chess_pos // 8

        return (row, col)

    '''
    Removes legal moves that do not originate from a given
    target square. This is used to display only moves
    for a given piece.
    '''
    def trim_moves(self, moves, target_square : int):
        c = 0
        while c < len(moves): 
            if moves[c].from_square != target_square:
                moves.pop(c)
                c -= 1
            
            c += 1
    '''
    Blits legal moves onto the board.
    Doesn't flip the pygame display, this is done
    somewhere else.
    '''
    def render_valid_moves(self, moves_to_render):
        for move in moves_to_render:
            key = move.to_square
            gridpos = self.convert_chesspos_to_gridpos(key)
            pixelpos = self.convert_grid_to_pixel(gridpos)
            self.screen.blit(self.legal_move, pixelpos)
    
    '''
    Big while loop that handles all the pygame events.
    The program executes inside this loop while the
    user is playing the game.
    '''
    def run_game(self):
        print(self.moves_to_render)

        running = False
        self.player_move = self.internal_board.turn

        while running == False:
            self.draw_board()
            self.render_valid_moves(self.moves_to_render)
            pygame.display.flip()

            if not self.player_move:
                if not self.sent_to_engine:
                    with open("output_file.txt", "w") as f:
                        f.write("")
                    Thread(target=self.send_board_to_engine).start()
                    self.sent_to_engine = True

                if self.timer.black_clock.get_seconds_remaining() <= 0:
                    running = True
                    pygame.quit()
                    self.timer.stopped = True
                    print("Game over. You won because black ran out of time!")
                    quit()

                status = self.check_engine_status()                                
                if status != "" and self.sent_to_engine == True:
                    self.sent_to_engine = False
                    self.internal_board.push_uci(status)
                    self.player_move = True
                    self.timer.black_clock.apply_move_bonus(self.timer.move_bonus)
                    self.timer.turn = True
                    self.moves_made += 1

                    print("current fen: ", self.internal_board.fen())
                    if self.minimax.eval.get_score_of_board(self.internal_board, verbose=True) in [-1000, 1000]:
                        print("Looks like the game is over.")
                        print("Here's the move stack in case you want to look back:")
                        print(self.internal_board.move_stack)
                        print(self.internal_board)
                        print(self.internal_board.fen())
            
            if self.timer.white_clock.get_seconds_remaining() <= 0:
                running = True
                pygame.quit()
                self.timer.stopped = True
                print("Game over. Black won because you ran out of time!")
                quit()
   
            for event in pygame.event.get():
                if event.type == pygame.MOUSEBUTTONUP and self.player_move == True:
                    mouse_pos = pygame.mouse.get_pos()
                    try:
                        grid_pos = self.convert_pixel_to_grid(mouse_pos)
                        chess_pos = self.convert_gridpos_to_chesspos(grid_pos)
                    except TypeError:
                        continue
                    
                    try:
                        piece_at = self.internal_board.piece_at(chess_pos)
                    except IndexError:
                        continue

                    if self.selected_square is None:
                        if piece_at is not None:
                            self.selected_square = chess_pos
                            moves = list(self.internal_board.legal_moves)
                            self.trim_moves(moves, chess_pos)
                            if len(moves) == 0:
                                self.selected_square = None
                            self.moves_to_render = moves
                        else:
                            self.moves_to_render = []
                            self.draw_board()
                            pygame.display.flip()
                    else:
                        if piece_at is not None and piece_at.color == self.internal_board.piece_at(self.selected_square).color:
                            self.selected_square = chess_pos
                            moves = list(self.internal_board.legal_moves)
                            self.trim_moves(moves, chess_pos)
                            self.moves_to_render = moves
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
                            self.moves_to_render = []
                if event.type == pygame.QUIT:
                    running = True
                    pygame.quit()
                    self.timer.stopped = True
                    quit()
    
    '''
    Converts from a pixel position to grid position.
    Great for converting mouse clicks to grid positions.
    '''
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

    '''
    Draws a piece onto the board.
    Takes in a key which is a chess position.
    '''
    def draw_piece(self, key, piece):
        row = key % 8
        col = key // 8

        if str(piece).upper() == str(piece):
            piece = pygame.image.load(f"assets/pieces/png-versions/{piece}-white.png")
        else:
            piece = pygame.image.load(f"assets/pieces/png-versions/{piece}-black.png")
        piece = pygame.transform.scale(piece, (90, 90))
        self.screen.blit(piece, self.convert_grid_to_pixel((row, col)))

    '''
    Converts from a grid position to a pixel position.
    Great for drawing pieces onto the board.
    '''
    def convert_grid_to_pixel(self, grid_pos):
        """
        Converts grid position to pixel position on screen
        """
        row, col = grid_pos
        col = 7 - col
        base_row = row * self.GRID_SIZE
        base_col = col * self.GRID_SIZE

        return (base_row - self.LOCAL_OFFSET + self.GLOBAL_OFFSET, base_col - self.LOCAL_OFFSET + self.GLOBAL_OFFSET)

    '''
    Draws the board onto the screen.
    '''
    def setup_board(self):
        background_colour = (255,255,255)
        (width, height) = (1350, 900)
        screen = pygame.display.set_mode((width, height))
        pygame.display.set_caption('chess-ai')
        screen.fill(background_colour)

        return screen

if __name__ == "__main__":
    window = ChessWindow() # Initializes the window
    # set default params this is bad practice
    window.minimax.dump_minimax_tree = False
    window.minimax.move_chaining = False
    window.minimax.MAX_SECONDS = 15

    # set up the timer also bad practice
    window.timer.white_clock.minutes = 10
    window.timer.black_clock.minutes = 10
    window.timer.white_clock.seconds = 0
    window.timer.black_clock.seconds = 0
    window.timer.move_bonus = 0
    window.timer.turn = window.internal_board.turn

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

    with open("input_file.txt", "w") as f:
        f.write("quit")

    pygame.quit()
    window.timer.stopped = True
    quit()