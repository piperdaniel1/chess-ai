class Entry:
    def __init__(self, key, eval_, depth):
        self.key = key
        self.eval = eval_
        self.depth = depth

class Transposition_Table:
    def __init__(self):
        self.table = [None] * 10000000
        self.successful_uses = 0
        self.indexes_used = 0
    
    def decode(self, key):
        index = key % len(self.table)
        if self.table[index] != None:
            if self.table[index].key == key:
                self.successful_uses += 1
                return self.table[index]
    
    def encode(self, entry : Entry):
        if self.table[entry.key % len(self.table)] == None:
            self.indexes_used += 1
        self.table[entry.key % len(self.table)] = entry

    