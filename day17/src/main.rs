mod aoc_parser;
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::time::Instant;

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
    limits: (usize, usize),
}

impl Network {
    fn dijkstra0(&mut self) {
        let current_block: Block = self.get_start_block();
        // Start the queue with 2 nodes one going E one going S
        // They start with 1 step!
        let mut queue: BTreeSet<(usize, BigCoord)> = BTreeSet::from(
            [(0, BigCoord::from_coord(current_block.coord, Direction::E)), 
             (0, BigCoord::from_coord(current_block.coord, Direction::S))]);
        let mut big_coords_done_with: HashSet<BigCoord> = HashSet::from([
            BigCoord::from_coord(current_block.coord, Direction::E), 
            BigCoord::from_coord(current_block.coord, Direction::S)]);
        

        let mut visited: HashSet<BigCoord> = HashSet::new();

        while queue.len() > 0 {
            let current_node = queue.pop_first().unwrap();
            visited.insert(current_node.1);
            // current_block = *self.blocks.get_mut(&current_node.1).unwrap();

            let neighbors = get_neighbor_coords0(current_node.1, self.limits);
            for neighbor in neighbors {
                // if the coord hasn't been worked on (copy of queue) AND hasn't been visited yet, slam it in
                if !big_coords_done_with.contains(&neighbor) && !visited.contains(&neighbor){
                    // get the block object which contains weights and such
                    let block = self.blocks.get_mut(&neighbor).unwrap();
                    let alternative_distance = current_node.0 + block.loss;
                    // Do the dijkstra bit
                    if alternative_distance < block.weight {
                        block.weight = alternative_distance;
                    }
                    // add neighbor to queue to be considered next!
                    queue.insert((block.weight, neighbor));
                    big_coords_done_with.insert(neighbor);
                }
            }
        }

    }
    fn dijkstra2(&mut self) {
        let current_block: Block = self.get_start_block();
        // Start the queue with 2 nodes one going E one going S
        // They start with 1 step!
        let mut queue: BTreeSet<(usize, BigCoord)> = BTreeSet::from(
            [(0, BigCoord::from_coord(current_block.coord, Direction::E)), 
             (0, BigCoord::from_coord(current_block.coord, Direction::S))]);
        let mut big_coords_done_with: HashSet<BigCoord> = HashSet::from([
            BigCoord::from_coord(current_block.coord, Direction::E), 
            BigCoord::from_coord(current_block.coord, Direction::S)]);
        

        let mut visited: HashSet<BigCoord> = HashSet::new();

        while queue.len() > 0 {
            let current_node = queue.pop_first().unwrap();
            visited.insert(current_node.1);
            // current_block = *self.blocks.get_mut(&current_node.1).unwrap();

            let neighbors = get_neighbor_coords2(current_node.1, self.limits);
            for neighbor in neighbors {
                // if the coord hasn't been worked on (copy of queue) AND hasn't been visited yet, slam it in
                if !big_coords_done_with.contains(&neighbor) && !visited.contains(&neighbor){
                    // get the block object which contains weights and such
                    let block = self.blocks.get_mut(&neighbor).unwrap();
                    let alternative_distance = current_node.0 + block.loss;
                    // Do the dijkstra bit
                    if alternative_distance < block.weight {
                        block.weight = alternative_distance;
                    }
                    // add neighbor to queue to be considered next!
                    queue.insert((block.weight, neighbor));
                    big_coords_done_with.insert(neighbor);
                }
            }
        }

    }
    fn get_end_block_weights(&self) -> usize{
        let mut min_weight: usize = 999999999;
        for (_k, block) in self.blocks.clone(){
            if block.type_ == BlockType::End {
                min_weight = min(block.weight, min_weight);
            }
        }
        min_weight
    }
    fn get_end_block_weights2(&self) -> usize{
        let mut min_weight: usize = 999999999;
        for (k, block) in self.blocks.clone(){
            if block.type_ == BlockType::End && k.steps >= 4 {
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
        Self{coord, direction, steps: 0}
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Block {
    coord: Coord,
    loss: usize,
    weight: usize,
    type_: BlockType,
}


fn get_neighbor_coords2(start: BigCoord, limits: (usize, usize)) -> HashSet<BigCoord>{
    let mut coords = HashSet::new();
    if start.steps >= 4{
        if start.coord.i > 0 && start.direction != Direction::S {
            let new = Coord{i: start.coord.i - 1, j: start.coord.j};
            if start.direction == Direction::N && start.steps < 10 {
                coords.insert(BigCoord{coord: new, direction: Direction::N, steps: start.steps + 1});
            }
            else if start.direction != Direction::N {
                coords.insert(BigCoord{coord: new, direction: Direction::N, steps: 1});
            }  
        }
        if start.coord.j > 0 && start.direction != Direction::E {
            let new = Coord{i: start.coord.i, j: start.coord.j - 1};
            if start.direction == Direction::W && start.steps < 10 {
                coords.insert(BigCoord{coord: new, direction: Direction::W, steps: start.steps + 1});
            }
            else if start.direction != Direction::W {
                coords.insert(BigCoord{coord: new, direction: Direction::W, steps: 1});
            }  
        }
        if start.coord.i < limits.0 && start.direction != Direction::N {
            let new = Coord{i: start.coord.i + 1, j: start.coord.j};
            if start.direction == Direction::S && start.steps < 10 {
                coords.insert(BigCoord{coord: new, direction: Direction::S, steps: start.steps + 1});
            }
            else if start.direction != Direction::S {
                coords.insert(BigCoord{coord: new, direction: Direction::S, steps: 1});
            }
        }
        if start.coord.j < limits.1 && start.direction != Direction::W {
            let new = Coord{i: start.coord.i, j: start.coord.j + 1};  
            if start.direction == Direction::E && start.steps < 10 {
                coords.insert(BigCoord{coord: new, direction: Direction::E, steps: start.steps + 1});
            }
            else if start.direction != Direction::E {
                coords.insert(BigCoord{coord: new, direction: Direction::E, steps: 1});
            }  
        }
    }
    else {
        match start.direction{
            Direction::E => {
                if start.coord.j < limits.1{
                    coords.insert(BigCoord{coord: Coord{i: start.coord.i, j: start.coord.j + 1}, direction: Direction::E, steps: start.steps + 1});
                }
            },
            Direction::N => {
                if start.coord.i > 0 {
                    coords.insert(BigCoord{coord: Coord{i: start.coord.i - 1, j: start.coord.j}, direction: Direction::N, steps: start.steps + 1});
                }
            },
            Direction::S =>  {
                if start.coord.i < limits.0 {
                    coords.insert(BigCoord{coord: Coord{i: start.coord.i + 1, j: start.coord.j}, direction: Direction::S, steps: start.steps + 1});
                }
            },
            Direction::W =>  {
                if start.coord.j > 0 {
                    coords.insert(BigCoord{coord: Coord{i: start.coord.i, j: start.coord.j - 1}, direction: Direction::W, steps: start.steps + 1});
                }
            },
        };
    }
    coords
}

fn get_neighbor_coords0(start: BigCoord, limits: (usize, usize)) -> HashSet<BigCoord>{
    let mut coords = HashSet::new();
    if start.coord.i > 0 && start.direction != Direction::S {
        let new = Coord{i: start.coord.i - 1, j: start.coord.j};
        if start.direction == Direction::N && start.steps < 3 {
            coords.insert(BigCoord{coord: new, direction: Direction::N, steps: start.steps + 1});
        }
        else if start.direction != Direction::N {
            coords.insert(BigCoord{coord: new, direction: Direction::N, steps: 1});
        }  
    }
    if start.coord.j > 0 && start.direction != Direction::E {
        let new = Coord{i: start.coord.i, j: start.coord.j - 1};
        if start.direction == Direction::W && start.steps < 3 {
            coords.insert(BigCoord{coord: new, direction: Direction::W, steps: start.steps + 1});
        }
        else if start.direction != Direction::W {
            coords.insert(BigCoord{coord: new, direction: Direction::W, steps: 1});
        }  
    }
    if start.coord.i < limits.0 && start.direction != Direction::N {
        let new = Coord{i: start.coord.i + 1, j: start.coord.j};
        if start.direction == Direction::S && start.steps < 3 {
            coords.insert(BigCoord{coord: new, direction: Direction::S, steps: start.steps + 1});
        }
        else if start.direction != Direction::S {
            coords.insert(BigCoord{coord: new, direction: Direction::S, steps: 1});
        }
    }
    if start.coord.j < limits.1 && start.direction != Direction::W {
        let new = Coord{i: start.coord.i, j: start.coord.j + 1};  
        if start.direction == Direction::E && start.steps < 3 {
            coords.insert(BigCoord{coord: new, direction: Direction::E, steps: start.steps + 1});
        }
        else if start.direction != Direction::E {
            coords.insert(BigCoord{coord: new, direction: Direction::E, steps: 1});
        }  
    }
    coords
}

fn parse_input2() -> Network{
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
                for steps in 0..11{
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
    Network{blocks, visited: HashSet::new(), limits: (imax, jmax)}
}

fn part1(){
    let mut network = parse_input2();
    network.dijkstra0();
    println!("Part 1 Answer: {:?}", network.get_end_block_weights());
}


fn part2(){
    let mut network = parse_input2();
    network.dijkstra2();
    println!("Part 2 Answer: {:?}", network.get_end_block_weights2());
}

fn main() {
    let start = Instant::now();
    part1();
    println!("*** Part 1 Took {:.2?} ***", start.elapsed());
    let start2 = Instant::now();
    part2();
    println!("*** Part 2 Took {:.2?} ***", start2.elapsed());
}
