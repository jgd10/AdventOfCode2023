from dataclasses import dataclass
from enum import Enum
import matplotlib.pyplot as plt
import numpy as np


class BlockType(Enum):
    START = 1
    END = 2
    NONE = 3


@dataclass(frozen=True)
class Coord:
    i: int
    j: int

    def get_neighbours(self):
        normal_coords = {Coord(self.i+1, self.j),
                         Coord(self.i-1, self.j),
                         Coord(self.i, self.j+1),
                         Coord(self.i, self.j-1)}
        three_fwd = {(Coord(self.i + 1, self.j), Coord(self.i + 2, self.j), Coord(self.i + 3, self.j - 1)),
                     (Coord(self.i + 1, self.j), Coord(self.i + 2, self.j), Coord(self.i + 3, self.j + 1)),
                     (Coord(self.i - 1, self.j), Coord(self.i - 2, self.j), Coord(self.i - 3, self.j - 1)),
                     (Coord(self.i - 1, self.j), Coord(self.i - 2, self.j), Coord(self.i - 3, self.j + 1)),
                     (Coord(self.i, self.j + 1), Coord(self.i, self.j + 2), Coord(self.i + 1, self.j + 3)),
                     (Coord(self.i, self.j + 1), Coord(self.i, self.j + 2), Coord(self.i - 1, self.j + 3)),
                     (Coord(self.i, self.j - 1), Coord(self.i, self.j - 2), Coord(self.i + 1, self.j - 3)),
                     (Coord(self.i, self.j - 1), Coord(self.i, self.j - 2), Coord(self.i - 1, self.j - 3)),
                     (Coord(self.i + 1, self.j), Coord(self.i + 2, self.j)),
                     (Coord(self.i + 1, self.j), Coord(self.i + 2, self.j)),
                     (Coord(self.i - 1, self.j), Coord(self.i - 2, self.j)),
                     (Coord(self.i - 1, self.j), Coord(self.i - 2, self.j)),
                     (Coord(self.i, self.j + 1), Coord(self.i, self.j + 2)),
                     (Coord(self.i, self.j + 1), Coord(self.i, self.j + 2)),
                     (Coord(self.i, self.j - 1), Coord(self.i, self.j - 2)),
                     (Coord(self.i, self.j - 1), Coord(self.i, self.j - 2))}
        return normal_coords, three_fwd


@dataclass
class Block:
    coord: Coord
    loss: int
    weight: int
    type_: BlockType



@dataclass
class Network:
    blocks: dict[Coord, Block]
    visited: set[Coord] = None

    def plot(self):
        fig, ax = plt.subplots()
        imax = 0
        jmax = 0
        for c in self.blocks:
            imax = max(imax, c.i)
            jmax = max(jmax, c.j)
        weight_array = np.zeros((jmax+1, imax+1))
        for c, v in self.blocks.items():
            weight_array[c.j, c.i] = v.weight

        ax.imshow(weight_array, cmap='plasma', interpolation=None)
        ax.set_aspect(1)
        plt.show()

    @property
    def end_block(self):
        return [b for b in self.blocks.values() if b.type_ == BlockType.END][0]

    def brute_force(self):
        if self.visited is None:
            self.visited = set()
        start = [b
                 for k, b in self.blocks.items()
                 if b.type_ == BlockType.START].pop(0)

        path_cost = start.loss
        path = [start.coord]
        self.visited.add(start.coord)
        neighbours = start.coord.get_neighbours()
        if all([p.i == path[-1].i for p in path[-3:]]):
            neighbours = {n for n in neighbours if n.i != path[-1].i}
        if all([p.j == path[-1].j for p in path[-3:]]):
            neighbours = {n for n in neighbours if n.j != path[-1].j}
        for neighbour in neighbours:
            if neighbour in self.blocks:
                path.append(self.blocks[neighbour])

    def dijkstra(self):
        queue = [c for c in self.blocks.keys()]
        queue.sort(key=lambda x: self.blocks[x].weight)
        self.blocks[queue[0]].weight = 0
        while queue:
            closest_block = self.blocks[queue.pop(0)]
            immediates = closest_block.coord.get_neighbours()
            for neighbour in immediates:
                if neighbour in queue:
                    alt_distance = closest_block.weight + self.blocks[neighbour].loss
                    if alt_distance < self.blocks[neighbour].weight:
                        self.blocks[neighbour].weight = alt_distance
        return


def parse_input():
    with open('../example.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    blocks = {}
    n = len(data) - 1
    m = len(data[0]) - 1
    for i, row in enumerate(data):
        for j, c in enumerate(row):
            coord = Coord(i, j)
            if (i, j) == (0, 0):
                type_ = BlockType.START
            elif (i, j) == (n, m):
                type_ = BlockType.END
            else:
                type_ = BlockType.NONE
            blocks[coord] = Block(coord, int(c), 999999999, type_)
    return Network(blocks)


def part1():
    network = parse_input()
    network.dijkstra()
    network.plot()
    print(f'Part 1 Answer {network.end_block.weight}')


def part2():
    answer = 0
    print(f'Part 2 Answer {answer}')


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
