import functools
from dataclasses import dataclass
import time


@dataclass
class PuzzleLine:
    springs: str
    clues: list[int]


@functools.cache
def get_number_of_variants(line: str, clues: tuple[int]) -> int:
    clues = list(clues)
    num = 0
    if not line and clues:
        return 0
    if not clues and line:
        if '#' in line:
            return 0
        else:
            return 1
    if len(line) < sum(clues) + len(clues) - 1:
        return 0
    if not line and not clues:
        return 1
    if line.startswith('.'):
        num = get_number_of_variants(line[1:], tuple(clues))
    elif line.startswith('#'):
        if {c for c in line[:clues[0]]}.issubset({'#', '?'}):
            if clues[0] > len(line):
                return 0
            elif clues[0] == len(line):
                return 1
            elif line[clues[0]] == '#':
                return 0
            else:
                num = get_number_of_variants(line[clues[0]+1:], tuple(clues[1:]))
        else:
            return 0
    elif line.startswith('?'):
        num = get_number_of_variants('.' + line[1:], tuple(clues)) \
              + get_number_of_variants('#' + line[1:], tuple(clues))
    return num


def get_input_data():
    with open('../input.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    return data


def part2():
    total = 0
    data = get_input_data()
    lines = []
    for row in data:
        spring_str, count_str = row.split(' ')
        springs = spring_str + '?'
        springs *= 5
        springs = springs[:-1]
        counts = [int(c) for c in count_str.split(',')]*5
        lines.append(PuzzleLine(springs, counts))
    for i, line in enumerate(lines):
        counter = get_number_of_variants(line.springs, tuple(line.clues))
        total += counter
    print(f'Part 2 Answer = {total}')


def part1():
    lines = []
    data = get_input_data()
    for row in data:
        spring_str, count_str = row.split(' ')
        counts = [int(c) for c in count_str.split(',')]
        lines.append(PuzzleLine(spring_str, counts))
    total = 0
    for i, line in enumerate(lines):
        counter = get_number_of_variants(line.springs, tuple(line.clues))
        total += counter
    print(f'Part 1 Answer = {total}')


def test_input():
    lines = ['???', '#', '#.', '#?', '##', '###', '##.', '##?', '#?#', '#?#??']
    expected = [2, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]
    clues = [[2], [2], [2], [2], [2], [2], [2], [2], [2], [2, 1]]
    for expect, line, clue in zip(expected, lines, clues):
        n = get_number_of_variants(line, tuple(clue))
        print(line, clue, f'Found {n}, Expected {expect}')


def main():
    start_time = time.time()
    part1()
    print("Part 1--- %s seconds ---" % (time.time() - start_time))
    part2_start = time.time()
    part2()
    print("Part 2--- %s seconds ---" % (time.time() - part2_start))


if __name__ == '__main__':
    # test_input()
    main()
