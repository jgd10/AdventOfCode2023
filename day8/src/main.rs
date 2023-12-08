mod aoc_parser;
use std::collections::HashMap;
use aoc_parser::{get_input_lines, InputType};

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Node {
    name: String,
    left: String,
    right: String,

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

fn part1(){
    let mut maps: (Commands, HashMap<String, (String, String)>) = parse_input();
    let mut location: &str = "AAA";
    let mut counter: usize = 0;
    while location != "ZZZ" {
        let direction: Direction = maps.0.get_next();
        counter += 1;
        match direction {
            Direction::Left => location = &maps.1[location].0,
            Direction::Right => location = &maps.1[location].1,
        }
    }
    println!("Part 1: {}", counter)
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

fn all_end_nodes(locations: Vec<String>) -> bool {
    for location in locations {
        if !location.ends_with('Z') {
            return false;
        }
    }
    true
}

fn part2(){
    let mut maps: (Commands, HashMap<String, (String, String)>) = parse_input();
    let mut locations: Vec<String> = get_start_nodes(maps.1.clone());
    let mut counter: usize = 0;
    while !all_end_nodes(locations.clone()) == true {
        let direction: Direction = maps.0.get_next();
        counter += 1;
        let mut new_locs: Vec<String> = Vec::new();
        for location in locations.clone() {
            match direction {
                Direction::Left => new_locs.push(maps.1[&location].0.clone()),
                Direction::Right => new_locs.push(maps.1[&location].1.clone()),
            }
        }
        locations = new_locs;
    }
    println!("Part 2: {}", counter)
}

fn main() {
    part1();
    part2();
}

