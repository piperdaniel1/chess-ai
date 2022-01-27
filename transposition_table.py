import chess

class Entry:
    def __init__(self, key, eval_, depth, board):
        self.key = key
        self.eval = eval_
        self.depth = depth
        self.board = board
    
    def __str__(self):
        return f"Entry(key: {self.key}, eval: {self.eval}, depth: {self.depth}\n{self.board}"

class Transposition_Table:
    def __init__(self):
        self.table = [None] * 10000000

    def decode(self, key, board):
        index = key % len(self.table)
        if self.table[index] != None:
            if self.table[index].key == key:
                if str(self.table[index].board) == str(board):
                    return self.table[index]
    
    def encode(self, entry : Entry):
        self.table[entry.key % len(self.table)] = entry