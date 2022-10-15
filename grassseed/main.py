c = float(input())
l = int(input())
t = 0
for _ in range(l):
    w, h = map(float, input().split())
    t += w * h * c
print(t)
