from dataclasses import dataclass
from enum import Enum
from typing import Union


class Pulse(Enum):
    LOW = 0,
    HIGH = 1,


class FlipFlopStatus(Enum):
    ON = 1,
    OFF = 0,


@dataclass
class TestOutput:
    name: str

    @property
    def targets(self):
        return []

    def pulse(self, pulse_type: Pulse):
        low_counter = 0
        high_counter = 0
        match pulse_type:
            case Pulse.HIGH:
                high_counter += 1
            case Pulse.LOW:
                low_counter += 1
        return low_counter, high_counter


@dataclass
class FlipFlop:
    name: str
    targets: dict[str, Union['FlipFlop', 'Conjunction', TestOutput]]
    status: FlipFlopStatus = FlipFlopStatus.OFF

    def switch(self):
        match self.status:
            case FlipFlopStatus.OFF:
                self.status = FlipFlopStatus.ON
            case FlipFlopStatus.ON:
                self.status = FlipFlopStatus.OFF

    def pulse(self, pulse_type: Pulse):
        low_counter = 0
        high_counter = 0
        match pulse_type:
            case Pulse.LOW:
                self.switch()
                match self.status:
                    case FlipFlopStatus.ON:
                        pulse_type = Pulse.HIGH
                    case FlipFlopStatus.OFF:
                        pulse_type = Pulse.LOW
                for target in self.targets.values():
                    low_count, high_count = target.pulse(pulse_type)
                    low_counter += low_count
                    high_counter += high_count

            case Pulse.HIGH:
                pass
        return low_counter, high_counter


@dataclass
class Conjunction:
    name: str
    inputs: list[Pulse]
    targets: dict[str, Union[FlipFlop, 'Conjunction', TestOutput]]

    def pulse(self, pulse_type: Pulse):
        low_counter = 0
        high_counter = 0
        self.inputs.pop(0)
        self.inputs.append(pulse_type)
        if all([p == Pulse.HIGH for p in self.inputs]):
            next_pulse = Pulse.LOW
        else:
            next_pulse = Pulse.HIGH
        for target in self.targets.values():
            low_count, high_count = target.pulse(next_pulse)
            low_counter += low_count
            high_counter += high_count
        return low_counter, high_counter


@dataclass
class Broadcaster:
    targets: dict[str, Union[FlipFlop, Conjunction, TestOutput]]

    @property
    def name(self):
        return 'broadcaster'

    def pulse(self, pulse_type: Pulse = Pulse.LOW):
        low_counter = 0
        high_counter = 0
        for target in self.targets.values():
            low_count, high_count = target.pulse(pulse_type)
            low_counter += low_count
            high_counter += high_count
        return low_counter, high_counter


@dataclass
class System:
    modules: dict[str, Union[FlipFlop, Broadcaster, Conjunction, TestOutput]]
    low_counter: int = 0
    high_counter: int = 0
    button_counter: int = 0

    @property
    def broadcaster(self):
        return self.modules['broadcaster']

    @property
    def flipflops_state(self):
        return tuple([f.status for f in self.modules.values()
                      if isinstance(f, FlipFlop)])

    @property
    def conjunction_modules(self):
        return [m for m in self.modules.values() if isinstance(m, Conjunction)]

    @property
    def all_targets(self):
        return [name for module in self.modules.values()
                for name in module.targets]

    @property
    def conjunctions_state(self):
        return tuple([tuple(c.inputs) for c in self.modules.values()
                      if isinstance(c, Conjunction)])

    def connect_modules(self):
        for name in self.all_targets:
            if name not in self.modules:
                self.modules[name] = TestOutput(name)
        for module in self.modules.values():
            for name in module.targets:
                module.targets[name] = self.modules[name]
        for module in self.conjunction_modules:
            num_inputs = self.all_targets.count(module.name)
            module.inputs = [FlipFlopStatus.OFF]*num_inputs

    def push_button(self):
        self.button_counter += 1
        low_count, high_count = self.broadcaster.pulse()
        self.low_counter += low_count
        self.high_counter += high_count

    def push_button_n_times(self, n: int = 1000):
        for i in range(n):
            self.push_button()
        return self.low_counter * self.high_counter




def parse_row_as_conjunction(line: str):
    name, targets_s = line.replace('&', '').split(' -> ')
    target_names = [s.strip(' ') for s in targets_s.split(',')]
    return Conjunction(name, [], {t: None for t in target_names})


def parse_row_as_flipflop(line: str):
    name, targets_s = line.replace('%', '').split(' -> ')
    target_names = [s.strip(' ') for s in targets_s.split(',')]
    return FlipFlop(name, {t: None for t in target_names})


def parse_row_as_broadcaster(line: str):
    targets_s = line.replace('broadcaster -> ', '')
    target_names = [s.strip(' ') for s in targets_s.split(',')]
    return Broadcaster({t: None for t in target_names})


def parse_input():
    with open('../input.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]
    modules = {}
    for row in data:
        if row.startswith('&'):
            new = parse_row_as_conjunction(row)
        elif row.startswith('%'):
            new = parse_row_as_flipflop(row)
        elif row.startswith('broadcaster'):
            new = parse_row_as_broadcaster(row)
        else:
            raise ValueError
        modules[new.name] = new
    return System(modules)


def part2():
    system = parse_input()
    system.connect_modules()
    print(f'Part 2 Answer: {system.find_button_pushes_for_rx_to_receive_low()}')


def main():
    part2()


if __name__ == '__main__':
    main()
