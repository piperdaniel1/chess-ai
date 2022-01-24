import chess
import os

def perft(board, depth):
    if depth == 0:
        return 1
    else:
        icount = 0
        count = 0

        if len(list(board.legal_moves)) == 0:
            return 1

        for move in board.legal_moves:
            board.push(move)

            if depth == MAX_DEPTH:
                icount = count
                count += perft(board, depth - 1)
                if len(str(move)) == 4:
                    print(str(move) + ": ", count - icount)
                else:
                    print(str(move) + ":", count - icount)
            else:
                count += perft(board, depth - 1)

            board.pop()
        return count

board = chess.Board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0")
MAX_DEPTH = int(input("Enter depth: "))
depth = MAX_DEPTH


try:
    while True:
        os.system('clear')
        print(f"Perft Results for depth {depth}:")
        print("Total:", perft(board, depth))
        move = input("Enter a move fen to traverse downward: ")

        move = chess.Move.from_uci(move)

        board.push(move)
        depth -= 1
        MAX_DEPTH = depth
        print()
except KeyboardInterrupt:
    print("\nExiting...")
    print(board.move_stack)
    exit()