import pickle
import chess

class Opening_Entry:
    def __init__(self, key=0, next_move=0):
        self.key = key
        self.next_move = next_move

class Opening_Book:
    def __init__(self, book_file = "millionbase-2.5.pgn"):
        self.book_file = book_file
        self.book_tt = [None] * 2000000
    
    def add_move_to_board(self, board, move_str):
        pass

    def add_position_to_book(self, board, next_move):
        pass

    def interpret_move_line(self, board, line : str):
        current_char = 0
        saved_pos = False
        move_one = False
        
        split_line = line.split()

        for elem in split_line:
            if elem[0].isdigit():
                # TODO check if we are out of the opening
                print("resetting...")
                saved_pos = False
                move_one = False
                continue
                
            if saved_pos == False:
                if move_one == False:
                    move_one_str = elem
                    self.add_move_to_board(board, move_one_str)
                    print("move_one_str: " + move_one_str)
                    move_one = True
                else:
                    move_two_str = elem
                    self.add_position_to_book(board, move_two_str)
                    self.add_move_to_board(board, move_two_str)
                    print("move_two_str: " + move_two_str)
            

    def dump_book_to_hash(self):
        #mode can be 'discovery'
        mode = "discovery"
        with open(self.book_file, 'r') as f:
            entry = Opening_Entry()
            board = chess.Board()
            while True:
                curr_line = f.readline()

                # skip until next event
                if mode == "skipping" and '[Event "' not in curr_line:
                    continue
                elif mode == "skipping" and '[Event "' in curr_line:
                    mode = "discovery"
                    continue

                # skip empty lines
                if curr_line == "\n":
                    continue

                # discover game, skip if neccessary
                if mode == "discovery":
                    if '[Black Elo ' in curr_line or '[White Elo ' in curr_line:
                        if int(curr_line[11:14]) < 2200:
                            mode = "skipping"
                            continue
                    try:
                        int(curr_line[0])
                        board = chess.Board()

                        
                        # extract moves from game
                        while True:
                            status = self.interpret_move_line(board, curr_line)

                            if status == -1:
                                mode = "skipping"
                                break

                            f.readline()
                    except ValueError:
                        continue

                input("Press Enter to continue...")
    
    def decode(self, key):
        index = key % len(self.table)
        if self.table[index] != None:
            if self.table[index].key == key:
                return self.table[index]
    
    def encode(self, entry : Opening_Entry):
        self.table[entry.key % len(self.table)] = entry

if __name__ == "__main__":
    book = Opening_Book()
    book.dump_book_to_hash()
