n = int(input())
c = int(input(), base=2)
c |= c >> 1 | c >> 2
print(bin(c).count("1"))
