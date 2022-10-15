from math import sqrt
n = int(input())
for _ in range(n):
    s = input()
    sq = int(sqrt(len(s)))
    print("".join(s[i::sq] for i in range(sq - 1, -1, -1)))
