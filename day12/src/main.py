from itertools import product
from dataclasses import dataclass
from typing import Collection
import time


def get_counts_from_arrangement(arrangement: Collection[int]):
    counter1 = 0
    counts = []
    for i in arrangement:
        if i == 1:
            counter1 += 1
        else:
            if counter1 > 0:
                counts.append(counter1)
                counter1 = 0
    if counter1 > 0:
        counts.append(counter1)
    return counts


def check_counts_from_arrangement(counts: list[int],
                                  arrangement: Collection[int]):
    counter1 = 0
    counts_index = 0
    for i, element in enumerate(arrangement):
        if element == 1:
            counter1 += 1
        else:
            if counter1 > 0:
                if counts[counts_index] != counter1:
                    return False
                else:
                    counter1 = 0
                counts_index += 1
    if counter1 > 0 and (counts[counts_index] != counter1):
        return False
    return True


@dataclass
class PuzzleLine:
    springs: list[int]
    counts: list[int]

    @property
    def remaining_springs(self):
        return self.total_springs - self.springs.count(1)

    @property
    def total_springs(self):
        return sum(self.counts)

    @property
    def number_blanks(self):
        return self.springs.count(9)

    def get_all_possibilities(self):
        arrangements = []
        stack = [self.springs]
        while len(stack) > 0:
            value = stack.pop(0)
            for j, element in enumerate(value):
                if element == 9:
                    new0 = value[:]
                    new1 = value[:]
                    new0[j] = 0
                    new1[j] = 1
                    stack.extend([new0, new1])
                    break
            if 9 not in value:
                arrangements.append(value)
        return arrangements

    def get_number_valid_arrangements(self):
        all_possibilities = (a for a in product([0, 1],
                                                repeat=self.number_blanks))
        # self.get_all_possibilities()
        counter = 0
        total_remaining_springs = self.remaining_springs
        for possibility in all_possibilities:
            if sum(possibility) != total_remaining_springs:
                continue
            else:
                new_line = self.fill_line_with_arrangement(list(possibility))
                if self.is_arrangement_valid(new_line):
                    counter += 1
        return counter

    def get_number_valid_arrangements2(self) -> int:
        def can_insert(sequence: list[int], count: int, index: int):
            # Take the region the chunk *could* go into out of the sequence of springs
            insert_region = sequence[index:index+count]
            # If the region isn't long enough that's a fail
            if len(insert_region) != count:
                return False
            # The char after the sequence should either be the end of the list or a 0/9
            if index+count < len(sequence):
                if sequence[index+count] not in [0, 9]:
                    return False
            # Finally, all elements of the sequence should only contain springs or ? (9)
            if set(sequence[index:index+count]) not in [{1, 9}, {1}, {9}]:
                return False

            # the pattern *can* be inserted here!
            return True

        def get_num_poss(sequence, counts):
            total = 0
            # minimum_remaining_space = sum(counts[1:]) + len(counts[1:])
            if len(sequence) < sum(counts) + len(counts) - 1:
                return total
            for i, count in enumerate(counts):
                match sequence[0]:
                    case 0:
                        pass
                    case 1:
                        total += 1
                        sequence = sequence[count+1:]
                    case 9:
                        total += get_num_poss([0] + sequence[1:], counts) + get_num_poss([1] + sequence[1:], counts)
            return total

        return get_num_poss(self.springs, self.counts[:])

    def fill_line_with_arrangement(self, arrangement: list[int]):
        new_spring = []
        for spring in self.springs:
            if spring == 9:
                new_spring.append(arrangement.pop(0))
            else:
                new_spring.append(spring)
        return new_spring

    def is_arrangement_valid(self, arrangement: Collection[int]):
        return check_counts_from_arrangement(self.counts, arrangement)


def part1():
    lines = []
    data = get_input_data()
    for row in data:
        spring_str, count_str = row.split(' ')
        springs = [1 if c == '#' else 0 if c == '.' else 9
                   for c in spring_str]
        counts = [int(c) for c in count_str.split(',')]
        lines.append(PuzzleLine(springs, counts))
    total = 0
    for i, line in enumerate(lines):
        counter = line.get_number_valid_arrangements2()
        print(counter)
        total += counter
    print(f'Part 1 Answer = {total}')


def part2():
    total = 0
    data = get_input_data()
    lines = []
    for row in data:
        spring_str, count_str = row.split(' ')
        springs = [1 if c == '#' else 0 if c == '.' else 9
                   for c in spring_str] + [9]
        springs *= 5
        springs.pop(-1)
        counts = [int(c) for c in count_str.split(',')]*5
        lines.append(PuzzleLine(springs, counts))
    for i, line in enumerate(lines):
        counter = line.get_number_valid_arrangements2()
        print(counter, i)
        total += counter
    print(f'Part 2 Answer = {total}')


def get_input_data():
    with open('../example.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    return data


def main():
    start_time = time.time()
    part1()
    print("Part 1--- %s seconds ---" % (time.time() - start_time))
    part2_time = time.time()
    part2()
    print("Part 2--- %s seconds ---" % (time.time() - part2_time))


if __name__ == '__main__':
    main()
