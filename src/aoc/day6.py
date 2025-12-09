# https://adventofcode.com/2025/day/6
from pathlib import Path


def part1(path: Path) -> int:
    result = 0
    with path.open() as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]
        operations = lines[-1].split()
        values = [line.split() for line in lines[:-1]]
        for i, op in enumerate(operations):
            column_result = 0 if op == "+" else 1
            for row in values:
                if op == "*":
                    column_result *= int(row[i])
                else:
                    column_result += int(row[i])
            result += column_result
    return result


if __name__ == "__main__":
    path = Path("./inputs/day6.txt")

    result_part1 = part1(path)

    print(f"{result_part1=}")
