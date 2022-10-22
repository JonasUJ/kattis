from functools import reduce

n = int(input())
print(reduce(lambda a, i: a + -min(i, 0), map(int, input().split()), 0))

