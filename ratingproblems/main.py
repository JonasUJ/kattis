from sys import stdin
n, k = map(int, input().split())
s = sum(map(int, stdin))
r = 3 * (n - k)
print((s - r) / n, (s + r) / n)
