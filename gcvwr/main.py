a, b, c = map(int, input().split())
print(int((a - b) * 0.9 - sum(map(int, input().split()))))
