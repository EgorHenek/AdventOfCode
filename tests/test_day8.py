from pathlib import Path
from aoc.day8 import part1


def test_part1() -> None:
    path = Path("./inputs/day8_example.txt")

    result = part1(path, 10)

    assert result == 40
