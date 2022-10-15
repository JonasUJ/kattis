a, b, c = map(int, input().split())
for _ in range(c):
    _, *j = map(int, input().split())
    print("KEEP" if b in j else "REMOVE")
