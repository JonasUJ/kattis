from sys import stdin

n, m = map(int, input().split())
res = list(map(str, range(1, n + 1)))

for i in map(int, stdin):
    res[i - 1], res[i] = res[i], res[i - 1]

print("\n".join(res))

