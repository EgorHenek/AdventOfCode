from pathlib import Path
from aoc.day5 import part1, part2


def test_part1() -> None:
    path = Path("./inputs/day5_example.txt")

    result = part1(path)

    assert result == 3


def test_part2() -> None:
    path = Path("./inputs/day5_example.txt")

    result = part2(path)

    assert result == 14
