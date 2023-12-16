mod aoc_parser; 
use std::{collections::{HashMap, BTreeSet, HashSet}, cmp::max};

use aoc_parser::get_input_as_chars;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Splitter{
    Vertical,
    Horizontal,
    ForwardDiagonal,
    BackwardDiagonal,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    i: i32,
    j: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Tile {
    coord: Coord,
    splitter: Splitter,
    energised: usize,
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Beam{
    direction: Direction,
    coord: Coord,
    path:  Vec<Coord>,
    history: BTreeSet<Vec<Coord>>,
}

impl Beam{
    fn advance(&mut self) {
        match self.direction {
            Direction::North => self.coord.i -= 1,
            Direction::East => self.coord.j += 1,
            Direction::South => self.coord.i += 1,
            Direction::West => self.coord.j -= 1,
        }
        self.path.push(self.coord);
    }
    fn advance_east(&mut self) {
        self.direction = Direction::East;
        self.advance();
    }
    fn advance_west(&mut self) {
        self.direction = Direction::West;
        self.advance();
    }
    fn advance_north(&mut self) {
        self.direction = Direction::North;
        self.advance();
    }
    fn advance_south(&mut self) {
        self.direction = Direction::South;
        self.advance();
    }
    fn split_north_south(&mut self) -> (Beam, Beam) {
        self.history.insert(self.path.clone());
        let mut beam1: Beam = Beam{direction: Direction::North, coord: self.coord, path: Vec::from([self.coord]), history: self.history.clone()};
        let mut beam2: Beam = Beam{direction: Direction::South, coord: self.coord, path: Vec::from([self.coord]), history: self.history.clone()};
        beam1.advance();
        beam2.advance();
        (beam1, beam2)
    }
    fn split_east_west(&mut self) -> (Beam, Beam) {
        self.history.insert(self.path.clone());
        let mut beam1: Beam = Beam{direction: Direction::East, coord: self.coord, path: Vec::from([self.coord]), history: self.history.clone()};
        let mut beam2: Beam = Beam{direction: Direction::West, coord: self.coord, path: Vec::from([self.coord]), history: self.history.clone()};
        beam1.advance();
        beam2.advance();
        (beam1, beam2)
    }
}


fn parse_input() -> HashMap<Coord, Tile>{
    let mut tiles: HashMap<Coord, Tile> = HashMap::new();
    let data: Vec<Vec<char>> = get_input_as_chars(include_str!("../input.txt"));
    for (i, row) in data.iter().enumerate(){
        for (j, c) in row.iter().enumerate() {
            let coord: Coord = Coord{i: i as i32, j: j as i32};
            let splitter: Splitter = match c {
                '-' => Splitter::Horizontal,
                '|' => Splitter::Vertical,
                '/' => Splitter::ForwardDiagonal,
                '\\' => Splitter::BackwardDiagonal,
                '.' => Splitter::None,
                _ => unreachable!(),
            };
            let new_tile = Tile{ coord, splitter, energised: 0};
            tiles.insert(coord, new_tile);
        }
     }
     tiles
}

fn count_energised_tiles(tiles: &HashMap<Coord, Tile>) -> usize {
    let mut total_energised: usize = 0;
    for v in tiles.values(){
        if v.energised > 0{
            total_energised += 1;
        }
    }
    total_energised
}

fn beam_passes_tiles<'a>(tiles: &'a mut HashMap<Coord, Tile>, mut beam: Beam, cache: &'a mut HashSet<(Coord, Direction)>) -> &'a HashMap<Coord, Tile>{
    while tiles.contains_key(&beam.coord) && !cache.contains(&(beam.coord, beam.direction)){
        cache.insert((beam.coord, beam.direction));
        let current_tile: &mut Tile = tiles.get_mut(&beam.coord).unwrap();
        current_tile.energised += 1;
        let splitter_situation: (Direction, Splitter) = (beam.direction, current_tile.splitter);
        match splitter_situation {
            (Direction::North, Splitter::Vertical) 
            | (Direction::South, Splitter::Vertical) 
            | (Direction::East, Splitter::Horizontal) 
            | (Direction::West, Splitter::Horizontal)
            | (Direction::North, Splitter::None) 
            | (Direction::South, Splitter::None) 
            | (Direction::East, Splitter::None) 
            | (Direction::West, Splitter::None) => beam.advance(),
            (Direction::North, Splitter::ForwardDiagonal) 
            | (Direction::South, Splitter::BackwardDiagonal) => beam.advance_east(),
            (Direction::North, Splitter::BackwardDiagonal) 
            | (Direction::South, Splitter::ForwardDiagonal) => beam.advance_west(),
            (Direction::East, Splitter::ForwardDiagonal) 
            | (Direction::West, Splitter::BackwardDiagonal) => beam.advance_north(),
            (Direction::East, Splitter::BackwardDiagonal) 
            | (Direction::West, Splitter::ForwardDiagonal) => beam.advance_south(),
            (Direction::East, Splitter::Vertical) 
            | (Direction::West, Splitter::Vertical) => {
                let beams: (Beam, Beam) = beam.split_north_south();
                beam = beams.0;
                beam_passes_tiles(tiles, beams.1, cache);
            },
            (Direction::North, Splitter::Horizontal) 
            | (Direction::South, Splitter::Horizontal) => {
                let beams = beam.split_east_west();
                beam = beams.0;
                beam_passes_tiles(tiles, beams.1, cache);
            },
        }
    }
    tiles
}
    



fn find_energised_tiles_from_start(start: Coord, direction: Direction) -> usize{
    let mut tiles: HashMap<Coord, Tile> = parse_input();
    let mut binding = HashSet::new();
    let new_tiles = beam_passes_tiles(&mut tiles, Beam{
        direction, 
        coord: start, 
        path: Vec::from([start]), 
        history: BTreeSet::new()
    }, &mut binding);
    count_energised_tiles(new_tiles)
}

fn part1(){
    let answer = find_energised_tiles_from_start(Coord{i: 0, j: 0}, Direction::East);
    println!("Part 1 Answer: {}", answer);
}

fn part2() {
    let mut max_number = 0;
    let data = get_input_as_chars(include_str!("../input.txt"));
    let n = data.len();
    let m = data[0].len();
    for i in 0..n{
        max_number = max(find_energised_tiles_from_start(Coord { i: i as i32, j: 0 }, Direction::West), max_number);
        max_number = max(find_energised_tiles_from_start(Coord { i: i as i32, j: m as i32 - 1 }, Direction::East), max_number);
    }
    for j in 0..m{
        max_number = max(find_energised_tiles_from_start(Coord { i: 0, j: j as i32}, Direction::South), max_number);
        max_number = max(find_energised_tiles_from_start(Coord { i: n as i32 - 1, j: j as i32 }, Direction::North), max_number);
    }
    
    println!("Part 2 Answer: {}", max_number);
}


fn main() {
    part1();
    part2();
}
