from dataclasses import dataclass
from enum import Enum
from typing import Union


class Pulse(Enum):
    LOW = 1,
    HIGH = 2,
    NONE = 0,


class FlipFlopStatus(Enum):
    ON = 1,
    OFF = 0,


@dataclass
class TestOutput:
    name: str
    next_pulse: Pulse = Pulse.LOW

    def __repr__(self):
        return f'TestOutput({self.name})'

    @property
    def targets(self):
        return {}

    def receive_pulse(self, pulse_type: Pulse):
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
    next_pulse: Pulse = Pulse.LOW

    def __repr__(self):
        return f'FlipFlop({self.name})'

    def switch(self):
        match self.status:
            case FlipFlopStatus.OFF:
                self.status = FlipFlopStatus.ON
            case FlipFlopStatus.ON:
                self.status = FlipFlopStatus.OFF

    def receive_pulse(self, pulse_type: Pulse):
        low_counter = 0
        high_counter = 0
        match pulse_type:
            case Pulse.LOW:
                # low_counter += 1
                self.switch()
                match self.status:
                    case FlipFlopStatus.ON:
                        pulse_type = Pulse.HIGH
                    case FlipFlopStatus.OFF:
                        pulse_type = Pulse.LOW
                self.next_pulse = pulse_type

            case Pulse.HIGH:
                high_counter += 1
                self.next_pulse = Pulse.NONE
        return low_counter, high_counter


@dataclass
class Conjunction:
    name: str
    inputs: dict[str, Pulse]
    targets: dict[str, Union[FlipFlop, 'Conjunction', TestOutput]]
    next_pulse: Pulse = Pulse.LOW

    def __repr__(self):
        return f'Conjunction({self.name})'

    def receive_pulse(self, pulse_type: Pulse, giver: str):
        low_counter = 0
        high_counter = 0
        self.inputs[giver] = pulse_type
        match pulse_type:
            case Pulse.HIGH:
                high_counter += 1
            case Pulse.LOW:
                pass
                # low_counter += 1
        if all([p == Pulse.HIGH for p in self.inputs.values()]):
            next_pulse = Pulse.LOW
        else:
            next_pulse = Pulse.HIGH
        self.next_pulse = next_pulse
        return low_counter, high_counter


@dataclass
class Broadcaster:
    targets: dict[str, Union[FlipFlop, Conjunction, TestOutput]]

    def __repr__(self):
        return f'Broadcaster()'

    @property
    def name(self):
        return 'broadcaster'

    def receive_pulse(self):
        low_counter = 0 #1
        high_counter = 0
        return low_counter, high_counter


@dataclass
class System:
    modules: dict[str, Union[FlipFlop, Broadcaster, Conjunction, TestOutput]]
    low_counter: int = 0
    high_counter: int = 0
    button_counter: int = 0

    def push_button(self):
        self.button_counter += 1
        low_counter = 0
        high_counter = 0
        self.broadcaster.receive_pulse()
        targets = [(mod, Pulse.LOW, 'broadcaster')
                   for mod in self.broadcaster.targets.values()]
        while targets:
            target = targets.pop(0)
            module, pulse_type, giver = target
            if isinstance(module, Conjunction):
                low_count, high_count = module.receive_pulse(pulse_type, giver)
            else:
                low_count, high_count = module.receive_pulse(pulse_type)
            low_counter += low_count
            high_counter += high_count
            targets.extend([(mod, module.next_pulse, module.name)
                            for mod in module.targets.values()
                            if module.next_pulse != Pulse.NONE])
        self.low_counter += low_counter
        self.high_counter += high_counter

    @property
    def broadcaster(self) -> Broadcaster:
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
            for name, value in self.modules.items():
                if module.name in value.targets:
                    module.inputs[value.name] = Pulse.LOW

    def push_button_n_times(self, n: int = 1000):
        for i in range(n):
            self.push_button()
        return self.low_counter * self.high_counter

    def find_button_pushes_for_rx_to_receive_low(self):
        states = {}
        while self.low_counter == 0:
            # new_state = (self.conjunctions_state, self.flipflops_state)
            # if new_state in states:
            #     print(f'Cycle completed at {self.button_counter} presses')
            #     print(f'Previous iteration occurred at {states[new_state]}')
            #     break
            # else:
            #     states[new_state] = self.button_counter

            self.push_button()
        return self.button_counter


def parse_row_as_conjunction(line: str):
    name, targets_s = line.replace('&', '').split(' -> ')
    target_names = [s.strip(' ') for s in targets_s.split(',')]
    return Conjunction(name, {}, {t: None for t in target_names})


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


def part1():
    system = parse_input()
    system.connect_modules()
    print(f'Part 1 Answer: {system.find_button_pushes_for_rx_to_receive_low()}')


def main():
    part1()


if __name__ == '__main__':
    main()
