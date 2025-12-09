from pathlib import Path
from aoc.day5 import part1


def test_part1() -> None:
    path = Path("./inputs/day5_example.txt")

    result = part1(path)

    assert result == 3
