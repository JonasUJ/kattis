n = int(input())
for _ in range(n):
    i = int(input())
    print(i, "is", "even" if i & 1 == 0 else "odd")
