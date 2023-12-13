from dataclasses import dataclass
import pathlib
from enum import Enum
import time
import copy


class SliceType(Enum):
    ROWS = 1
    COLS = 2


@dataclass
class Valley:
    rows: list[list[str]]
    _cols: list[list[str]] = None

    @property
    def cols(self):
        if self._cols is None:
            self._cols = list(map(list, zip(*self.rows)))
        return self._cols

    def display(self):
        print('\n'.join([''.join(r) for r in self.rows]))

    def check_for_reflection_point_in_rows(self) -> tuple[bool, int]:
        return self.find_reflection_point_in(SliceType.ROWS)

    def check_for_reflection_point_in_cols(self) -> tuple[bool, int]:
        return self.find_reflection_point_in(SliceType.COLS)

    def find_reflection_point_in(self, slice_type: SliceType):
        match slice_type:
            case SliceType.ROWS:
                data = self.rows[:]
            case SliceType.COLS:
                data = self.cols[:]
        reflection_found = False
        reflection_number = 0
        for mirror in range(1, len(data)):
            section1 = data[:mirror]
            section2 = data[mirror:]
            if mirror <= len(data) // 2:
                section2 = section2[:len(section1)]
            else:
                section1 = section1[-len(section2):]
            if section1 == section2[::-1]:
                reflection_found = True
                reflection_number = mirror
                break
        if reflection_found:
            return reflection_found, reflection_number
        else:
            return reflection_found, reflection_number

    def find_reflection_number(self):
        reflection_row, number_row = self.check_for_reflection_point_in_rows()
        reflection_col, number_col = self.check_for_reflection_point_in_cols()
        return (number_col, number_row*100)

    def all_possible_smudges(self):
        new = []
        for i, row in enumerate(self.rows):
            for j, c in enumerate(row):
                new_rows = copy.deepcopy(self.rows)
                new_rows[i][j] = flip(new_rows[i][j])
                new.append(Valley(new_rows))
        return new


def flip(c: str):
    match c:
        case '#':
            return '.'
        case '.':
            return '#'

def parse_input(fname: pathlib.Path) -> list[Valley]:
    with open(fname) as f:
        data = [s.strip('\n') for s in f.readlines()]
    block = []
    valleys = []
    for row in data:
        if not row:
            valleys.append(Valley(rows=block[:]))
            block = []
        else:
            block.append([c for c in row])
    valleys.append(Valley(rows=block[:]))
    return valleys


def part1():
    valleys = parse_input(pathlib.Path('../input.txt'))
    totals = [v.find_reflection_number() for v in valleys]
    print(f'Part 1 Answer: {sum([t[0]+t[1] for t in totals])}')

def part2():
    valleys = parse_input(pathlib.Path('../input.txt'))
    total = 0
    for valley in valleys:
        initial_result = valley.find_reflection_number()
        results = [v.find_reflection_number() for v in valley.all_possible_smudges()]
        for result in results:
            if initial_result[0] != result[0] and result[0] != 0:
                total += result[0]
                break
            if initial_result[1] != result[1] and result[1] != 0:
                total += result[1]
                break
    print(f'Part 2 Answer: {total}')


def main():
    start_time = time.time()
    part1()
    print("Part 1--- %s seconds ---" % (time.time() - start_time))
    part2()


if __name__ == '__main__':
    main()