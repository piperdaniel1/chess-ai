import chess
from chess import svg
import pygame

board = chess.Board()
mySvg = svg.board()
print(type(mySvg))

with open(f"pieces/board.svg", "w") as f:
    f.write(mySvg)
