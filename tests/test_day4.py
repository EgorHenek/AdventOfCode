from pathlib import Path
from aoc.day4 import part1


def test_part1() -> None:
    path = Path("./inputs/day4_example.txt")

    result = part1(path)

    assert result == 13
