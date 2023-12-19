from dataclasses import dataclass
from typing import Callable, Union
from enum import Enum


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
        print(destination_)
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
                    new_func = lambda x: self.func_template(x,
                                                            prop,
                                                            amount,
                                                            destination,
                                                            Comparator.GREATER)
                elif '<' in comparison:
                    binding2 = comparison.split('<')
                    prop = Property[binding2[0].upper()]
                    amount = int(binding2[1])
                    new_func = lambda x: self.func_template(x,
                                                            prop,
                                                            amount,
                                                            destination,
                                                            Comparator.LESSER)
            else:
                comparison = ''
                destination = fstring
                new_func = lambda x: self.func_template(x,
                                                        Property.NONE,
                                                        0,
                                                        destination,
                                                        Comparator.NOTHING)
            new_functions.append(new_func)
        
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


def part1():
    system, gears = parse_input()
    system.initialise_end_workflows()
    for gear in gears:
        system.workflows['in'](gear)
    print(system.get_answer())


def part2():
    pass


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
