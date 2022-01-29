from os import remove
import pickle
import chess
import random

class Opening_Entry:
    def __init__(self, key=0, next_move=0):
        self.key = key
        self.next_move = next_move

class Opening_Book:
    def __init__(self, book_file = "millionbase-2.5.pgn"):
        self.book_file = book_file
        '''try:
            pickle_book = open('hashed_book.p', 'rb')
            pickle_table = open('hashed_table.p', 'rb')

            self.book_tt = pickle.load(pickle_book)
            self.zorbist_table = pickle.load(pickle_table)

            print("Opening book loaded from file")
        except FileNotFoundError:
            print("Opening book not loaded from file")
            self.book_tt = [[None]] * 2000000
            self.zorbist_table = [[random.randint(1,2**64 - 1) for i in range(12)] for j in range(64)]'''

    def get_zorbist_hash(self, board):
        piece_map = board.piece_map()
        zorbist_hash = 0

        for key in range(64):
            try:
                piece = piece_map[key]
                piece_index = self.get_index_of_piece(piece.symbol())
                zorbist_hash ^= self.zorbist_table[key][piece_index]
            except KeyError:
                pass
        
        return zorbist_hash
    
    def get_index_of_piece(self, piece):
        if (piece=='P'):
            return 0
        if (piece=='N'):
            return 1
        if (piece=='B'):
            return 2
        if (piece=='R'):
            return 3
        if (piece=='Q'):
            return 4
        if (piece=='K'):
            return 5
        if (piece=='p'):
            return 6
        if (piece=='n'):
            return 7
        if (piece=='b'):
            return 8
        if (piece=='r'):
            return 9
        if (piece=='q'):
            return 10
        if (piece=='k'):
            return 11
        else:
            return -1

    def add_move_to_board(self, board : chess.Board, move_str : str):
        all_legal = list(board.legal_moves)
        piece = None
        castling = False
        # remove capture from string
        move_str = move_str.replace("x", "")
        # remove check from string
        move_str = move_str.replace("+", "")
        
        if len(move_str) == 2:
            to_square = move_str
        elif len(move_str) == 3 and move_str == "O-O":
            castling = True
        elif len(move_str) == 3:
            piece = move_str[0]
            if not board.turn:
                piece = piece.lower()
            to_square = move_str[1:len(move_str)]  
        else:
            return -1      

        for move in all_legal:
            if castling == True:
                if board.is_castling(move):
                    board.push(move)
                    return move
                else:
                    continue

            m_from_square = str(move)[0:2]
            m_to_square = str(move)[2:4]
            
            if piece == None and to_square == m_to_square:
                board.push(move)
                return move
            elif piece != None and to_square == m_to_square:
                if(board.piece_at(chess.SQUARE_NAMES.index(m_from_square)).symbol() == piece):
                    board.push(move)
                    return move

        return -1

    def add_position_to_book(self, board : chess.Board, next_move : str):
        next_move = self.add_move_to_board(board, next_move)
        if next_move != -1:
            board.pop()

            hash = self.get_zorbist_hash(board)
            self.encode(Opening_Entry(hash, next_move))

    def interpret_move_line(self, board, line : str):
        current_char = 0
        saved_pos = False
        move_one = False
        
        split_line = line.split()

        for elem in split_line:
            if elem[0].isdigit():
                # TODO check if we are out of the opening
                if elem[1].isdigit():
                    return -1
                saved_pos = False
                move_one = False
                continue
                
            if saved_pos == False:
                if move_one == False:
                    move_one_str = elem
                    self.add_move_to_board(board, move_one_str)
                    move_one = True
                else:
                    move_two_str = elem
                    self.add_position_to_book(board, move_two_str)
                    self.add_move_to_board(board, move_two_str)
        return 0

    def dump_book_to_hash(self, endl=10000000000000000):
        #mode can be 'discovery'
        mode = "discovery"
        with open(self.book_file, 'r') as f:
            entry = Opening_Entry()
            board = chess.Board()
            line = 0
            skip = False
            while line < endl:
                try:
                    try:
                        curr_line = f.readline()

                        if curr_line == '':
                            break
                        line += 1
                        if line % 500 == 0:
                            print(f"Reading line {line}...")

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

                                    curr_line = f.readline()
                            except ValueError:
                                continue
                    except Exception:
                        print("Error in line " + str(line))
                        mode = 'skipping'
                        continue
                except KeyboardInterrupt:
                    print("Caught keyboard interrupt, non-destructively exiting...")
                    break
    
    def decode(self, key) -> chess.Move:        
        index = key % len(self.book_tt)
        if self.book_tt[index][0] != None:
            random.shuffle(self.book_tt[index])

            for response in self.book_tt[index]:
                if response.key == key:
                    return response.next_move
        return None
    
    def encode(self, entry : Opening_Entry):
        if self.book_tt[entry.key % len(self.book_tt)][0] == None:
            self.book_tt[entry.key % len(self.book_tt)][0] = entry
        else:
            for response in self.book_tt[entry.key % len(self.book_tt)]:
                if response.key == entry.key:
                    if response.next_move == entry.next_move:
                        return
            
            self.book_tt[entry.key % len(self.book_tt)].append(entry)

if __name__ == "__main__":
    book = Opening_Book()
    book.dump_book_to_hash()
    hashed_book = open('hashed_book.p', 'wb')
    hashed_table = open('hashed_table.p', 'wb')
    pickle.dump(book.book_tt, hashed_book)
    pickle.dump(book.zorbist_table, hashed_table)
    hashed_book.close()
    hashed_table.close()