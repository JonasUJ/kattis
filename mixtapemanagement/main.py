n = int(input())

sub = len(str(n))
for i, j in enumerate(map(int, input().split())):
    print((i + 1) * 10**j + 10**(j+sub))
