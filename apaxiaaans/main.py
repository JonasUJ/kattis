s = input()
for c in s:
    s = s.replace(c + c, c)
print(s)
