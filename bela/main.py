from sys import stdin

t = {
    "A": (11, 11),
    "K": (4, 4),
    "Q": (3, 3),
    "J": (2, 20),
    "T": (10, 10),
    "9": (0, 14),
    "8": (0, 0),
    "7": (0, 0),
}

n, d = input().split()

print(sum(t[i[0]][i[1] == d] for i in stdin))
