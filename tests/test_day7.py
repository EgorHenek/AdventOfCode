from pathlib import Path
from aoc.day7 import part1, part2


def test_part1() -> None:
    path = Path("./inputs/day7_example.txt")

    result = part1(path)

    assert result == 21


def test_part2() -> None:
    path = Path("./inputs/day7_example.txt")

    result = part2(path)

    assert result == 40
