#cython: language_level=3
from cpython cimport array
import array

cpdef char score_board(object board):
    return board[0]