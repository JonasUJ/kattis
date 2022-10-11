import math
n = int(input())
acc = 0
for _ in range(n):
    a = input()
    acc += math.pow(int(a)//10, int(a[-1]))
print(int(acc))
