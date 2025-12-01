# https://adventofcode.com/2025/day/1
from pathlib import Path


def main(path: Path) -> int:
    zero_counter = 0

    with path.open("r") as file:
        lines = file.readlines()

        rotations: list[int] = []
        for line in lines:
            rotations.append(int(line[1:]))
            if line[0] == "L":
                rotations[-1] *= -1

        position = 50
        for rotation in rotations:
            position += rotation
            position %= 100
            if position == 0:
                zero_counter += 1
    return zero_counter


if __name__ == "__main__":
    path = Path("./inputs/day1.txt")
    result = main(path)
    print(result)
