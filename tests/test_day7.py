from pathlib import Path
from aoc.day7 import part1


def test_part1() -> None:
    path = Path("./inputs/day7_example.txt")

    result = part1(path)

    assert result == 21
