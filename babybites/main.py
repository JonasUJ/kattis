n = int(input())
for i, a in enumerate(input().split()):
    if a.isdigit() and int(a) != i + 1:
        print("something is fishy")
        break
else:
    print("makes sense")

