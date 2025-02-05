from enum import Enum
from typing import List
class MatrixType(Enum):
    Filled = 1,
    Sequence = 2



def matrix_new(row: int, cols: int) -> list[list[int]]:
    return  [ [0] * cols for r in range(row)] 

def init_seq(ip: list[list[int]]):
    rl = len(ip)
    cl = len(ip[0])

    val = 0
    for r in range(rl): 
        for c in range(cl):
            ip[r][c] = val 
            val = val + 1


#1 2 3 
#4 5 6
def transpose(matrix: list[list[int]]) -> list[list[int]]:
    rc, cc = len(matrix), len(matrix[0])
    mat =  [[row[i] for row in matrix] for i in range(cc)]
    mat  = [[row[c] for row in matrix] for c in range (cc)] 
    return mat

def printmatrix(ip: list[list[int]]) -> None:
    for r in ip:
        print(f"{r}")

m  = matrix_new(3, 3)
init_seq(m)
printmatrix(m)

print("Transposed")
tm = transpose(m)
printmatrix(tm)