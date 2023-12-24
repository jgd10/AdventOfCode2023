from dataclasses import dataclass
import numpy as np
import pathlib
from collections import namedtuple
import sympy as sym


@dataclass
class Vector:
    x: int
    y: int
    z: int

    def cross(self, other):
        return Vector(self.y * other.z - self.z * other.y,
                      self.z * other.x - self.x * other.z,
                      self.x * other.y - self.y * other.x)

    def is_zero(self):
        return self.x == 0 and self.y == 0 and self.z == 0

@dataclass
class Hailstone:
    id_: int
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
        for i, row in enumerate(data):
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
            stones.append(Hailstone(i, pos, vel))
        return cls(np.array(x_), np.array(y_), np.array(z_), np.array(vx_), np.array(vy_), np.array(vz_), stones)

    def parallel_vectors(self):
        counter = 0
        pairs_done = set()
        for stone1 in self.stones:
            for stone2 in self.stones:
                pair = frozenset([stone1.id_, stone2.id_])
                if stone2.id_ == stone1.id_ or pair in pairs_done:
                    pass
                else:
                    vec = stone1.v.cross(stone2.v)
                    print(vec)
                    if vec.is_zero():
                        counter += 1
        return counter

    def will_hailstones_enter_xy_slice(self, limits: Limits):
        counter = 0
        pairs_done = set()
        for stone1 in self.stones:
            for stone2 in self.stones:
                pair = frozenset([stone1.id_, stone2.id_])
                if stone2.id_ == stone1.id_ or pair in pairs_done:
                    pass
                else:
                    pairs_done.add(pair)
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
    answer = blizzard.will_hailstones_enter_xy_slice(
        limits=Limits(200_000_000_000_000, 400_000_000_000_000)
    )
    print(answer)


def part2():
    blizzard = Blizzard.from_file(pathlib.Path('./input.txt'))
    symbols = sym.symbols(
        'x0,y0,z0,u0,v0,w0,t1,t2,t3'
    )
    x0, y0, z0, u0, v0, w0, t1, t2, t3 = symbols
    equations = []
    times = [t1, t2, t3]
    for stone, t in zip(blizzard.stones[:3], times):
        eqx = sym.Eq(x0 + u0 * t, stone.v.x * t + stone.s.x)
        eqy = sym.Eq(y0 + v0 * t, stone.v.y * t + stone.s.y)
        eqz = sym.Eq(z0 + w0 * t, stone.v.z * t + stone.s.z)
        equations.extend([eqx, eqy, eqz])
    result = sym.solve(equations, symbols)[0]
    print(sum([r for r in result[:3]]))


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
