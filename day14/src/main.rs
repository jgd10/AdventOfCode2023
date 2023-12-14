mod aoc_parser;
use std::collections::{HashMap, BTreeSet, BTreeMap};

use aoc_parser::get_input_as_chars;

#[derive(Clone, Debug, Copy)]
enum Direction{
    North,
    South,
    East,
    West,
}


#[derive(Clone, Debug, Eq, PartialEq)]
struct Platform {
    height: i32,
    width: i32,
    grid: HashMap<(i32, i32), char>,
    rocks: BTreeSet<(i32, i32)>,
    fixed: BTreeSet<(i32, i32)>,
}

impl Platform {
    fn tilt(&mut self, direction: Direction){
        match direction {
            Direction::North => self.tilt_north(), // toward i = 0
            Direction::East => self.tilt_east(),  // toward j = width
            Direction::South => self.tilt_south(),  // toward i = height
            Direction::West => self.tilt_west(),  // toward j = 0
        }
    }
    fn move_rock(&mut self, coord: (i32, i32), direction: Direction) {
        assert!(self.rocks.contains(&coord));
        let mut can_move: bool = true;
        let mut new_coord: (i32, i32) = coord;
        let mut trial_coord: (i32, i32);

        while can_move {
            match direction{
                Direction::North => trial_coord = (new_coord.0 - 1, new_coord.1),
                Direction::East => trial_coord = (new_coord.0, new_coord.1 + 1),
                Direction::South => trial_coord = (new_coord.0 + 1, new_coord.1),
                Direction::West => trial_coord = (new_coord.0, new_coord.1 - 1),
            }
            if self.grid.contains_key(&trial_coord) 
                && !self.fixed.contains(&trial_coord) 
                && !self.rocks.contains(&trial_coord) {
                    self.rocks.remove(&new_coord);
                    self.rocks.insert(trial_coord);
                    new_coord = trial_coord;

            }
            else {
                can_move = false;
            }
        }
    }
    fn tilt_north(&mut self){
        for i in 0..self.height {
            for j in 0..self.width {
                if self.rocks.contains(&(i, j)) {
                    self.move_rock((i, j), Direction::North)
                }
            }
        }
    }
    fn tilt_west(&mut self){
        for j in 0..self.width {
            for i in 0..self.height {
                if self.rocks.contains(&(i, j)) {
                    self.move_rock((i, j), Direction::West)
                }
            }
        }
    }
    fn tilt_east(&mut self){
        for j in 0..self.width {
            for i in 0..self.height {
                if self.rocks.contains(&(i, self.width-1-j)) {
                    self.move_rock((i, self.width-1-j), Direction::East)
                }
            }
        }
    }
    fn tilt_south(&mut self){
        for i in 0..self.height {
            for j in 0..self.width {
                if self.rocks.contains(&(self.height - 1 - i, j)) {
                    self.move_rock((self.height - 1 - i, j), Direction::South)
                }
            }
        }
    }
    fn get_load(&self, direction: Direction) -> i32 {
        let mut total: i32 = 0;
        let mut amount: i32;
        for coord in self.rocks.clone() {
            match direction{
                Direction::North => amount = (coord.0 - self.height).abs(),
                Direction::East => amount = coord.1 + 1,
                Direction::South => amount = coord.0 + 1,
                Direction::West => amount = (coord.1 - self.width).abs(),
            }
            total += amount
        }
        total
    }
    fn get_load_after(&mut self, direction: Direction, cycles: i32) -> i32{
        let mut cache: BTreeMap<BTreeSet<(i32, i32)>, i32> = BTreeMap::new();
        let mut n: i32 = 0;
        while n < cycles {
            if cache.contains_key(&self.rocks){
                let offset = cache.get(&self.rocks).unwrap();
                let state_length = n - offset;
                let remaining = (cycles - offset) % state_length;
                n = cycles - remaining;
            }
            else {
                cache.insert(self.rocks.clone(), n); 
            }
            self.tilt(Direction::North);
            self.tilt(Direction::West);
            self.tilt(Direction::South);
            self.tilt(Direction::East);
            n += 1;
        }
        self.get_load(direction)
    }
    #[allow(dead_code)]
    fn display(&self) -> String{
        let mut display_string = String::new();
        for i in 0..self.height{
            let mut row = String::new();
            for j in 0..self.width {
                if self.rocks.contains(&(i, j)) {
                    row.push('O');
                }
                else if self.fixed.contains(&(i,j)) {
                    row.push('#')
                }
                else if self.grid.contains_key(&(i, j)){
                    row.push('.');
                }
            }
            display_string.push_str(&row.to_string());
            display_string.push_str(&format!("--- {}", (i - self.height).abs()));
            display_string.push('\n');
        }
        display_string
    }
}


fn parse_input(mut string: &str) -> Platform{
    if string.is_empty(){
        string = include_str!("../input.txt");
    }
    let data = get_input_as_chars(string);
    let mut platform: HashMap<(i32, i32), char> = HashMap::new();
    let mut fixed_rocks: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut rocks: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (i, row) in data.clone().iter().enumerate(){
        for (j, c) in row.iter().enumerate(){
            platform.insert((i as i32, j as i32), *c);
            match c {
                '#' => _ = fixed_rocks.insert((i as i32, j as i32)),
                'O' => _ = rocks.insert((i as i32, j as i32)),
                _ => (),
            }
        }
    }
    Platform{height: data.len() as i32, width: data[0].len() as i32, grid: platform, rocks, fixed: fixed_rocks}
}

#[test]
pub fn test_one_cycle(){
    let expected_input: &str = r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
    let input:&str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...";
    let mut platform1 = parse_input(input);
    let platform2 = parse_input(expected_input);
    assert_eq!(platform1.get_load_after(Direction::North, 1), platform2.get_load(Direction::North));
}


#[test]
pub fn test_two_cycles(){
    let expected_input: &str = r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
    let input:&str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let mut platform1 = parse_input(input);
    let platform2 = parse_input(expected_input);
    assert_eq!(platform1.get_load_after(Direction::North, 2), platform2.get_load(Direction::North));
}

#[test]
pub fn test_three_cycles(){
    let expected_input: &str = r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
    let input:&str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let mut platform1 = parse_input(input);
    let platform2 = parse_input(expected_input);
    assert_eq!(platform1.get_load_after(Direction::North, 3), platform2.get_load(Direction::North));
}

fn part1(){
    let mut platform: Platform = parse_input("");
    platform.tilt(Direction::North);
    println!("Part 1 Answer: {}", platform.get_load(Direction::North));
}


fn part2(){
    let mut platform: Platform = parse_input("");
    println!("Part 2 Answer: {}", platform.get_load_after(Direction::North, 1_000_000_000));
}

fn main() {
    part1();
    part2();
}
