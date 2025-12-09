# https://adventofcode.com/2025/day/5
from heapq import heapify, heappop
from pathlib import Path


def part1(path: Path) -> int:
    result = 0
    with path.open("r") as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]

        fresh_ranges: list[list[int]] = []

        lines_iter = iter(lines)
        line = next(lines_iter)

        while line:
            start_end = line.split("-")
            start, end = int(start_end[0]), int(start_end[1])
            fresh_ranges.append([start, end])
            line = next(lines_iter)
        for line in lines_iter:
            for fresh_range in fresh_ranges:
                if fresh_range[0] <= int(line) <= fresh_range[1]:
                    result += 1
                    break
    return result


def part2(path: Path) -> int:
    result = 0
    with path.open("r") as file:
        lines = file.readlines()
        lines = map(lambda line: line.strip(), lines)
        lines = filter(lambda line: line.count("-"), lines)
        lines = [(int(line.split("-")[0]), int(line.split("-")[1])) for line in lines]
        heapify(lines)

        fresh_ranges = [heappop(lines)]

        while lines:
            start, end = heappop(lines)
            if fresh_ranges[-1][0] <= start <= fresh_ranges[-1][1]:
                fresh_ranges[-1] = fresh_ranges[-1][0], max(end, fresh_ranges[-1][1])
            else:
                fresh_ranges.append((start, end))

    for start, end in fresh_ranges:
        result += end - start + 1
    return result


if __name__ == "__main__":
    path = Path("./inputs/day5.txt")

    result_part1 = part1(path)
    result_part2 = part2(path)

    print(f"{result_part1=}")
    print(f"{result_part2=}")
