import sys

p = set()
r = set()
c = set()
b = set()
options = set()

for i in range(9):
    for j in range(9):
        p.add(f"p{i}{j}")
        for k in range(1, 10):
            l = 3 * (i // 3) + (j // 3)
            r.add(f"r{i}{k}")
            c.add(f"c{j}{k}")
            b.add(f"b{l}{k}")
            options.add(f"p{i}{j} r{i}{k} c{j}{k} b{l}{k}")

with open(sys.argv[1]) as input_grid:
    grid = [[int(d) for d in line.strip()] for line in input_grid]

    for i in range(9):
        for j in range(9):
            if grid[i][j] != 0:
                k = grid[i][j]
                l = 3 * (i // 3) + (j // 3)
                p.remove(f"p{i}{j}")
                r.remove(f"r{i}{k}")
                c.remove(f"c{j}{k}")
                b.remove(f"b{l}{k}")

print(' '.join(sorted(p)), end=' ')
print(' '.join(sorted(r)), end=' ')
print(' '.join(sorted(c)), end=' ')
print(' '.join(sorted(b)))

for option in options:
    ps, rs, cs, bs = option.split()
    if ps in p and rs in r and cs in c and bs in b:
        print(option)
