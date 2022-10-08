lines = int(input())

truths = set()

for i in range(lines):
    line = input().split(" ")
    assumptions = line[:-2]

    for ass in assumptions:
        if ass not in truths:
            print(i + 1)
            exit(0)

    conclusion = line[-1]
    truths.add(conclusion)

print("correct")
