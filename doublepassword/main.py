a = input()
b = input()
c = 1
for aa, bb in zip(a, b):
    if aa != bb:
        c <<= 1
print(c)
