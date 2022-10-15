n = int(input())
for _ in range(n):
    a = input()
    b = input()
    print(a)
    print(b)
    for aa, bb in zip(a, b):
        if aa == bb:
            print(".", end="")
        else:
            print("*", end="")
    print()
    print()
