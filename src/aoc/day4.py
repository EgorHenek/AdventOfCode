# https://adventofcode.com/2025/day/4
from pathlib import Path


def part1(path: Path) -> int:
    result = 0
    with path.open("r") as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]
        for n in range(len(lines)):
            for m in range(len(lines[n])):
                if lines[n][m] == "@":
                    adjacent_count = sum(
                        lines[i][j] == "@"
                        for i in range(max(n - 1, 0), min(n + 2, len(lines)))
                        for j in range(max(m - 1, 0), min(m + 2, len(lines[0])))
                        if i != n or j != m
                    )
                    if adjacent_count < 4:
                        result += 1
    return result


def part2(path: Path) -> int:
    result = 0
    with path.open("r") as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]
        lines = [[ch == "@" for ch in line] for line in lines]
        while True:
            old_result = result
            for n in range(len(lines)):
                for m in range(len(lines[n])):
                    if lines[n][m]:
                        adjacent_count = sum(
                            lines[i][j]
                            for i in range(max(n - 1, 0), min(n + 2, len(lines)))
                            for j in range(max(m - 1, 0), min(m + 2, len(lines[0])))
                            if i != n or j != m
                        )
                        if adjacent_count < 4:
                            result += 1
                            lines[n][m] = False
            if old_result == result:
                break
    return result


if __name__ == "__main__":
    path = Path("./inputs/day4.txt")

    result_part1 = part1(path)
    result_part2 = part2(path)

    print(f"{result_part1=}")
    print(f"{result_part2=}")
