t, s, n = map(int, input().split())

top = 0
bot = s
cur = 0

for i in input().split():
    i = int(i)
    diff = i - cur
    top = max(0, top - diff)
    bot = min(s, bot + diff)
    top, bot = bot, top
    cur = i

print(max(0, top - (t - cur)))

