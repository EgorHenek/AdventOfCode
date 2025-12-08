from pathlib import Path
from aoc.day3 import part1


def test_part1() -> None:
    path = Path("./inputs/day3_example.txt")

    result = part1(path)

    assert result == 357
