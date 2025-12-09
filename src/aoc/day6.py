# https://adventofcode.com/2025/day/6
from functools import reduce
import operator
from pathlib import Path
from typing import Callable

OPERATIONS: dict[str, Callable[[int, int], int]] = {
    "+": operator.add,
    "*": operator.mul,
}


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
                column_result = reduce(OPERATIONS[op], [column_result, int(row[i])])
            result += column_result
    return result


def part2(path: Path) -> int:
    result = 0
    with path.open() as file:
        lines = file.readlines()
        lines = [line.strip("\n") for line in lines]
        op = ""
        nums: list[int] = []
        for i in range(len(lines[0])):
            if (new_op := lines[-1][i]) != " ":
                op = new_op
            num = "".join([row[i] for row in lines[:-1] if row[i] != " "])
            if num == "":
                result += reduce(OPERATIONS[op], nums)
                nums = []
            else:
                nums.append(int(num))
        result += reduce(OPERATIONS[op], nums)
    return result


if __name__ == "__main__":
    path = Path("./inputs/day6.txt")

    result_part1 = part1(path)
    result_part2 = part2(path)

    print(f"{result_part1=}")
    print(f"{result_part2=}")
