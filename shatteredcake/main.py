W = int(input())
N = int(input())
area = 0
for _ in range(N):
    a, b = map(int, input().split())
    area += a * b
print(int(area/W))
