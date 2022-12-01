with open("task1.txt", "r") as f:
    file = f.read()

elves_str = file.split("\n\n")
elves = []
for i in range(len(elves_str)):
    splitter = elves_str[i].split("\n")
    if splitter[-1] == '':
        splitter.pop()
    elf = [int(x) for x in splitter]
    elves.append(sum(elf))

def part_one():
    print("Part One")
    maximum = max(elves)
    print(maximum)

def part_two():
    print("Part Two")
    elves.sort()
    last_three = elves[-3:]
    sum_of_remaining = sum(last_three)
    print(sum_of_remaining)

if __name__ == "__main__":
    part_one()
    part_two()