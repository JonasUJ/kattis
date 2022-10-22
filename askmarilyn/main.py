from random import choice

for _ in range(1000):
    s = choice(["A", "B", "C"])
    print(s)
    d, c = input().split()
    if c == "1":
        print(d)
    else:
        if s == "A":
            print("B" if d != "B" else "C")
        elif s == "B":
            print("A" if d != "A" else "C")
        else:
            print("A" if d != "A" else "B")
    input()
