from pathlib import Path
from aoc.day4 import part1, part2


def test_part1() -> None:
    path = Path("./inputs/day4_example.txt")

    result = part1(path)

    assert result == 13


def test_part2() -> None:
    path = Path("./inputs/day4_example.txt")

    result = part2(path)

    assert result == 43
