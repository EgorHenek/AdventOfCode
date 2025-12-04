# https://adventofcode.com/2025/day/2
from pathlib import Path


def part1(path: Path) -> int:
    result = 0
    with path.open("r") as file:
        ranges = file.readline().split(",")
        ranges = map(lambda x: map(int, x.split("-")), ranges)
        for ids_range in ranges:
            start = next(ids_range)
            end = next(ids_range)
            for i in range(start, end + 1):
                str_i = str(i)
                n = len(str_i)
                if n % 2 == 0 and str_i[n // 2 :] == str_i[: n // 2]:
                    result += i
    return result


if __name__ == "__main__":
    path = Path("./inputs/day2.txt")

    result_part1 = part1(path)

    print(f"Part 1: {result_part1}")
