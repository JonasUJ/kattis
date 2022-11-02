out = ""
for i in range(5):
    if "FBI" in input():
        out += str(i + 1) + " "
if out:
    print(out)
else:
    print("HE GOT AWAY!")
