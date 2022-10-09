L = int(input())
D = int(input())
X = int(input())

N = L
while sum(int(i) for i in str(N)) != X:
    N += 1

print(N)

M = D
while sum(int(i) for i in str(M)) != X:
    M -= 1

print(M)
