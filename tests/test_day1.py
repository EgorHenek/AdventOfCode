from pathlib import Path

from aoc.day1 import main


def test_example_input() -> None:
    path = Path("./inputs/day1_example.txt")

    result = main(path)

    assert result == 3
