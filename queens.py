import sys

n = int(sys.argv[1])

for i in range(1, n + 1):
    print(f"r{i}", end=' ')
for i in range(1, n + 1):
    print(f"c{i}", end=' ')
print('|', end=' ')
for i in range(2, 2 * n + 1):
    print(f"a{i}", end=' ')
for i in range(1 - n, n):
    print(f"b{i}", end=' ')
print()

for i in range(1, n + 1):
    for j in range(1, n + 1):
        print(f"r{i} c{j} a{i+j} b{i-j}")
