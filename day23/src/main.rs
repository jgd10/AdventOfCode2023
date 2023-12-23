mod aoc_parser;
use std::{time::Instant, collections::{HashMap, HashSet}, cmp::max};
use aoc_parser::{Coord32, get_input_as_chars};


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Land {
    Forest,
    FLat,
    East,
    South,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Tile {
    point: Coord32,
    land: Land,

}

impl Tile {
    fn get_neighbors1(&self) -> Vec<Coord32>{
        match self.land {
            Land::East => vec![Coord32{x: self.point.x+1, y: self.point.y}],
            Land::South => vec![Coord32{x: self.point.x, y: self.point.y+1}],
            Land::FLat => vec![
                Coord32{x: self.point.x+1, y: self.point.y},
                Coord32{x: self.point.x-1, y: self.point.y},
                Coord32{x: self.point.x, y: self.point.y+1},
                Coord32{x: self.point.x, y: self.point.y-1}],
            Land::Forest => Vec::new(),

        }
    }
    fn get_neighbors2(&self) -> Vec<Coord32>{
        vec![
            Coord32{x: self.point.x+1, y: self.point.y},
            Coord32{x: self.point.x-1, y: self.point.y},
            Coord32{x: self.point.x, y: self.point.y+1},
            Coord32{x: self.point.x, y: self.point.y-1}
            ]
    }
}

struct Trail {
    tiles: HashMap<Coord32, Tile>,
    ground: HashSet<Coord32>,
    distances: HashMap<Coord32, usize>

} 
impl Trail {
    fn get_limits(&self) -> (i32, i32){
        let mut xmax: i32 = 0;
        let mut ymax: i32 = 0;
        for tile in self.tiles.keys(){
            xmax = max(tile.x, xmax);
            ymax = max(tile.y, ymax);
        }
        (xmax, ymax)
    }
    fn find_longest_path_with_slopes(&mut self, start: Coord32, distance: usize, mut previous: HashSet<Coord32>) -> usize{
        let mut current_distance = distance;
        previous.insert(start);
        let mut current_tile = self.tiles.get(&start).unwrap();
        let mut neighbors: Vec<Coord32> = Vec::new();
        for n in current_tile.get_neighbors1(){
            if self.ground.contains(&n) && !previous.contains(&n) {
                neighbors.push(n);
            }
        }
        while neighbors.len() > 0 {
            if neighbors.len() == 1 {
                let neighbor = neighbors.pop().unwrap();
                current_distance += 1;
                previous.insert(neighbor);
                self.distances.insert(neighbor, *max(self.distances.clone().get_mut(&neighbor).unwrap(), &mut current_distance));
                current_tile = self.tiles.get(&neighbor).unwrap();
                for n in current_tile.get_neighbors1(){
                    if self.ground.contains(&n) && !previous.contains(&n) {
                        neighbors.push(n);
                    }
                }

            }
            else {
                let neighbor = neighbors.pop().unwrap();
                previous.insert(neighbor);
                let mut dist = self.find_longest_path_with_slopes(neighbor, current_distance+1, previous.clone());
                self.distances.insert(neighbor, *max( &mut current_distance, &mut dist));
            }
        }
        current_distance
    }
    fn find_longest_path_without_slopes(&mut self, start: Coord32, distance: usize, mut previous: HashSet<Coord32>) -> usize{
        let mut current_distance = distance;
        previous.insert(start);
        let mut current_tile = self.tiles.get(&start).unwrap();
        let mut neighbors: Vec<Coord32> = Vec::new();
        for n in current_tile.get_neighbors2(){
            if self.ground.contains(&n) && !previous.contains(&n) {
                neighbors.push(n);
            }
        }
        while neighbors.len() > 0 {
            if neighbors.len() == 1 {
                let neighbor = neighbors.pop().unwrap();
                current_distance += 1;
                previous.insert(neighbor);
                self.distances.insert(neighbor, *max(self.distances.clone().get_mut(&neighbor).unwrap(), &mut current_distance));
                current_tile = self.tiles.get(&neighbor).unwrap();
                for n in current_tile.get_neighbors2(){
                    if self.ground.contains(&n) && !previous.contains(&n) {
                        neighbors.push(n);
                    }
                }

            }
            else {
                let neighbor = neighbors.pop().unwrap();
                previous.insert(neighbor);
                let mut dist = self.find_longest_path_without_slopes(neighbor, current_distance+1, previous.clone());
                self.distances.insert(neighbor, *max( &mut current_distance, &mut dist));
            }
        }
        current_distance
    }
}

fn parse_input() -> Trail{
    let data = get_input_as_chars(include_str!("../input.txt"));
    let mut tiles: HashMap<Coord32, Tile> = HashMap::new();
    let mut ground: HashSet<Coord32> = HashSet::new();
    let mut distances: HashMap<Coord32, usize> = HashMap::new();
    for (j, row) in data.iter().enumerate(){
        for (i, c) in row.iter().enumerate() {
            let coord = Coord32{x: i as i32, y: j as i32};
            let tile_type = match c {
                '#' => Land::Forest,
                '.' => Land::FLat,
                '>' => Land::East,
                'v' => Land::South,
                _ => unreachable!(),
            };
            if tile_type != Land::Forest {
                ground.insert(coord);
            }
            tiles.insert(coord, Tile { point: coord, land: tile_type });
            distances.insert(coord, 0);

        }
    }
    Trail{tiles, ground, distances}

}

fn part1(){
    let mut trail: Trail = parse_input();
    let start = Coord32{x: 1 as i32, y: 0 as i32};
    let limits = trail.get_limits();
    trail.find_longest_path_with_slopes(start, 0, HashSet::new());
    println!("Part 1 Answer: {}", trail.distances.get(&Coord32{x: limits.0-1, y: limits.1}).unwrap());
}


fn part2(){
    let mut trail: Trail = parse_input();
    let start = Coord32{x: 1 as i32, y: 0 as i32};
    let limits = trail.get_limits();
    trail.find_longest_path_without_slopes(start, 0, HashSet::new());
    println!("Part 2 Answer: {}", trail.distances.get(&Coord32{x: limits.0-1, y: limits.1}).unwrap());
}


fn main() {
    let start = Instant::now();
    part1();
    println!("*** Part 1 Took {:.2?} ***", start.elapsed());
    let start2 = Instant::now();
    part2();
    println!("*** Part 2 Took {:.2?} ***", start2.elapsed());
}
