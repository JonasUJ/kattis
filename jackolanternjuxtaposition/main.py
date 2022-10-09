from functools import reduce
print(reduce(lambda a, b: a * int(b), input().split(), 1))
