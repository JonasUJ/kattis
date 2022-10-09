from sys import stdin
X = int(input())
N = int(input())
print(X + X * N - sum(map(int, stdin)))
