mod aoc_parser;
use std::{time::Instant, collections::{BTreeSet, HashMap, HashSet}};
use aoc_parser::{get_input_as_lines, Coord3D};


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Tower {
    bricks: BTreeSet<Brick>,
}

impl Tower {
    fn let_bricks_fall(&mut self){
        let mut fallen_bricks = BTreeSet::new(); //self.get_bricks_at_base().clone();
        for brick in self.bricks.clone(){
            let mut mut_brick = brick.clone();
            if fallen_bricks.len() > 0{
                let top_brick: Brick = fallen_bricks.clone().pop_last().unwrap();
                let zdiff: i64 = mut_brick.ends.1.z - mut_brick.ends.0.z;
                mut_brick.ends.0.z = top_brick.ends.1.z + 5;
                mut_brick.ends.1.z = top_brick.ends.1.z + 5 + zdiff;
            }

            while self.brick_in_freefall(mut_brick.clone(), fallen_bricks.clone()){
                mut_brick.ends.0.z -= 1;
                mut_brick.ends.1.z -= 1;
            }

            fallen_bricks.insert(mut_brick.clone());
        }
        self.bricks = fallen_bricks;
    
    }

    fn brick_in_freefall(&mut self, mut brick: Brick, fallen_bricks: BTreeSet<Brick>) -> bool{
        for coord in brick.get_coords(){
            let c = Coord3D{z: coord.z - 1, x: coord.x, y: coord.y};
            if c.z < 1 {
                return false;
            }
            for brick2 in fallen_bricks.clone().iter().rev(){
                //dbg!(brick2);
                let brick2_coords = brick2.clone().get_coords();
                if brick2_coords.contains(&c){
                    return false;
                }
            }
        }
        true
    }

    fn find_bricks_could_be_disintegrated(&mut self) -> usize{
        let mut total: usize = 0;
        for brick in self.bricks.clone(){
            let supported_bricks = self.get_neighbours_to(brick);
            if supported_bricks.len() > 0{
                for supported_brick in supported_bricks{
                    let supporting_bricks = self.get_supporting_bricks(supported_brick);
                    //dbg!(supporting_bricks.clone());
                    if supporting_bricks.len() > 1{
                        total += 1;
                        break;
                    }
                }
            }
            else {
                total += 1;
            }
        }
        total
    }

    fn find_load_bearing_bricks(&mut self) -> BTreeSet<Brick>{
        let mut load_bearers: BTreeSet<Brick> = BTreeSet::new();
        let mut parents: HashMap<Brick, Brick> = HashMap::new();
        let mut explored: HashSet<Brick> = HashSet::new();
        let mut queue: BTreeSet<Brick> = self.get_bricks_at_base();
        // queue.insert(bricks.pop_first().unwrap());
        while !queue.is_empty() {
            let current_block: Brick = queue.pop_first().unwrap();
            let neighbours = self.get_neighbours_to(current_block.clone());
            for neighbour in neighbours {
                if !explored.contains(&neighbour) {
                    explored.insert(neighbour.clone());
                    parents.insert(neighbour.clone(), current_block.clone());
                    queue.insert(neighbour.clone());
                }
            }
            if queue.is_empty(){
                let mut block: Brick = current_block.clone();
                load_bearers.insert(block.clone());
                while parents.contains_key(&block) {
                    let new_block: Brick = parents.get(&block).unwrap().clone();
                    load_bearers.insert(new_block.clone());
                    block = new_block;
                }
            }
        }
        dbg!(load_bearers.clone());
        load_bearers
    }
    fn get_bricks_at_base(&self) -> BTreeSet<Brick>{
        let mut bricks = BTreeSet::new();
        for brick in self.bricks.clone(){
            if brick.ends.0.z == 1{
                bricks.insert(brick.clone());
            }
        }
        bricks
    }
    fn get_brick_containing_coord(self, coord: Coord3D) -> Brick{
        for mut brick in self.bricks{
            if brick.get_coords().contains(&coord){
                return brick;
            }
        }
        unreachable!();
    }
    fn is_there_a_brick_containing_coord(self, coord: Coord3D) -> bool{
        for mut brick in self.bricks{
            let brick_coords = brick.get_coords();
            if brick_coords.contains(&coord){
                return true;
            }
        }
        false
    }
    fn get_neighbours_to(&mut self, mut brick: Brick) -> BTreeSet<Brick>{
        let mut neighbours: BTreeSet<Brick> = BTreeSet::new();
        for coord in brick.get_coords(){
            let c = Coord3D{z: coord.z + 1, x: coord.x, y: coord.y};
            if !brick.get_coords().contains(&c) && self.clone().is_there_a_brick_containing_coord(c){
                neighbours.insert(self.clone().get_brick_containing_coord(c));
            }
        }
        neighbours
    } 
    fn get_supporting_bricks(&mut self, mut brick: Brick) -> BTreeSet<Brick>{
        let mut neighbours: BTreeSet<Brick> = BTreeSet::new();
        for coord in brick.get_coords(){
            let c = Coord3D{z: coord.z - 1, x: coord.x, y: coord.y};
            if !brick.get_coords().contains(&c) && self.clone().is_there_a_brick_containing_coord(c){
                neighbours.insert(self.clone().get_brick_containing_coord(c));
            }
        }
        neighbours
    } 
}


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Brick{
    ends: (Coord3D, Coord3D),
}

impl Brick{
    fn get_coords(&mut self) -> BTreeSet<Coord3D>{
        let mut coords: BTreeSet<Coord3D> = BTreeSet::new();
        let xdiff = self.ends.0.x.abs_diff(self.ends.1.x);
        let ydiff = self.ends.0.y.abs_diff(self.ends.1.y);
        let zdiff = self.ends.0.z.abs_diff(self.ends.1.z);
        for i in 0..=xdiff {
            for j in 0..=ydiff {
                for k in 0..=zdiff {
                    coords.insert(Coord3D { 
                        z: self.ends.0.z + k as i64, 
                        x: self.ends.0.x + i as i64, 
                        y: self.ends.0.y + j as i64, 
                    });
                }
            }
        }
        coords
    }
}


fn string_to_coord3d(string: &str) -> Coord3D {
    let bindings: Vec<&str> = string.split(',').collect();
    let x: i64 = bindings[0].parse::<i64>().unwrap();
    let y: i64 = bindings[1].parse::<i64>().unwrap();
    let z: i64 = bindings[2].parse::<i64>().unwrap();
    Coord3D{z, x, y}
}


fn parse_row_into_brick(line: &str) -> Brick{
    let ends_string: Vec<&str> = line.split('~').collect();
    let start: Coord3D = string_to_coord3d(ends_string[0]);
    let end: Coord3D = string_to_coord3d(ends_string[1]);
    Brick { ends: (start, end) }
}


fn parse_input() -> Tower{
    let data = get_input_as_lines(include_str!("../input.txt"));
    let mut bricks: BTreeSet<Brick> = BTreeSet::new();
    for row in data {
        bricks.insert(parse_row_into_brick(row));
    }
    Tower { bricks }
}

fn part1(){
    let mut tower: Tower = parse_input();
    tower.let_bricks_fall();
    dbg!("bricks fallen!");
    //dbg!(bricks.clone());
    // let difference: Vec<Brick> = tower.bricks.difference(&bricks).cloned().collect();
    println!("Part 1 Answer: {}", tower.find_bricks_could_be_disintegrated());
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
