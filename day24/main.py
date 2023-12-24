from dataclasses import dataclass
import numpy as np
import pathlib
from collections import namedtuple


@dataclass
class Vector:
    x: int
    y: int
    z: int


@dataclass
class Hailstone:
    s: Vector
    v: Vector
    _xygrad: float = None
    _xyconst: float = None

    @property
    def m_xy(self):
        if self._xygrad is None:
            self._xygrad = self.v.y/self.v.x
        return self._xygrad

    @property
    def c_xy(self):
        if self._xyconst is None:
            self._xyconst = self.s.y - self.m_xy*self.s.x
        return self._xyconst

# Hailstone = namedtuple('Hailstone', ['s', 'v'])
Limits = namedtuple('Limits', ['min', 'max'])


@dataclass
class Blizzard:
    x: np.ndarray
    y: np.ndarray
    z: np.ndarray
    vx: np.ndarray
    vy: np.ndarray
    vz: np.ndarray
    stones: list[Hailstone]

    @classmethod
    def from_file(cls, fname: pathlib.Path):
        with open(fname) as f:
            data = [s.strip('\n') for s in f.readlines()]
        x_, y_, z_ = [], [], []
        vx_, vy_, vz_ = [], [], []
        stones = []
        for row in data:
            position, velocity = row.split(' @ ')
            positions = position.split(',')
            x, y, z = int(positions[0]), int(positions[1]), int(positions[2])
            vels = velocity.split(',')
            vx, vy, vz = int(vels[0]), int(vels[1]), int(vels[2])
            x_.append(x)
            y_.append(y)
            z_.append(z)
            vx_.append(vx)
            vy_.append(vy)
            vz_.append(vz)
            vel = Vector(vx, vy, vz)
            pos = Vector(x, y, z)
            stones.append(Hailstone(pos, vel))
        return cls(np.array(x_), np.array(y_), np.array(z_), np.array(vx_), np.array(vy_), np.array(vz_), stones)

    def will_hailstones_enter_xy_slice(self, limits: Limits):
        counter = 0
        pairs_done = []
        for stone1 in self.stones:
            for stone2 in self.stones:
                pair = [stone1, stone2]
                if stone2 == stone1 or pair in pairs_done:
                    pass
                else:
                    pairs_done.append(pair)
                    pairs_done.append(pair[::-1])
                    if stone2.m_xy != stone1.m_xy:
                        x = (stone2.c_xy - stone1.c_xy) / (stone1.m_xy - stone2.m_xy)
                        y = stone1.m_xy*x + stone1.c_xy
                        t1 = (x - stone1.s.x) / stone1.v.x
                        t2 = (x - stone2.s.x) / stone2.v.x
                        if (limits.min <= x <= limits.max) and (limits.min <= y <= limits.max) and (t1>0 and t2>0):
                            counter += 1
                    else:
                        continue
        return counter


def part1():
    blizzard = Blizzard.from_file(pathlib.Path('./input.txt'))
    answer = blizzard.will_hailstones_enter_xy_slice(limits=Limits(200000000000000, 400000000000000))
    print(answer)

def part2():
    pass


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
