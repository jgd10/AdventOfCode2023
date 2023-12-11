mod aoc_parser;
use std::collections::{HashSet, HashMap, BTreeSet};
use aoc_parser::get_input_as_chars;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Galaxy {
    number: usize,
    x: i64,
    y: i64,
}

impl Galaxy {
    fn distance_to(&self, other: Galaxy) -> i64{
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

fn expand_the_universe2(offset: i64) -> HashMap<usize, Galaxy>{
    let data: Vec<Vec<char>> = get_input_as_chars(include_str!("../input.txt"));
    let mut empty_columns: HashSet<usize> = HashSet::from_iter(0..data[0].len());
    let mut empty_rows: HashSet<usize> = HashSet::from_iter(0..data.len());
    let mut galaxy_number: usize = 1;
    let mut galaxies: HashMap<usize, Galaxy> = HashMap::new();

    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate(){
            if c == &'#' {
                if empty_columns.contains(&j){
                    empty_columns.remove(&j);
                }
                if empty_rows.contains(&i){
                    empty_rows.remove(&i);
                }
            }
        }
    }
    let mut ioffset: i64 = 0;
    for (i, row) in data.iter().enumerate(){
        let mut joffset: i64 = 0;
        if empty_rows.contains(&i){
            ioffset += offset - 1;
        }
        for (j, c) in row.iter().enumerate(){
            if empty_columns.contains(&j){
                joffset += offset - 1;
            }
            if c == &'#' {
                galaxies.insert(galaxy_number, Galaxy { number: galaxy_number, x: j as i64 + joffset, y: i as i64 + ioffset });
                galaxy_number += 1;
            }
        }
    }
    galaxies
    
}

fn expand_the_universe() -> Vec<Vec<char>>{
    let data: Vec<Vec<char>> = get_input_as_chars(include_str!("../input.txt"));
    let mut row_expanded: Vec<Vec<char>> = Vec::new();
    let mut fully_expanded: Vec<Vec<char>> = Vec::new();
    let mut empty_columns: HashSet<usize> = HashSet::from_iter(0..data.len());

    for row in data {
        row_expanded.push(row.clone());
        if row.clone().iter().all(|&item| item == '.') {
            row_expanded.push(row.clone());
        }
        for (j, c) in row.iter().enumerate(){
            if c == &'#' && empty_columns.contains(&j) {
                empty_columns.remove(&j);
            }
        }
    }
    for row in row_expanded {
        let mut new_row: Vec<char> = Vec::new();
        for (j, c) in row.iter().enumerate(){
            new_row.push(*c);
            if empty_columns.contains(&j) {
                new_row.push(*c);
            }
        }
        fully_expanded.push(new_row);
    }
    fully_expanded
}

fn find_galaxies(universe: Vec<Vec<char>>) -> HashMap<usize, Galaxy>{
    let mut galaxy_number: usize = 1;
    let mut galaxies: HashMap<usize, Galaxy> = HashMap::new();
    for (i, row) in universe.iter().enumerate(){
        for (j, c) in row.iter().enumerate(){
            if c == &'#' {
                galaxies.insert(galaxy_number, Galaxy { number: galaxy_number, x: j as i64, y: i as i64 });
                galaxy_number += 1;
            }
        }
    }
    galaxies
}

fn distances_between_galaxies(galaxies: HashMap<usize, Galaxy>) -> Vec<i64>{
    let mut exhausted_pairs: BTreeSet<BTreeSet<usize>> = BTreeSet::new();
    let mut distances: Vec<i64> = Vec::new();
    for (k, galaxy) in galaxies.clone(){
        for (m, galaxy2) in galaxies.clone() {
            let pair: BTreeSet<usize> = BTreeSet::from([k, m]);
            if !exhausted_pairs.contains(&pair){
                let d = galaxy.distance_to(galaxy2);
                distances.push(d);
                exhausted_pairs.insert(pair);
            }
        }
    }
    distances
}

fn part1(){
    let data = expand_the_universe();
    let galaxies = find_galaxies(data);
    let distances = distances_between_galaxies(galaxies);
    println!("Part 1 Answer: {}", distances.iter().sum::<i64>());
}

fn part2(){
    let galaxies = expand_the_universe2(1000000);
    let distances = distances_between_galaxies(galaxies);
    println!("Part 2 Answer: {}", distances.iter().sum::<i64>());
}

fn main() {
    part1();
    part2();
}
