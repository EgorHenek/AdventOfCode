from pathlib import Path
from aoc.day2 import part1


def test_part1() -> None:
    path = Path("./inputs/day2_example.txt")

    result = part1(path)

    assert result == 1227775554
