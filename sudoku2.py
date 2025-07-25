# This program is a different design from the other one.

# reference: https://microsoft.github.io/z3guide/programming/Z3%20Python%20-%20Readonly/Introduction/

from z3 import *
import itertools

# Declare Boolean variables representing whether a value x is in a position.
v = [[[Bool(f"{x},{row},{col}") for col in range(9)]
      for row in range(9)]
      for x in range(9)]

# For each 1x1 square, exactly one of the 9 possible values is true.
c1 = [Or(*[v[x][row][col]
           for x in range(9)])
           for row in range(9)
           for col in range(9)]

# For each row, a number appears in exactly one column.
c2 = [AtMost(*[v[x][row][col]
               for col in range(9)], 1)
               for row in range(9)
               for x in range(9)]

# For each column, a number appears in exactly one row.
c3 = [AtMost(*[v[x][row][col]
               for row in range(9)], 1)
               for col in range(9)
               for x in range(9)]

# A variable is only set once in a 3x3 square.
c4 = [AtMost(*[v[x][row + 3 * i][col + 3 * j]
               for row in range(3)
               for col in range(3)], 1)
               for x in range(9)
               for i in range(3)
               for j in range(3)]

# Literal assignments for our puzzle input.
example = [[None, None, 8, 4, 2, None, 9, 1, None],
           [4, 3, 2, None, 1, 5, None, 8, 7],
           [9, None, None, None, 8, None, 2, None, 4],
           [8, None, None, None, None, 2, None, 7, None],
           [None, 7, 4, None, None, 8, None, None, None],
           [None, 2, 9, None, None, 4, 5, 3, None],
           [None, None, None, None, 7, None, None, None, None],
           [None, 4, 3, None, None, 6, None, 9, None],
           [5, 8, None, None, None, 9, 7, 2, 6]]

# Assign literals to variables where applicable.
# Subtract one because the array indices start at zero.
c5 = [v[example[row][col] - 1][row][col] == True 
      for row in range(9)
      for col in range(9)
      if example[row][col]]

# Create and initialize an instance of a Z3 constraint solver
s = Solver()
s.add(c1 + c2 + c3 + c4 + c5)
# Is the problem satisfiable?
s.check()
m = s.model()
solution = [row.copy() for row in example]
for (x,row,col) in itertools.product(range(9), range(9), range(9)):
      if m.evaluate(v[x][row][col]) == True:
            solution[row][col] = x+1
for row in solution:
      print(row)
