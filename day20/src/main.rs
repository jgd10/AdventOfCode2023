mod aoc_parser;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}


enum FlipFlop {
    On,
    Off,
}


struct FlipFlopModule {
    activity: FlipFlop,
    targets:
}

fn parse_input() {

}

fn part1(){
    println!("Part 1 Answer: {}", 0);
}


fn part2(){
    println!("Part 2 Answer: {}", 0);
}

fn main() {
    let start = Instant::now();
    part1();
    println!("*** Part 1 Took {:.2?} ***", start.elapsed());
    let start2 = Instant::now();
    part2();
    println!("*** Part 2 Took {:.2?} ***", start2.elapsed());
}
