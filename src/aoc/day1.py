# https://adventofcode.com/2025/day/1
from pathlib import Path


def part1(path: Path) -> int:
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


def part2(path: Path) -> int:
    zero_counter = 0

    with path.open("r") as file:
        lines = file.readlines()

        rotations: list[int] = []
        for line in lines:
            rotations.append(int(line[1:]))
            if line[0] == "L":
                rotations[-1] *= -1

        current_position = 50
        for rotation in rotations:
            start_pos = current_position
            end_pos = start_pos + rotation
            crossings = 0
            if rotation > 0:
                crossings = end_pos // 100 - start_pos // 100
            elif rotation < 0:
                crossings = (start_pos - 1) // 100 - (end_pos - 1) // 100
            zero_counter += crossings
            current_position = end_pos % 100
    return zero_counter


if __name__ == "__main__":
    path = Path("./inputs/day1.txt")

    result_part1 = part1(path)
    result_part2 = part2(path)

    print(f"Part 1: {result_part1}")
    print(f"Part 2: {result_part2}")
