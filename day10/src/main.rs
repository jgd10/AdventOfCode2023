mod aoc_parser;
use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_parser::get_input_as_chars;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum NodeType {
    Start,
    Pipe,
    Ground,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    coord: (usize, usize),
    connections: BTreeSet<(usize, usize)>,
    type_: NodeType,
    char_: char,
    on_path: bool,
}

fn get_node_from_char(c: &char, coord: (usize, usize)) -> Node{
    match c {
        '.' => Node { coord: coord, connections: BTreeSet::new(), type_: NodeType::Ground, char_: '.', on_path: false },
        'S' => Node { coord: coord, connections: BTreeSet::new(), type_: NodeType::Start, char_: 'S', on_path: false },
        '|' => Node { coord: coord, connections: BTreeSet::from([(coord.0+1, coord.1), (coord.0-1, coord.1)]), type_: NodeType::Pipe, char_: '|', on_path: false },
        '-' => Node { coord: coord, connections: BTreeSet::from([(coord.0, coord.1+1), (coord.0, coord.1-1)]), type_: NodeType::Pipe, char_: '-', on_path: false },
        'L' => Node { coord: coord, connections: BTreeSet::from([(coord.0-1, coord.1), (coord.0, coord.1+1)]), type_: NodeType::Pipe, char_: 'L', on_path: false },
        'J' => Node { coord: coord, connections: BTreeSet::from([(coord.0-1, coord.1), (coord.0, coord.1-1)]), type_: NodeType::Pipe, char_: 'J', on_path: false },
        '7' => Node { coord: coord, connections: BTreeSet::from([(coord.0+1, coord.1), (coord.0, coord.1-1)]), type_: NodeType::Pipe, char_: '7', on_path: false },
        'F' => Node { coord: coord, connections: BTreeSet::from([(coord.0+1, coord.1), (coord.0, coord.1+1)]), type_: NodeType::Pipe, char_: 'F', on_path: false },
        _ => unreachable!(),
    }
}

fn parse_input() -> (HashMap<(usize, usize), Node>, Node) {
    let data: Vec<Vec<char>> = get_input_as_chars(include_str!("../input.txt"));
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
    let mut start: Node = Node {coord: (99999, 99999), connections: BTreeSet::new(), type_: NodeType::Start, char_: 'S', on_path: true};
    for (i, row) in data.iter().enumerate(){
        for (j, c) in row.iter().enumerate() {
            let n = get_node_from_char(c, (i+1, j+1));
            if n.type_ == NodeType::Start{
                start = n.clone();
            }
            nodes.insert((i+1, j+1), n);
        }
    }
    (nodes, start)
}

fn find_connecting_nodes(node: Node, nodes: HashMap<(usize, usize), Node>) -> Vec<(usize, usize)>{
    let mut connecting_nodes: Vec<(usize, usize)> = Vec::new();
    for (_k, n) in nodes {
        if n.connections.contains(&node.coord) {
            connecting_nodes.push(n.coord.clone());
        }
    }
    connecting_nodes
}

fn find_loop(start: Node, nodes: HashMap<(usize, usize), Node>) -> (usize, BTreeSet<(usize, usize)>) {
    let mut current_nodes: Vec<(usize, usize)> = find_connecting_nodes(start.clone(), nodes.clone());
    let mut counter: usize = 1;
    let mut visited: BTreeSet<(usize, usize)> = BTreeSet::from([start.coord]);
    while current_nodes[0] != current_nodes[1] {
        let mut next_steps: Vec<(usize, usize)> = Vec::new();
        for coord in current_nodes {
            visited.insert(coord);
            let n: Node = nodes.get(&coord).unwrap().clone();
            for connection in n.connections{
                if !visited.contains(&connection) {
                    next_steps.push(connection.clone());
                }
            }
        }
        counter += 1;
        current_nodes = next_steps;
    }
    visited.insert(current_nodes[0]);
    (counter, visited)
}


fn part1(){
    let nodes: HashMap<(usize, usize), Node>;
    let start: Node;
    (nodes, start) = parse_input();
    println!("Part 1 Answer: {}", find_loop(start, nodes).0);
}

fn part2(){
    let nodes: HashMap<(usize, usize), Node>;
    let start: Node;
    let loop_nodes: BTreeSet<(usize, usize)>;
    let mut counter: usize = 0;
    let mut last_char: char = '.';
    (nodes, start) = parse_input();
    (_, loop_nodes) = find_loop(start, nodes.clone());
    let data: Vec<Vec<char>> = get_input_as_chars(include_str!("../input.txt"));
    for (i, row) in data.iter().enumerate(){
        let mut num_times_crossed_loop: i32 = 0;
        for (j, _c) in row.iter().enumerate() {
            if loop_nodes.contains(&(i+1, j+1)) {
                let n1: Node = nodes.get(&(i+1, j+1)).unwrap().clone();
                if n1.char_ == '|' {
                    num_times_crossed_loop += 1;
                    last_char = '|';
                }
                else {
                    if n1.char_ != '-' {
                        match (last_char, n1.char_) {
                            ('F', 'J') | ('L', '7') => num_times_crossed_loop += 1,
                            ('L', 'J') | ('F', '7') => num_times_crossed_loop += 2,
                            _ => last_char = n1.char_,
                        }
                    }

                }

            }
            else{
                if num_times_crossed_loop % 2 != 0 && num_times_crossed_loop != 0 {
                    //dbg!(i+1, j+1, _c, num_times_crossed_loop);
                    counter += 1;
                }
            }
        }
    }
    println!("Part 2 Answer: {}", counter);
}

fn main() {
    part1();
    part2();
}

