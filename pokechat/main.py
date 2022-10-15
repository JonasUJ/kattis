alph = input()
enc = input()
print("".join(map(lambda i: alph[int(i)-1], (enc[i:i+3] for i in range(0, len(enc), 3)))))
