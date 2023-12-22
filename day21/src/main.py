import pathlib
from dataclasses import dataclass
from collections import namedtuple
import plotext as plt
import plotille

Point = namedtuple('Point', ['x', 'y'])

@dataclass
class Coord:
    point: Point
    x_iteration: int
    y_iteration: int


@dataclass
class Garden:
    edges: set[Point]
    ground: set[Point]
    rocks: set[Point]
    start: Coord
    xmax: int
    ymax: int
    figure: plotille.Figure = None

    def unique_squares_after_many_steps(self, n: int):
        queue = [self.start]
        for _k in range(n):
            #self.plot(queue, f'{_k+1}/{n} steps traversed, # possible endpoints: {len(queue)}', _k==n-1)
            new_queue = []
            for square in queue:
                for coord in self.get_next_coords(square):
                    if coord.point in self.ground and coord.point not in self.rocks and coord not in new_queue:
                        new_queue.append(coord)
            queue = new_queue
            #self.plot(queue, f'{_k+1}/{n} steps traversed, # possible endpoints: {len(queue)}', _k==n-1)
            print(f'{_k+1}/{n} steps traversed, # possible endpoints: {len(queue)}')
        return queue


    def plot(self, coords: list[Coord], title: str, end=False):
        if self.figure is None:
            self.figure = plotille.Figure()
            self.figure.set_x_limits(-5, 136)
            self.figure.set_y_limits(-5, 136)


        xs = [c.point.x for c in coords]
        ys = [c.point.y for c in coords]
        rock_x = [p.x for p in self.rocks]
        rock_y = [p.y for p in self.rocks]
        ground_x = [p.x for p in self.ground]
        ground_y = [p.y for p in self.ground]
        plt.xlim(-10, 136)
        plt.ylim(-10, 136)
        plt.title(title)
        plt.scatter(ground_x, ground_y, marker='.', color='black')
        plt.scatter(rock_x, rock_y, marker='#', color='gray')

        plt.scatter(xs, ys, marker='o', color='green')
        plt.show()
        if not end:
            plt.clear_data()
            plt.clear_terminal()


    def get_next_coords(self, coord: Coord):
        west = Coord(Point(coord.point.x - 1, coord.point.y), coord.x_iteration, coord.y_iteration)
        east = Coord(Point(coord.point.x + 1, coord.point.y), coord.x_iteration, coord.y_iteration)
        north = Coord(Point(coord.point.x, coord.point.y - 1), coord.x_iteration, coord.y_iteration)
        south = Coord(Point(coord.point.x, coord.point.y + 1), coord.x_iteration, coord.y_iteration)
        if coord.point in self.edges:
            if west.point.x < 0:
                west.point = Point(self.xmax, west.point.y)
                west.x_iteration -= 1

            elif east.point.x > self.xmax:
                east.point = Point(0, east.point.y)
                east.x_iteration += 1

            if north.point.y < 0:
                north.point = Point(north.point.x, self.ymax)
                north.y_iteration -= 1

            elif south.point.y > self.ymax:
                south.point = Point(south.point.x, 0)
                south.y_iteration += 1
        return [east, north, west, south]


def parse_input(fpath: pathlib.Path):
    with open(fpath) as f:
        data = [s.strip('\n') for s in f.readlines()]
    ground = set()
    rocks = set()
    edges = set()
    xmax = len(data[0])
    ymax = len(data)
    for j, row in enumerate(data):
        edges.add(Point(0, j))
        edges.add(Point(xmax, j))
        for i, c in enumerate(row):
            edges.add(Point(i, 0))
            edges.add(Point(i, ymax))
            match c:
                case '.':
                    ground.add(Point(i, j))
                case '#':
                    rocks.add(Point(i, j))
                case 'S':
                    ground.add(Point(i, j))
                    start = Coord(Point(i, j), 0, 0)
    return Garden(edges, ground, rocks, start, xmax, ymax)


def part1():
    garden = parse_input(fpath=pathlib.Path('../input.txt'))
    result = garden.unique_squares_after_many_steps(65)
    print(f'Part 1 Answer: {len(result)}')


def main():
    part1()

if __name__ == '__main__':
    main()