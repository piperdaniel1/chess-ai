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

board = chess.Board("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
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