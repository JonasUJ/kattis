s = input()
acc = [0, 0, 0, 0]
for c in s:
    if c == "_": acc[0] += 1
    elif c.islower(): acc[1] += 1
    elif c.isupper(): acc[2] += 1
    else: acc[3] += 1
print("\n".join(str(i / len(s)) for i in acc))
