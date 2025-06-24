import sys

n = int(sys.argv[1])

for i in range(1, n + 1):
    print(i, end= ' ')
for i in range(1, 2 * n + 1):
    print(f"s{i}", end=' ')
print()

for i in range(1, n + 1):
    for j in range(1, 2 * n):
        for k in range(j + 1, 2 * n + 1):
            if k != i + j + 1:
                continue
            print(f"{i} s{j} s{k}")
