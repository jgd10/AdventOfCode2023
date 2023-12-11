mod aoc_parser;
use std::collections::HashMap;
use aoc_parser::{get_input_lines, InputType};
use num::integer::lcm;

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Commands {
    current: usize,
    instructions: Vec<Direction>,
}

impl Commands {
    fn get_next(&mut self) -> Direction {
        let result: Direction;
        if self.current == self.instructions.len() {
            self.current = 0;
        }
        result = self.instructions[self.current].clone();
        self.current += 1;
        result
    }
    fn reset(&mut self) {
        self.current = 0;
    }
}

fn commands_from_str(line: &str) -> Commands {
    let mut instructions: Vec<Direction> = Vec::new();
    for c in line.chars().into_iter() {
        match c {
            'L' => instructions.push(Direction::Left),
            'R' => instructions.push(Direction::Right),
            _ => (),
        }
    }
    Commands { current: 0, instructions: instructions }
}


fn parse_input() -> (Commands, HashMap<String, (String, String)>){
    let data = get_input_lines(InputType::Input);
    let commands: Commands = commands_from_str(data[0]); 
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for row in data[2..].into_iter() {
        let s = row.replace(&['(', ')', ',', '='][..], "");
        let binding: Vec<&str> = s.trim().split_ascii_whitespace().collect();
        network.insert(binding[0].to_string(),  (binding[1].to_string(), binding[2].to_string() ));
    }
    (commands, network)
}

fn find_steps_to_end(start: &str, condition: &dyn Fn(String) -> bool, mut maps: (Commands, HashMap<String, (String, String)>)) -> u128{
    let mut location: &str = start;
    let mut counter: u128 = 0;
    while condition(location.to_string().clone()) {
        let direction: Direction = maps.0.get_next();
        counter += 1;
        match direction {
            Direction::Left => location = &maps.1[location].0,
            Direction::Right => location = &maps.1[location].1,
        }
    }
    maps.0.reset();
    counter
}

fn part1(){
    let maps: (Commands, HashMap<String, (String, String)>) = parse_input();
    println!("Part 1: {}", find_steps_to_end("AAA", &is_not_end_node1, maps.clone()))
}

fn get_start_nodes(map_: HashMap<String, (String, String)>) -> Vec<String>{
    let mut starts: Vec<String> = Vec::new();
    for key in map_.clone().keys() {
        if key.ends_with('A') {
            starts.push(key.to_string());
        }
    }
    starts
}

fn is_not_end_node1(location: String) -> bool {
    location != "ZZZ"
}

fn is_not_end_node2(location: String) -> bool {
    !location.ends_with('Z')
}

fn get_lcm_of_vector(vec: Vec<u128>) -> u128 {
    let mut lowest_multiple: u128 = lcm(vec[0], vec[1]);
    for number in vec[2..].into_iter() {
        lowest_multiple = lcm(lowest_multiple, *number);
    }
    lowest_multiple
}

fn part2(){
    let maps: (Commands, HashMap<String, (String, String)>) = parse_input();
    let locations: Vec<String> = get_start_nodes(maps.1.clone());
    let answer: u128;
    let mut steps_for_each_route: Vec<u128> = Vec::new();
    for location in locations {
        steps_for_each_route.push(find_steps_to_end(&location, &is_not_end_node2, maps.clone()))
    }
    answer = get_lcm_of_vector(steps_for_each_route);
    println!("Part 2: {}", answer)
}

fn main() {
    part1();
    part2();
}

