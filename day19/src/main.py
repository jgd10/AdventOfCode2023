from dataclasses import dataclass
from typing import Callable, Optional
from enum import Enum
from copy import deepcopy


class Comparator(Enum):
    GREATER = '>'
    LESSER = '<'
    NOTHING = ':'


class Property(Enum):
    X = 1,
    M = 2
    A = 3
    S = 4
    NONE = 5


@dataclass
class Gear:
    x: int
    m: int
    a: int
    s: int

    @classmethod
    def from_dict(cls, values: dict[str, int]):
        return cls(values['x'], values['m'], values['a'], values['s'])

    def sum_props(self):
        return self.x + self.m + self.a + self.s


@dataclass
class Range:
    min: int
    max: int

    @property
    def span(self):
        return self.max - self.min + 1


@dataclass
class GearRange:
    x: Range
    m: Range
    a: Range
    s: Range

    @classmethod
    def init(cls):
        x = Range(1, 4000)
        m = Range(1, 4000)
        a = Range(1, 4000)
        s = Range(1, 4000)
        return cls(x, m, a, s)

    def new_x(self, x: Range):
        return GearRange(x, self.m, self.a, self.s)

    def new_m(self, m: Range):
        return GearRange(self.x, m, self.a, self.s)

    def new_a(self, a: Range):
        return GearRange(self.x, self.m, a, self.s)

    def new_s(self, s: Range):
        return GearRange(self.x, self.m, self.a, s)

    def possible_permutations(self):
        return self.x.span*self.m.span*self.a.span*self.s.span


@dataclass
class System:
    workflows: dict[str, Callable]
    accepted: list[Gear]
    rejected: list[Gear]

    def get_answer(self):
        return sum([g.sum_props() for g in self.accepted])

    def accept(self, gear: Gear):
        self.accepted.append(gear)

    def reject(self, gear: Gear):
        self.rejected.append(gear)

    def initialise_end_workflows(self):
        self.workflows['A'] = lambda x: self.accept(x)
        self.workflows['R'] = lambda x: self.reject(x)

    def func_template(self, gear_: Gear, property_: Property, amount_: int,
                      destination_: str, comparator_: Comparator) -> bool:
        match comparator_:
            case Comparator.GREATER:
                if self.greater_than(gear_, property_, amount_):
                    self.workflows[destination_](gear_)
                    return True
                else:
                    return False
            case Comparator.LESSER:
                if self.less_than(gear_, property_, amount_):
                    self.workflows[destination_](gear_)
                    return True
                else:
                    return False
            case Comparator.NOTHING:
                self.workflows[destination_](gear_)
                return True
        return False

    def parse_workflow(self, line: str) -> None:
        parts = line.split('{')
        name = parts.pop(0)
        functions = parts[0].removesuffix('}').split(',')
        new_functions = []
        for fstring in functions:
            if ':' in fstring:
                binding = fstring.split(':')
                comparison = binding[0]
                destination = binding[1]
                if '>' in comparison:
                    binding2 = comparison.split('>')
                    prop = Property[binding2[0].upper()]
                    amount = int(binding2[1])
                    comparator = Comparator.GREATER
                elif '<' in comparison:
                    binding2 = comparison.split('<')
                    prop = Property[binding2[0].upper()]
                    amount = int(binding2[1])
                    comparator = Comparator.LESSER
            else:
                comparison = ''
                destination = fstring
                prop = Property.NONE
                amount = 0
                comparator = Comparator.NOTHING

            def workflow_function(x,
                                  prop=prop,
                                  amount=amount,
                                  destination=destination,
                                  comparator=comparator):
                return self.func_template(x,
                                          property_=prop,
                                          amount_=amount,
                                          destination_=destination,
                                          comparator_=comparator)
            new_functions.append(workflow_function)
        
        self.workflows[name] = lambda x: self.workflow(x, new_functions)

    def workflow(self, gear: Gear, function_series: list[Callable]):
        for function in function_series:
            result = function(gear)
            if result:
                break

    def greater_than(self, gear: Gear, property: Property, amount: int) -> bool:
        match property:
            case Property.X:
                return gear.x > amount
            case Property.M:
                return gear.m > amount
            case Property.A:
                return gear.a > amount
            case Property.S:
                return gear.s > amount

    def less_than(self, gear: Gear, property: Property, amount: int) -> bool:
        match property:
            case Property.X:
                return gear.x < amount
            case Property.M:
                return gear.m < amount
            case Property.A:
                return gear.a < amount
            case Property.S:
                return gear.s < amount


@dataclass
class System2:
    workflows: dict[str, Callable]
    accepted: list[GearRange]
    rejected: list[GearRange]

    def combine_accepted_ranges(self):
        min_x = 4001
        max_x = 0
        min_m = 4001
        max_m = 0
        min_a = 4001
        max_a = 0
        min_s = 4001
        max_s = 0
        for g in self.accepted:
            min_x = max(min_x, g.x.min)
            max_x = min(max_x, g.x.max)
            min_m = max(min_m, g.m.min)
            max_m = min(max_m, g.m.max)
            min_a = max(min_a, g.a.min)
            max_a = min(max_a, g.a.max)
            min_s = max(min_s, g.s.min)
            max_s = min(max_s, g.s.max)
        return GearRange(Range(min_x, max_x), Range(min_m, max_m), Range(min_a, max_a), Range(min_s, max_s))

    def accept(self, gear: GearRange):
        self.accepted.append(gear)

    def reject(self, gear: GearRange):
        self.rejected.append(gear)

    def initialise_end_workflows(self):
        self.workflows['A'] = lambda x: self.accept(x)
        self.workflows['R'] = lambda x: self.reject(x)

    def func_template(self, gear_: GearRange, property_: Property, amount_: int,
                      destination_: str, comparator_: Comparator) \
            -> Optional[GearRange]:
        match comparator_:
            case Comparator.GREATER:
                gear_success, gear_fail = self.greater_than(gear_,
                                                            property_,
                                                            amount_)
            case Comparator.LESSER:
                gear_success, gear_fail = self.less_than(gear_,
                                                         property_,
                                                         amount_)

            case Comparator.NOTHING:
                gear_success = gear_
                gear_fail = None
            case _:
                raise TypeError
        self.workflows[destination_](gear_success)
        return gear_fail

    def parse_workflow(self, line: str) -> None:
        parts = line.split('{')
        name = parts.pop(0)
        functions = parts[0].removesuffix('}').split(',')
        new_functions = []
        for fstring in functions:
            if ':' in fstring:
                binding = fstring.split(':')
                comparison = binding[0]
                destination = binding[1]
                if '>' in comparison:
                    binding2 = comparison.split('>')
                    prop = Property[binding2[0].upper()]
                    amount = int(binding2[1])
                    comparator = Comparator.GREATER
                elif '<' in comparison:
                    binding2 = comparison.split('<')
                    prop = Property[binding2[0].upper()]
                    amount = int(binding2[1])
                    comparator = Comparator.LESSER
            else:
                comparison = ''
                destination = fstring
                prop = Property.NONE
                amount = 0
                comparator = Comparator.NOTHING

            def workflow_function(x,
                                  prop=prop,
                                  amount=amount,
                                  destination=destination,
                                  comparator=comparator):
                return self.func_template(x,
                                          property_=prop,
                                          amount_=amount,
                                          destination_=destination,
                                          comparator_=comparator)

            new_functions.append(workflow_function)

        self.workflows[name] = lambda x: self.workflow(x, new_functions)

    def workflow(self, gear: Gear, function_series: list[Callable]):
        result = gear
        for function in function_series:
            if result is not None:
                result = function(result)
            else:
                break

    def greater_than(self, gear: GearRange, property: Property,
                     amount: int) -> tuple[GearRange, GearRange]:
        match property:
            case Property.X:
                success_min = amount + 1
                success_max = gear.x.max
                failure_min = gear.x.min
                failure_max = amount
                return gear.new_x(Range(success_min, success_max)), \
                    gear.new_x(Range(failure_min, failure_max))
            case Property.M:
                success_min = amount + 1
                success_max = gear.m.max
                failure_min = gear.m.min
                failure_max = amount
                return gear.new_m(Range(success_min, success_max)), \
                    gear.new_m(Range(failure_min, failure_max))
            case Property.A:
                success_min = amount + 1
                success_max = gear.a.max
                failure_min = gear.a.min
                failure_max = amount
                return gear.new_a(Range(success_min, success_max)), \
                    gear.new_a(Range(failure_min, failure_max))
            case Property.S:
                success_min = amount + 1
                success_max = gear.s.max
                failure_min = gear.s.min
                failure_max = amount
                return gear.new_s(Range(success_min, success_max)), \
                    gear.new_s(Range(failure_min, failure_max))

    def less_than(self, gear: GearRange, property: Property,
                     amount: int) -> tuple[GearRange, GearRange]:
        match property:
            case Property.X:
                success_min = gear.x.min
                success_max = amount - 1
                failure_min = amount
                failure_max = gear.x.max
                return gear.new_x(Range(success_min, success_max)), \
                    gear.new_x(Range(failure_min, failure_max))
            case Property.M:
                success_min = gear.m.min
                success_max = amount - 1
                failure_min = amount
                failure_max = gear.m.max
                return gear.new_m(Range(success_min, success_max)), \
                    gear.new_m(Range(failure_min, failure_max))
            case Property.A:
                success_min = gear.a.min
                success_max = amount - 1
                failure_min = amount
                failure_max = gear.a.max
                return gear.new_a(Range(success_min, success_max)), \
                    gear.new_a(Range(failure_min, failure_max))
            case Property.S:
                success_min = gear.s.min
                success_max = amount - 1
                failure_min = amount
                failure_max = gear.s.max
                return gear.new_s(Range(success_min, success_max)), \
                    gear.new_s(Range(failure_min, failure_max))


def parse_input() -> tuple[System, list[Gear]]:
    with open('../example.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    parts = [row for row in data if row.startswith('{')]
    flows = [row for row in data if not row.startswith('{') and row]
    system = System({}, [], [])
    for flow in flows:
        system.parse_workflow(flow)

    gears = []
    for part in parts:
        part = part.removeprefix('{')
        part = part.removesuffix('}')
        value_strings = part.split(',')
        values = {}
        for v_string in value_strings:
            name, val = v_string.split('=')
            values[name] = int(val)
        gears.append(Gear.from_dict(values))
    return system, gears


def parse_input2():
    with open('../input.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    flows = [row for row in data if not row.startswith('{') and row]
    system = System2({}, [], [])
    for flow in flows:
        system.parse_workflow(flow)
    system.initialise_end_workflows()
    return system


def part1():
    system, gears = parse_input()
    system.initialise_end_workflows()
    for gear in gears:
        system.workflows['in'](gear)
    print(f'Part 1 Answer: {system.get_answer()}')


def part2():
    system2 = parse_input2()
    starting_gear = GearRange.init()
    system2.workflows['in'](starting_gear)
    total = 0
    for gear in system2.accepted:
        total += gear.possible_permutations()
    print(f'Part 2 Answer: {total}')


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
