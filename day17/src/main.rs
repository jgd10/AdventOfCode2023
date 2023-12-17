mod aoc_parser;
use std::collections::{HashMap, HashSet};
use std::cmp::min;

use aoc_parser::get_input_as_chars;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum BlockType{
    Start,
    End,
    None,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}


#[derive(Clone, Debug, Eq, PartialEq)]
struct Network {
    blocks: HashMap<BigCoord, Block>,
    visited: HashSet<Coord>,
}

impl Network {
    fn dijkstra(&mut self) {
        let mut current_block: Block = self.get_start_block();
        // Start the queue with 2 nodes one going E one going S
        // They start with 1 step!
        let mut queue: Vec<BigCoord> = Vec::from(
            [BigCoord::from_coord(current_block.coord, Direction::E), 
             BigCoord::from_coord(current_block.coord, Direction::S)]);

        // get all the x,y coords possible
        let all_coords: HashSet<BigCoord> = self.blocks.keys().cloned().collect();
        // set of all the nodes we've been to (none so far)
        let mut visited: HashSet<BigCoord> = HashSet::new();

        while  queue.len() > 0 {
            // pop out the node in our queue with the smallest weight
            let current_node = get_shortest_distance_in_queue(&mut queue, self.blocks.clone());
            // add it to the visited list
            visited.insert(current_node);
            // get its associated block (contains the heat loss and weighting)
            current_block = *self.blocks.get_mut(&current_node).unwrap();

            // get the neighbors of that block, there should be no more than 3
            // e.g. if starting going east (with 2 steps east taken) then next blocks would be east, north, south, but not west,
            // because you can't go back on yourself (not immediately anyway)
            let neighbors = get_neighbor_coords(current_node);
            for neighbor in neighbors {
                // if the neighbor exists in space AND hasn't been visited AND isn't in the queue then we consider it
                if all_coords.contains(&neighbor) && !visited.contains(&neighbor) && !queue.contains(&neighbor){
                    // add neighbor to queue to be considered next!
                    queue.push(neighbor);
                    // get the block object which contains weights and such
                    let block = self.blocks.get_mut(&neighbor).unwrap();
                    let alternative_distance = current_block.weight + block.loss;
                    // Do the dijkstra bit
                    if alternative_distance < block.weight {
                        block.weight = alternative_distance;
                    }
                }
            }
        }

    }
    fn get_end_block_weights(&self) -> usize{
        let mut min_weight: usize = 999999999;
        for block in self.blocks.values(){
            if block.type_ == BlockType::End {
                min_weight = min(block.weight, min_weight);
            }
        }
        min_weight
    }
    fn get_start_block(&mut self) -> Block{
        for block in self.blocks.values(){
            if block.type_ == BlockType::Start {
                return block.clone();
            }
        }
        unreachable!();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Coord {
    i: usize,
    j: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct BigCoord {
    coord: Coord,
    direction: Direction,
    steps: usize,
}

impl BigCoord {
    fn from_coord(coord: Coord, direction: Direction) -> Self{
        Self{coord, direction, steps: 1}
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Block {
    coord: Coord,
    loss: usize,
    weight: usize,
    type_: BlockType,
}


fn get_shortest_distance_in_queue(queue: &mut Vec<BigCoord>, blocks: HashMap<BigCoord, Block>) -> BigCoord {
    queue.sort_by_key(|x| blocks.get(&x).unwrap().weight);
    queue.remove(0)
}

fn get_neighbor_coords(start: BigCoord) -> HashSet<BigCoord>{
    let mut coords = HashSet::new();
    if start.coord.i > 0 && start.direction != Direction::S{
        let new = Coord{i: start.coord.i - 1, j: start.coord.j};
        if start.direction == Direction::N && start.steps < 3{
            coords.insert(BigCoord{coord: new, direction: Direction::N, steps: start.steps + 1});
        }
        else if start.direction != Direction::N{
            coords.insert(BigCoord{coord: new, direction: Direction::N, steps: 0});
        }  
    }
    if start.coord.j > 0 && start.direction != Direction::E{
        let new = Coord{i: start.coord.i, j: start.coord.j - 1};
        if start.direction == Direction::W && start.steps < 3{
            coords.insert(BigCoord{coord: new, direction: Direction::W, steps: start.steps + 1});
        }
        else if start.direction != Direction::W{
            coords.insert(BigCoord{coord: new, direction: Direction::W, steps: 0});
        }  
    }
    if start.direction != Direction::N{
        let new = Coord{i: start.coord.i + 1, j: start.coord.j};
        if start.direction == Direction::S && start.steps < 3{
            coords.insert(BigCoord{coord: new, direction: Direction::S, steps: start.steps + 1});
        }
        else if start.direction != Direction::S{
            coords.insert(BigCoord{coord: new, direction: Direction::S, steps: 0});
        }
    }
    if start.direction != Direction::W {
        let new = Coord{i: start.coord.i, j: start.coord.j + 1};  
        if start.direction == Direction::E && start.steps < 3{
            coords.insert(BigCoord{coord: new, direction: Direction::E, steps: start.steps + 1});
        }
        else if start.direction != Direction::E {
            coords.insert(BigCoord{coord: new, direction: Direction::E, steps: 0});
        }  
    }
    coords
}


fn parse_input() -> Network{
    let data = get_input_as_chars(include_str!("../input.txt"));
    let imax = data.len()-1;
    let jmax = data[0].len() -1;
    let mut blocks: HashMap<BigCoord, Block> = HashMap::new();
    for (i, row) in data.iter().enumerate(){
        for (j, c) in row.iter().enumerate() {
            let coord = Coord{i, j};
            let block_type: BlockType;
            let mut weight: usize = 999999999;
            if (i, j) == (0, 0){
                block_type = BlockType::Start;
                weight = 0 ;
            }
            else if (i, j) == (imax, jmax) {
                block_type = BlockType::End;
            }
            else{
                block_type = BlockType::None;
            }
            for direction in [Direction::N, Direction::E, Direction::S, Direction::W]{
                for steps in 0..3{
                    blocks.insert(BigCoord{coord, direction, steps: steps as usize}, Block { 
                        coord: Coord{i, j}, 
                        loss: c.to_digit(10).unwrap() as usize ,
                        type_: block_type, 
                        weight: weight,
                    });
                }

            }

            
        }
    }
    Network{blocks, visited: HashSet::new()}
}

fn part1(){
    let mut network = parse_input();
    network.dijkstra();
    println!("Part 1 Answer: {:?}", network.get_end_block_weights());
}


fn part2(){
    println!("Part 2 Answer: {}", 0);
}

fn main() {
    part1();
    part2();
}
