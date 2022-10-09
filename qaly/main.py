N = int(input())
acc = 0
for _ in range(N):
    a, b = map(float, input().split())
    acc += a * b
print(acc)
