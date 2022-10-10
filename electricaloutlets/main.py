n = int(input())
for _ in range(n):
    a, *r = map(int, input().split())
    print(sum(r) - a + 1)
