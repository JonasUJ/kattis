from math import sqrt
from sys import stdin
N, W, H = map(int, input().split())
L = sqrt(W*W + H*H)
for m in stdin:
    print("DA" if int(m) <= L else "NE")
