for i in range(1, 1+int(input())):
    n = int(input().split()[1]) + 1
    print(i, int(n*(n+1)/2 - 1))
