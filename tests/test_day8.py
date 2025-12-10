from pathlib import Path
from aoc.day8 import part1, part2


def test_part1() -> None:
    path = Path("./inputs/day8_example.txt")

    result = part1(path, 10)

    assert result == 40


def test_part2() -> None:
    path = Path("./inputs/day8_example.txt")

    result = part2(path)

    assert result == 25272
