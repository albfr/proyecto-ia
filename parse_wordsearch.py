from random import choice
from string import ascii_uppercase
import sys

n_names = 18
grid_size_i = 8
grid_size_j = 16

with open(sys.argv[1]) as solutions:
    # This is on a while-loop since input file might have different solutions
    while True:
        grid = [[choice(ascii_uppercase) for _ in range(grid_size_j)] for _ in range(grid_size_i)]
        marked = [[False for _ in range(grid_size_j)] for _ in range(grid_size_i)]
        overlaps = [[0 for _ in range(grid_size_j)] for _ in range(grid_size_i)]
        try:
            n_sol = next(solutions).strip()

            for _ in range(n_names):
                opt = next(solutions).split('(')
                colors = opt[0].split()[1:]

                for color in colors:
                    c = color.split(':')
                    coord = c[0]
                    ch = c[1]

                    x = int(coord[0], 16)
                    y = int(coord[1], 16)

                    grid[x][y] = ch
                    marked[x][y] = True
                    overlaps[x][y] += 1


            # Count overlaps
            ct = 0
            for i in range(len(overlaps)):
                for x in overlaps[i]:
                    if x == 3:
                        ct += 1

            # Print word search solution with solutions
            # on green
            print("solution number", int(n_sol[:-1]))
            print("overlap count", ct)
            for i, row in enumerate(grid):
                for j, c in enumerate(row):
                    if marked[i][j]:
                        # print('\033[91m', end='') # red
                        print('\033[92m', end='') # green
                        print(c, end='')
                        print('\033[0m', end='')
                    else:
                        print(c, end='')
                print()

            t = 0
            for row in overlaps:
                for r in row:
                    if r > 1:
                        t += 1
            print("overlap metric:", t / (grid_size_i * grid_size_j))
        except:
            break
