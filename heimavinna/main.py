acc = 0
for i in input().split(";"):
    s = i.split("-")
    if len(s) == 1:
        acc += 1
    else:
        acc += int(s[1]) - int(s[0]) + 1
print(acc)
