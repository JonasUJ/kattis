import sys
c, m = 0, 0
for i, l in enumerate(sys.stdin):
    s = sum(map(int, l.split()))
    if s > m:
        c, m = i, s
print(c + 1, m)
