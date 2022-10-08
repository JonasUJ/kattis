sqrt2 = 2**0.5

blocks = []

n = int(input())
for i in range(n):
    name, original_size = input().split()
    size = int(original_size)
    footprint = size

    if name == "cylinder":
        footprint = size * 2
        size = size * sqrt2

    blocks.append((name, size, footprint, original_size))

blocks.sort(key=lambda b: (b[1], b[2]))

m = 0
for b in blocks:
    if b[2] >= m:
        m = b[2]
    else:
        print("impossible")
        break
else:
    print("\n".join(f"{b[0]} {b[3]}" for b in blocks))
