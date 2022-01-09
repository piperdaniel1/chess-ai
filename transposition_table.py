class Entry:
    def __init__(self, key, eval, depth):
        self.key = key
        self.eval = eval
        self.depth = depth

class Transposition_Table:
    def __init__(self):
        self.table = [None] * 10000000
    
    def decode(self, key):
        index = key % len(self.table)
        return self.table[index]
    
    def encode(self, entry : Entry):
        self.table[entry.key % len(self.table)] = entry

    