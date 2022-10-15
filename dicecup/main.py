I = tuple(map(int, input().split(" ")))
N, M = (min(I), max(I))
print("\n".join(map(str, range(N + 1, M + 2))))

