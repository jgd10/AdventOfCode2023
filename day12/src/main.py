from dataclasses import dataclass
import time


@dataclass
class PuzzleLine:
    springs: str
    clues: list[int]


def get_number_of_variants(line: str, clues: list[int], cache: dict[tuple[str, tuple[int]], int]) -> int:
    key = (line, tuple(clues))
    if key in cache:
        return cache[key]
    num = 0
    if not line and clues:
        cache[key] = 0
        return 0
    if not clues and line:
        if '#' in line:
            cache[key] = 0
            return 0
        else:
            cache[key] = 1
            return 1
    if len(line) < sum(clues) + len(clues) - 1:
        cache[key] = 0
        return 0
    if not line and not clues:
        cache[key] = 1
        return 1
    if line.startswith('.'):
        num = get_number_of_variants(line[1:], clues, cache)
    elif line.startswith('#'):
        if {c for c in line[:clues[0]]}.issubset({'#', '?'}):
            if clues[0] > len(line):
                cache[key] = 0
                return 0
            elif clues[0] == len(line):
                return 1
            elif line[clues[0]] == '#':
                cache[key] = 0
                return 0
            else:
                num = get_number_of_variants(line[clues[0]+1:], clues[1:], cache)
        else:
            cache[key] = 0
            return 0
    elif line.startswith('?'):
        num = get_number_of_variants('.' + line[1:], clues, cache) \
              + get_number_of_variants('#' + line[1:], clues, cache)
    cache[key] = num
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
        counter = get_number_of_variants(line.springs, line.clues, {})
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
        counter = get_number_of_variants(line.springs, line.clues, {})
        total += counter
    print(f'Part 1 Answer = {total}')


def test_input():
    lines = ['???', '#', '#.', '#?', '##', '###', '##.', '##?', '#?#', '#?#??']
    expected = [2, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0]
    clues = [[2], [2], [2], [2], [2], [2], [2], [2], [2], [2, 1]]
    for expect, line, clue in zip(expected, lines, clues):
        n = get_number_of_variants(line, clue, {})
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
