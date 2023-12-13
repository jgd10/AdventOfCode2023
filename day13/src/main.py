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
        return '\n'.join([''.join(r) for r in self.rows])

    def check_for_reflection_point_in_rows(self) -> tuple[bool, list[int]]:
        return self.find_reflection_point_in(SliceType.ROWS)

    def check_for_reflection_point_in_cols(self) -> tuple[bool, list[int]]:
        return self.find_reflection_point_in(SliceType.COLS)

    def find_reflection_point_in(self, slice_type: SliceType):
        match slice_type:
            case SliceType.ROWS:
                pattern = copy.deepcopy(self.rows)
            case SliceType.COLS:
                pattern = copy.deepcopy(self.cols)
            case _:
                raise TypeError
        reflection_found = False
        reflection_numbers = []
        for mirror in range(1, len(pattern)):
            section1 = pattern[:mirror]
            section2 = pattern[mirror:]
            if mirror <= len(pattern) // 2:
                section2 = section2[:len(section1)]
            else:
                section1 = section1[-len(section2):]
            if section1 == section2[::-1]:
                reflection_found = True
                reflection_numbers.append(mirror)

        return reflection_found, reflection_numbers

    def find_reflection_number(self):
        reflection_row, number_row = self.check_for_reflection_point_in_rows()
        reflection_col, number_col = self.check_for_reflection_point_in_cols()
        return number_col, [i*100 for i in number_row]

    def all_possible_smudges(self):
        new = []
        n = len(self.rows)
        m = len(self.rows[0])
        answer = """.#.###.....
#..#.##...#
.#.....#.#.
.#.....#.#.
#..#.##...#
.#.###.....
####..##.##
#....##..#.
.##.##.#.##
...#..###.#
...#..###.#
.##.##.#.##
#....##..#.
####..##.##
.#.###.....
#..#.##...#
.#.....#.#."""
        for i in range(n):
            for j in range(m):
                new_rows = copy.deepcopy(self.rows)
                new_rows[i][j] = flip(new_rows[i][j])
                v = Valley(new_rows)
                new.append(v)
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
    print(f'Part 1 Answer: {sum([sum(t[0]) + sum(t[1]) for t in totals])}')

def part2():
    valleys = parse_input(pathlib.Path('../input.txt'))
    total = 0
    answers = []
    old_answers = []
    for i, valley in enumerate(valleys):
        initial_result = valley.find_reflection_number()
        initial_number = {j for entry in initial_result for j in entry}.pop()
        old_answers.append(max(initial_result))
        results = [v.find_reflection_number() for v in valley.all_possible_smudges()]
        for result in results:
            numbers = {j for entry in result for j in entry}
            if numbers and numbers != {initial_number}:
                if i in [19, 20, 21, 22]:
                    print()
                if initial_number in numbers:
                    numbers.remove(initial_number)
                num = numbers.pop()
                answers.append(num)
                total += num
                break
    print(answers)
    assert(len(answers) == len(valleys))
    print(f'Part 2 Answer: {total}')


def main():
    start_time = time.time()
    part1()
    print("Part 1--- %s seconds ---" % (time.time() - start_time))
    part2()


if __name__ == '__main__':
    main()
