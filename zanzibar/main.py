n = int(input())
for _ in range(n):
    prev = 1
    acc = 0
    for i in map(int, input().split()):
        acc += max(i - prev * 2, 0)
        prev = i
    print(acc)
