class Entry:
    def __init__(self, key, eval_, depth):
        self.key = key
        self.eval = eval_
        self.depth = depth

class Transposition_Table:
    def __init__(self):
        self.table = [None] * 40000000

    def decode(self, key):
        index = key % len(self.table)
        if self.table[index] != None:
            if self.table[index].key == key:
                return self.table[index]
    
    def encode(self, entry : Entry):
        self.table[entry.key % len(self.table)] = entry

    