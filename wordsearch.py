# Produces DLXCC compatible input format for a wordsearch puzzle

grid_size_i = 8
grid_size_j = 16
names = [name.strip() for name in input().split()]

# Items
for name in names:
    print(name, end=' ')
print('|', end=' ')
for i in range(grid_size_i):
    for j in range(grid_size_j):
        x = hex(i)[2:]
        y = hex(j)[2:]
        print(f"{x}{y}", end=' ')
print()

directions = [
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1)
]
options = set()

# Options, a set is needed so that we do not consider the reverse word
# orientation when positioning center of word on dual coordinate
for name in names:
    # Forward
    for i in range(grid_size_i):
        for j in range(grid_size_j):
            for di, dj in directions:
                option = [name]

                valid = True
                for k, c in enumerate(name):
                    ii = i + k * di
                    jj = j + k * dj

                    if ii < 0 or ii >= grid_size_i or jj < 0 or jj >= grid_size_j:
                        valid = False
                        break

                    x = hex(ii)[2:]
                    y = hex(jj)[2:]
                    option.append(f"{x}{y}:{c}")

                if valid:
                    options.add(' '.join(sorted(option)))

    # Backwards
    for i in range(grid_size_i):
        for j in range(grid_size_j):
            for di, dj in directions:
                option = [name]

                valid = True
                for k, c in enumerate(name[::-1]):
                    ii = i + k * di
                    jj = j + k * dj

                    if ii < 0 or ii >= grid_size_i or jj < 0 or jj >= grid_size_j:
                        valid = False
                        break

                    x = hex(ii)[2:]
                    y = hex(jj)[2:]
                    option.append(f"{x}{y}:{c}")

                if valid:
                    options.add(' '.join(sorted(option)))

print('\n'.join(sorted(options)))
