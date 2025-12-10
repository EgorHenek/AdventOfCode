# https://adventofcode.com/2025/day/7
from pathlib import Path
from functools import cache


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


def part2(path: Path) -> int:
    with path.open() as file:
        rows = file.readlines()
        rows_len = len(rows)
        start_col = rows[0].find("S")

        @cache
        def dfs(row: int, col: int) -> int:
            if row >= rows_len - 1:
                return 1

            result = 0
            if rows[row][col] == "^":
                result = dfs(row + 2, col - 1) + dfs(row + 2, col + 1)
            else:
                result = dfs(row + 2, col)
            return result

        return dfs(0, start_col)


if __name__ == "__main__":
    path = Path("./inputs/day7.txt")

    result_part1 = part1(path)
    result_part2 = part2(path)

    print(f"{result_part1=}")
    print(f"{result_part2=}")
