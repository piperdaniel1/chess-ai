class Entry:
    def __init__(self, key, eval_, depth, move_chain):
        self.key = key
        self.eval = eval_
        self.depth = depth
        self.move_chain = move_chain

class Transposition_Table:
    def __init__(self):
        self.table = [None] * 10000000

    
    def decode(self, key):
        index = key % len(self.table)
        if self.table[index] != None:
            if self.table[index].key == key:
                return self.table[index]
    
    def encode(self, entry : Entry):
        self.table[entry.key % len(self.table)] = entry

    