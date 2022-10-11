G, S, C = map(int, input().split())
c = G * 3 + S * 2 + C
if c >= 8:
    print("Province or Gold")
elif c >= 6:
    print("Duchy or Gold")
elif c >= 5:
    print("Duchy or Silver")
elif c >= 3:
    print("Estate or Silver")
elif c >= 2:
    print("Estate or Copper")
else:
    print("Copper")
