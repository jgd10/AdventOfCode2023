import warnings
from dataclasses import dataclass
from collections import namedtuple
from enum import Enum
import pathlib
import sys
import copy
from typing import Union

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
    _neighbours: set[Point] = None
    _neighbours1: set[Point] = None

    def get_neighbours0(self):
        if self._neighbours is None:
            self._neighbours = {Point(self.point.x+1, self.point.y),
                                Point(self.point.x-1, self.point.y),
                                Point(self.point.x, self.point.y+1),
                                Point(self.point.x, self.point.y-1)}
        return self._neighbours

    def get_neighbours(self):
        if self._neighbours1 is None:
            match self.land:
                case Land.EAST:
                    self._neighbours1 = {Point(self.point.x + 1, self.point.y)}
                case Land.SOUTH:
                    self._neighbours1 = {Point(self.point.x, self.point.y + 1)}
                case Land.FLAT:
                    self._neighbours1 = self.get_neighbours0()
                case Land.FOREST:
                    warnings.warn("Something's gone wrong. This is a forest tile.")
                    self._neighbours1 = set()
        return self._neighbours1


@dataclass
class Trail:
    tiles: dict[Point, Tile]
    _xmax: int = None
    _ymax: int = None
    _trail: set[Point] = None
    _distances: dict[Point, int] = None
    _visited: set[Point] = None
    _cache: dict[tuple[Union[Point, int]], int] = None
    _tile_path: dict[Point, set[Point]] = None

    @property
    def tile_path(self):
        if self._tile_path is None:
            self._tile_path = {k: set() for k in self.trail_tiles}
        return self._tile_path

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

    @property
    def distances(self):
        if self._distances is None:
            self._distances = {p: 0 for p in self.trail_tiles}
        return self._distances

    def set_neighbours(self):
        for tile in self.tiles.values():
            neighbours0 = {n for n in tile.get_neighbours0()
                           if n in self.trail_tiles}
            neighbours1 = {n for n in tile.get_neighbours()
                           if n in self.trail_tiles}
            tile._neighbours = neighbours0
            tile._neighbours1 = neighbours1

    def depth_first_search(self, start: Point, start_distance: int, previous: set[Point]):
        key = tuple([start, start_distance] + list(previous))
        if key in self.cache:
            return self.cache[key]
        current_distance = start_distance
        previous.add(start)
        neighbours = [n for n in self.tiles[start].get_neighbours() if n not in previous]
        while neighbours:
            if len(neighbours) == 1:
                neighbour = neighbours.pop(0)
                current_distance += 1
                self.distances[neighbour] = max(current_distance, self.distances[neighbour])
                previous.add(neighbour)
                neighbours = [n for n in self.tiles[neighbour].get_neighbours() if n not in previous]
            else:
                neighbour = neighbours.pop(0)
                previous.add(neighbour)
                self.distances[neighbour] = max(current_distance, self.depth_first_search(neighbour, current_distance + 1, copy.deepcopy(previous)))
        self._cache[key] = current_distance
        return current_distance

    @property
    def cache(self):
        if self._cache is None:
            self._cache = {}
        return self._cache

    def get_intersection_tiles(self):
        intersections = []
        for key in self.trail_tiles:
            neighbours = [v for v in self.tiles[key].get_neighbours0() if v in self.trail_tiles]
            if len(neighbours) >= 3:
                intersections.append(key)
        return intersections

    def lengths_from_intersections(self, ends):
        intersections = self.get_intersection_tiles() + ends
        results = {i: [] for i in intersections}
        for intersection in intersections:
            start = intersection
            visited = {start}
            neighbours = [n for n in self.tiles[start].get_neighbours0() if n in self.trail_tiles and n not in visited]
            for neighbour in neighbours:
                length = 1
                visited = {neighbour, start}
                neighbours = [p for p in self.tiles[neighbour].get_neighbours0() if p not in visited]
                while len(neighbours) == 1:
                    n = neighbours.pop(0)
                    visited.add(n)
                    length += 1
                    neighbours = [p for p in self.tiles[n].get_neighbours0() if p not in visited]
                results[intersection].append((length, n))
        return results



    def depth_first_search2(self, start: Point, end: Point):
        self.tile_path[start].add(start)
        while start != end:
            neighbours = [n for n in self.tiles[start].get_neighbours0() if
                          n not in self.tile_path[start]]
            if not neighbours:
                break
            for neighbour in neighbours:
                if len(self.tile_path[neighbour]) < len(self.tile_path[start]) + 1:
                    self.tile_path[neighbour] = self.tile_path[start]
                    self.tile_path[neighbour].add(neighbour)
                start = neighbour
                self.depth_first_search2(start, end)

        return


def part1():
    trail = Trail.from_file(pathlib.Path('./example.txt'))
    trail.set_neighbours()
    start = Point(1, 0)
    end = Point(trail.xmax - 1, trail.ymax)
    trail.depth_first_search(start, 0, set())
    print(trail.distances[end])

def would_create_a_cycle(network, edge):
    point1, point2 = edge
    node1, node2 = False, False
    for edge_ in network:
        if point1 in edge_:
            node1 = True
    for edge_ in network:
        if point2 in edge_:
            node2 = True
    return node1 and node2

def part2():
    trail = Trail.from_file(pathlib.Path('./input.txt'))
    trail.set_neighbours()
    start = Point(1, 0)
    end = Point(trail.xmax - 1, trail.ymax)
    results = trail.lengths_from_intersections([start, end])
    rename = {k: c for k, c in zip(results.keys(), list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')+['AA', 'BB', 'CC', 'DD', 'EE', 'FF', 'GG', 'HH', 'II', 'JJ', 'KK', 'LL', 'MM'])}
    with open('./input2.txt', 'w') as f:
        for k, v in results.items():
            string = ';'
            nodes = string.join([f'{rename[val[1]]}={val[0]}' for val in v])
            f.write(f'{rename[k]} - {nodes}\n')
    print(results)
    #trail.depth_first_search2(start, end)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
