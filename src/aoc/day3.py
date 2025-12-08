# https://adventofcode.com/2025/day/2
from pathlib import Path


def part1(path: Path) -> int:
    result = 0
    with path.open("r") as file:
        lines = file.readlines()
        lines = map(lambda L: [int(L[i]) for i in range(len(L.strip()))], lines)
        for line in lines:
            num1_pos = -1
            num2_pos = -1
            for searchable_number in range(9, -1, -1):
                start = max(0, num1_pos)
                for i in range(start, len(line)):
                    if line[i] == searchable_number:
                        if i != len(line) - 1 and num1_pos < 0:
                            num1_pos = i
                        elif num2_pos < 0:
                            num2_pos = i
            result += line[num1_pos] * 10 + line[num2_pos]
    return result


if __name__ == "__main__":
    path = Path("./inputs/day3.txt")

    result_part1 = part1(path)

    print(f"{result_part1=}")
