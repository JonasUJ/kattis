from collections import Counter
from statistics import median_low, median_high

N, t = map(int, input().split())
Ns = list(map(int, input().split()))

if t == 1:
    s = set(Ns)
    for n in Ns:
        if 7777 - n in s:
            print("Yes")
            break
    else:
        print("No")

elif t == 2:
    s = set(Ns)
    if len(Ns) == len(s):
        print("Unique")
    else:
        print("Contains duplicate")

elif t == 3:
    c = Counter(Ns)
    n, count = c.most_common(1)[0]
    if count > N / 2:
        print(n)
    else:
        print(-1)

elif t == 4:
    if N & 1:
        print(median_low(Ns))
    else:
        print(median_low(Ns), median_high(Ns))

elif t == 5:
    print(" ".join(map(str, sorted(filter(lambda n: 100 <= n <= 999, Ns)))))

