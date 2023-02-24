from operator import add, sub, mul
from itertools import permutations
from functools import reduce

def div(x, y):
    if x % y != 0:
        return x / y
    else:
        return x // y

ops = [add, sub, mul, div]
ops = ops + ops
perms = permutations(ops, r=2)

inp = tuple(map(int, input().split()))

res = [reduce(lambda x, v: v[0](x, v[1]), zip(perm, inp[1:]), inp[0]) for perm in perms]

print(min(filter(lambda x: x >= 0 and isinstance(x, int), res)))
