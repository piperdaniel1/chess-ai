from typing import Union
from chess import Board, Move

import subprocess

class RustMinimax:
    def __init__(self):
        pass

    # Returns a move. Takes a python chess Board.
    # Optionally takes a depth, color, and time remaining in seconds.
    def get_move(self, board: Board, is_white = False,\
            depth = 4, time_remaining = -1) -> Union[Move, None]:

        # Convert board to FEN
        fen = board.fen()

        # Call rust minimax
        cmd = ["./rust_minimax", fen, str(depth), str(time_remaining)]
        popen = subprocess.Popen(cmd, stdout=subprocess.PIPE)
        popen.wait()

        # Get output
        output = popen.stdout
        assert output is not None
        output = output.read().decode("utf-8")

        print(output)

def main():
    # Runs tests
    test_board = Board()
    rust_minimax = RustMinimax()

    # Test 1
    test_board.push_san("e4")
    move = rust_minimax.get_move(test_board)

    assert move is not None
    print("Predicted move:", move)

if __name__ == "__main__":
    main()
