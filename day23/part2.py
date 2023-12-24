import copy
import pathlib
from enum import Enum
from dataclasses import dataclass
from collections import namedtuple
from typing import Union

Node = namedtuple('Node', ['name', 'edges'])
Edge = namedtuple('Edge', ['start', 'end', 'length'])
Point = namedtuple('Point', ['x', 'y'])


class Land(Enum):
    FLAT = 0
    FOREST = 1
    EAST = 2
    SOUTH = 3


@dataclass
class Tile:
    point: Point
    land: Land
    _neighbours: set[Point] = None

    def get_neighbours0(self):
        if self._neighbours is None:
            self._neighbours = {Point(self.point.x+1, self.point.y),
                                Point(self.point.x-1, self.point.y),
                                Point(self.point.x, self.point.y+1),
                                Point(self.point.x, self.point.y-1)}
        return self._neighbours


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

    def set_neighbours(self):
        for tile in self.tiles.values():
            neighbours0 = {n for n in tile.get_neighbours0()
                           if n in self.trail_tiles}
            tile._neighbours = neighbours0

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


@dataclass
class Network:
    nodes: dict[str, Node]
    cache: dict[tuple[frozenset[Node], Node], int]
    distance: int = 0
    _edges: dict[frozenset[str], int] = None

    @property
    def start(self):
        return self.nodes['II']

    @property
    def end(self):
        return self.nodes['JJ']

    @classmethod
    def from_file(cls, fname: str):
        with open(fname) as f:
            data = [s.strip('\n') for s in f.readlines()]
        nodes = {}
        for row in data:
            node_name, connections = row.split(' - ')
            edges = []
            for connection in connections.split(';'):
                edge_name, weight = connection.split('=')
                edge = Edge(node_name, edge_name, int(weight))
                edges.append(edge)
            nodes[node_name] = Node(node_name, frozenset(edges))
        return cls(nodes, {})

    @property
    def edges(self):
        if self._edges is None:
            self._edges = {}
            for k, v in self.nodes.items():
                for e in v.edges:
                    self._edges[frozenset([v.name, e.end])] = e.length
        return self._edges

    def find_longest_path_to_end(self, start: Node = None,
                                 previous_nodes: list[Node] = None):
        previous_nodes.append(start)
        if self.end == start:
            total = 0
            for i in range(len(previous_nodes) - 1):
                key = frozenset([previous_nodes[i].name, previous_nodes[i+1].name])
                total += self.edges[key]
            self.distance = max(self.distance, total)

        for edge in start.edges:
            n = self.nodes[edge.end]
            if n not in previous_nodes:
                self.find_longest_path_to_end(n, previous_nodes[:])
        return


def create_new_graph():
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

def main():
    create_new_graph()
    network = Network.from_file('input2.txt')
    print(network.find_longest_path_to_end(network.start, []))
    print(network.distance)


if __name__ == '__main__':
    main()
