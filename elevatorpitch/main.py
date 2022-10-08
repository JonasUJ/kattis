import sys
import heapq
import collections

class Cell:
    def __init__(self, level: int, x: int, y: int) -> None:
        self.level = level
        self._level = level
        self.x = x
        self.y = y

    def __gt__(self, other):
        return self._level < other._level

    def __lt__(self, other):
        return self._level > other._level

    def __eq__(self, other):
        return self._level == other._level

h, w = map(int, input().split())

area = []
for y, line in enumerate(sys.stdin):
    area.append([Cell(l, x, y) for x, l in enumerate(map(int, line.split()))])

def lower(c: Cell, level: int):
    queue = collections.deque([(c, level)])
    while len(queue) > 0:
        c, level = queue.pop()
        for off in ((0, 1), (1, 0), (-1, 0), (0, -1)):
            x = c.x + off[0]
            y = c.y + off[1]
            if x < 0 or x >= w or y < 0 or y >= h:
                continue
            other = area[y][x]
            if other.level > 1 and other.level <= level:
                other_level = other.level 
                other.level = 0
                queue.appendleft((other, other_level))

heap = [c for l in area for c in filter(lambda x: x.level > 1, l)]
heapq.heapify(heap)

elevators = 0
while len(heap) > 0:
    top = heapq.heappop(heap)
    if top.level > 1:
        level = top.level
        top.level = 0
        lower(top, level)
        elevators += 1

print(elevators)
