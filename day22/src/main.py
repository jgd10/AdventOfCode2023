from dataclasses import dataclass
from typing import Collection


@dataclass(frozen=True)
class Coord:
    x: int
    y: int
    z: int

    def __add__(self, other: 'Coord'):
        return Coord(self.x+other.x, self.y+other.y, self.z+other.z)

    def drop(self):
        return Coord(self.x, self.y, self.z-1)

    def to_string(self):
        return f'{self.x},{self.y},{self.z}'


@dataclass
class Brick:
    ends: tuple[Coord]
    supporting: list[tuple[Coord, Coord]] = None
    supports: list[tuple[Coord, Coord]] = None
    _coords: set[Coord] = None

    def __repr__(self):
        return f'Brick({self.ends[0], self.ends[1]}'

    def to_string(self):
        return f'{self.ends[0].to_string()}~{self.ends[1].to_string()}'

    @classmethod
    def from_string(cls, string: str):
        end0, end1 = string.split('~')
        x0, y0, z0 = end0.split(',')
        x1, y1, z1 = end1.split(',')
        coord0 = Coord(int(x0), int(y0), int(z0))
        coord1 = Coord(int(x1), int(y1), int(z1))
        return cls((coord0, coord1))

    def drop(self) -> 'Brick':
        new_ends = tuple([e.drop() for e in self.ends])
        return Brick(new_ends)

    def base_copy(self) -> 'Brick':
        return Brick(self.ends)

    @property
    def coords(self) -> set[Coord]:
        if self._coords is None:
            self._coords = set()
            xdiff = self.ends[1].x - self.ends[0].x
            ydiff = self.ends[1].y - self.ends[0].y
            zdiff = self.ends[1].z - self.ends[0].z
            for i in range(xdiff+1):
                for j in range(ydiff+1):
                    for k in range(zdiff+1):
                        self._coords.add(self.ends[0] + Coord(i, j, k))
        return self._coords


@dataclass
class Tower:
    bricks: dict[tuple[Coord, Coord], Brick]
    _cache: dict[tuple[Coord, Coord], set[tuple[Coord, Coord]]] = None

    def let_bricks_fall(self):
        fallen_bricks = {}
        all_bricks = [b for b in self.bricks.values()]
        all_bricks.sort(key=lambda b: b.ends[0].z)
        counter = 0
        for brick in all_bricks:
            counter += 1
            print(f'{counter} bricks complete')
            falling_brick = brick.base_copy()
            while self.brick_in_freefall(falling_brick, fallen_bricks):
                falling_brick = falling_brick.drop()
            fallen_bricks[falling_brick.ends] = falling_brick
        self.bricks = fallen_bricks
        self.save()

    def save(self):
        with open('../example3.txt', 'w') as f:
            for brick in self.bricks.values():
                f.write(f'{brick.to_string()}\n')

    def brick_in_freefall(self, brick: Brick, fallen: dict[tuple[Coord, Coord], Brick]):
        bricks = [b for b in fallen.values()]
        # bricks.sort(key=lambda b: b.ends[0].z, reverse=True)
        for coord in brick.coords:
            c = Coord(coord.x, coord.y, coord.z-1)
            if c.z < 1:
                return False
            for brick2 in bricks:
                if c in brick2.coords:
                    return False
        return True

    def find_supporting(self, brick: Brick):
        supporting = set()
        for coord in brick.coords:
            c = Coord(coord.x, coord.y, coord.z - 1)
            if c not in brick.coords:
                for k, b in self.bricks.items():
                    if c in b.coords:
                        supporting.add(k)
        brick.supporting = list(supporting)

    def find_supports(self, brick: Brick):
        supported = set()
        for coord in brick.coords:
            c = Coord(coord.x, coord.y, coord.z + 1)
            if c not in brick.coords:
                for k, b in self.bricks.items():
                    if c in b.coords:
                        supported.add(k)
        brick.supports = list(supported)

    def get_total_collapse_if_brick_disintegrated(self, brick: Brick) -> set[tuple[Coord, Coord]]:
        disintegrated_bricks = {brick.ends}
        queue = {brick.ends}
        while queue:
            brick1 = self.bricks[queue.pop()]
            for key in brick1.supports:
                b = self.bricks[key]
                if all([support in disintegrated_bricks for support in b.supporting]):
                    disintegrated_bricks.add(key)
                    queue.add(key)
        disintegrated_bricks.remove(brick.ends)
        return disintegrated_bricks

    def get_bricks_that_could_be_disintegrated(self):
        potentials = []
        for key, brick in self.bricks.items():
            if brick.supports:
                other_supports = set()
                for supported_brick in brick.supports:
                    if len(self.bricks[supported_brick].supporting) > 1:
                        other_supports.add(True)
                    else:
                        other_supports.add(False)
                if all(other_supports):
                    potentials.append(brick)
            else:
                potentials.append(brick)
        return potentials



def main():
    with open('../input3.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    bricks = {}
    for row in data:
        brick = Brick.from_string(row)
        bricks[brick.ends] = brick
    tower = Tower(bricks)
    #tower.let_bricks_fall()

    #all_brick_coords = {k: b.coords for k, b in tower.bricks.items()}
    for brick in tower.bricks.values():
        tower.find_supports(brick)
        tower.find_supporting(brick)
    print('Done!')
    disintegrated = tower.get_bricks_that_could_be_disintegrated()
    print(f'Part 1 Answer: {len(disintegrated)}')
    total = 0
    for brick in tower.bricks.values():
        collapsed = tower.get_total_collapse_if_brick_disintegrated(brick)
        total += len(collapsed)
    print(f'Part 2 Answer: {total}')

if __name__ == '__main__':
    main()