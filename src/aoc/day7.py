# https://adventofcode.com/2025/day/7
from pathlib import Path


def part1(path: Path) -> int:
    result = 0
    with path.open() as file:
        row = file.readline().strip()
        beams: set[int] = {row.find("S")}
        while row := file.readline().strip():
            for i in range(len(row)):
                if row[i] == "^" and i in beams:
                    beams.remove(i)
                    beams.add(i - 1)
                    beams.add(i + 1)
                    result += 1
    return result


def part2(path: Path) -> int: ...


if __name__ == "__main__":
    path = Path("./inputs/day7.txt")

    result_part1 = part1(path)

    print(f"{result_part1=}")
