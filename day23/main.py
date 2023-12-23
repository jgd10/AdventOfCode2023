import warnings
from dataclasses import dataclass
from collections import namedtuple
from enum import Enum
import pathlib
import sys
import copy

sys.setrecursionlimit(10000)


Point = namedtuple('Point', ['x', 'y'])


class Land(Enum):
    FLAT = 0
    FOREST = 1
    EAST = 2
    SOUTH = 3


class Direction:
    NORTH = 0
    SOUTH = 1
    EAST = 2
    WEST = 3


@dataclass
class Tile:
    point: Point
    land: Land

    def get_neighbours0(self):
        return {Point(self.point.x+1, self.point.y),
                Point(self.point.x-1, self.point.y),
                Point(self.point.x, self.point.y+1),
                Point(self.point.x, self.point.y-1)}

    def get_neighbours(self):
        match self.land:
            case Land.EAST:
                return {Point(self.point.x + 1, self.point.y)}
            case Land.SOUTH:
                return {Point(self.point.x, self.point.y + 1)}
            case Land.FLAT:
                return {Point(self.point.x+1, self.point.y),
                        Point(self.point.x-1, self.point.y),
                        Point(self.point.x, self.point.y+1),
                        Point(self.point.x, self.point.y-1)}
            case Land.FOREST:
                warnings.warn("Something's gone wrong. This is a forest tile.")
                return set()

@dataclass
class Trail:
    tiles: dict[Point, Tile]
    _xmax: int = None
    _ymax: int = None
    _trail: set[Point] = None
    _distances: dict[Point, int] = None
    _visited: set[Point] = None

    @property
    def visited(self):
        if self._visited is None:
            self._visited = set()
        return self._visited

    @property
    def trail_tiles(self):
        if self._trail is None:
            self._trail = {p for p, v in self.tiles.items() if v.land in [Land.EAST, Land.SOUTH, Land.FLAT]}
        return self._trail

    @property
    def xmax(self):
        if self._xmax is None:
            xmax = 0
            for p in self.tiles:
                xmax = max(xmax, p.x)
            self._xmax = xmax
        return self._xmax

    @property
    def ymax(self):
        if self._ymax is None:
            ymax = 0
            for p in self.tiles:
                ymax = max(ymax, p.y)
            self._ymax = ymax
        return self._ymax

    @classmethod
    def from_file(cls, fname: pathlib.Path):
        with open(fname) as f:
            data = [s.strip('\n') for s in f.readlines()]
        tiles = {}
        for i, row in enumerate(data):
            for j, c in enumerate(row):
                p = Point(j, i)
                match c:
                    case '#':
                        type_ = Land.FOREST
                    case '.':
                        type_ = Land.FLAT
                    case '>':
                        type_ = Land.EAST
                    case 'v':
                        type_ = Land.SOUTH
                    case _:
                        raise ValueError
                tiles[p] = Tile(p, type_)
        return cls(tiles)

    def find_longest_path(self, start: Point):
        weights = {p: 0 for p, v in self.tiles.items() if v.land in [Land.EAST, Land.SOUTH, Land.FLAT]}
        weights[start] = 0
        rest_of_queue = [p for p, v in self.tiles.items() if p != start and v.land in [Land.EAST, Land.SOUTH, Land.FLAT]]
        queue = [start] + rest_of_queue
        while queue:
            queue.sort(key=lambda x: weights[x])
            point = queue.pop(0)
            neighbours = self.tiles[point].get_neighbours()
            if point == Point(3, 5):
                print(neighbours)
            for neighbour in neighbours:
                if neighbour in queue:
                    alt_distance = weights[point] + 1
                    if alt_distance > weights[neighbour]:
                        weights[neighbour] = alt_distance
        return weights

    @property
    def distances(self):
        if self._distances is None:
            self._distances = {p: 0 for p in self.trail_tiles}
        return self._distances

    def depth_first_search(self, start: Point, start_distance: int, previous: set[Point]):
        current_distance = start_distance
        previous.add(start)
        neighbours = [n for n in self.tiles[start].get_neighbours() if n in self.trail_tiles and n not in previous]
        while neighbours:
            if len(neighbours) == 1:
                neighbour = neighbours.pop(0)
                current_distance += 1
                self.distances[neighbour] = max(current_distance, self.distances[neighbour])
                previous.add(neighbour)
                neighbours = [n for n in self.tiles[neighbour].get_neighbours() if n in self.trail_tiles and n not in previous]
            else:
                neighbour = neighbours.pop(0)
                previous.add(neighbour)
                self.distances[neighbour] = max(current_distance, self.depth_first_search(neighbour, current_distance + 1, copy.deepcopy(previous)))
        return current_distance

    def depth_first_search2(self, start: Point, start_distance: int, previous: set[Point]):
        current_distance = start_distance
        previous.add(start)
        neighbours = [n for n in self.tiles[start].get_neighbours0() if n in self.trail_tiles and n not in previous]
        while neighbours:
            if len(neighbours) == 1:
                neighbour = neighbours.pop(0)
                current_distance += 1
                self.distances[neighbour] = max(current_distance, self.distances[neighbour])
                previous.add(neighbour)
                neighbours = [n for n in self.tiles[neighbour].get_neighbours0() if n in self.trail_tiles and n not in previous]
            else:
                neighbour = neighbours.pop(0)
                previous.add(neighbour)
                self.distances[neighbour] = max(current_distance, self.depth_first_search2(neighbour, current_distance + 1, copy.deepcopy(previous)))
        return current_distance


def part1():
    trail = Trail.from_file(pathlib.Path('./input.txt'))
    start = Point(1, 0)
    end = Point(trail.xmax - 1, trail.ymax)
    trail.depth_first_search(start, 0, set())
    print(trail.distances[end])


def part2():
    trail = Trail.from_file(pathlib.Path('./input.txt'))
    start = Point(1, 0)
    end = Point(trail.xmax - 1, trail.ymax)
    trail.depth_first_search2(start, 0, set())
    print(trail.distances[end])



def main():
    part1()
    part2()


if __name__ == '__main__':
    main()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
