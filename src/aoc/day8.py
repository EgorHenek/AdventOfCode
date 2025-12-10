# https://adventofcode.com/2025/day/8
from dataclasses import dataclass
from pathlib import Path


@dataclass
class Point:
    x: int
    y: int
    z: int


def part1(path: Path, conntections_limit: int) -> int:
    with path.open() as file:
        points: list[Point] = []
        while line := file.readline():
            x, y, z = line.split(",")
            points.append(Point(int(x), int(y), int(z)))
        parent = [i for i in range(len(points))]
        component_size = [1] * len(points)

        def find_root(i: int) -> int:
            if parent[i] == i:
                return i
            parent[i] = find_root(parent[i])
            return parent[i]

        def union(i: int, j: int) -> bool:
            root_i = find_root(i)
            root_j = find_root(j)
            if root_i == root_j:
                return False
            if component_size[root_i] < component_size[root_j]:
                parent[root_i] = root_j
                component_size[root_j] += component_size[root_i]
            else:
                parent[root_j] = root_i
                component_size[root_i] += component_size[root_j]
            return True

        edges: list[tuple[int, int, int]] = []
        for i in range(len(points) - 1):
            for j in range(i + 1, len(points)):
                point1 = points[i]
                point2 = points[j]
                euclidean_distance = (
                    (point2.x - point1.x) ** 2
                    + (point2.y - point1.y) ** 2
                    + (point2.z - point1.z) ** 2
                )
                edges.append((euclidean_distance, i, j))
        edges.sort()

        connections_attempts = 0
        for edge in edges:
            _ = union(edge[1], edge[2])
            connections_attempts += 1
            if connections_attempts == conntections_limit:
                break

        final_component_sizes: list[int] = []
        for i in range(len(points)):
            if parent[i] == i:
                final_component_sizes.append(component_size[i])

        final_component_sizes.sort(reverse=True)
        return (
            final_component_sizes[0]
            * final_component_sizes[1]
            * final_component_sizes[2]
        )


if __name__ == "__main__":
    path = Path("./inputs/day8.txt")

    result_part1 = part1(path, 1000)

    print(f"{result_part1=}")
