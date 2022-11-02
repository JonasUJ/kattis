print(int(sum(map(lambda t: t[0]*t[1], zip((4,3,2,7,6,5,4,3,2,1), map(int, filter(lambda c: c.isdigit(), input()))))) % 11 == 0))
