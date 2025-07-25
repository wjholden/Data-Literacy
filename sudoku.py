# This program is heavily inspired by https://ericpony.github.io/z3py-tutorial/guide-examples.htm.

from z3 import *

# Declare Integer variables v[0][0] through v[8][8]. Each variable
# represents a position of the Sudouku puzzle.
v = [[Int(f"v{row}{col}") for col in range(1,10)]
     for row in range(1,10)]

# Variables have values between 1 and 9, inclusive.
constraint1 = [And(1 <= v[row][col], v[row][col] <= 9) 
               for row in range(9) for col in range(9)]

# The values in each row are distinct.
constraint2 = [Distinct(v[row]) for row in range(9)]

# The values in each column are distinct.
constraint3 = [Distinct([v[row][col] for row in range(9)]) 
               for col in range(9)]

# The values in each 3x3 square are distinct.
constraint4 = [Distinct([v[row + 3 * y][col + 3 * x] 
                         for row in range(3) for col in range(3)]) 
                         for y in range(3) for x in range(3)]

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

constraint5 = [v[row][col] == example[row][col]
               for row in range(9) for col in range(9)
               if example[row][col] is not None]

# Create and initialize an instance of a Z3 constraint solver
s = Solver()
s.add(constraint1 + constraint2 + constraint3 + constraint4 + constraint5)

# Is the problem satisfiable?
s.check()

m = s.model()
[[m.evaluate(v[row][col]) for col in range(9)] for row in range(9)]

