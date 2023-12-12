mod aoc_parser;
use aoc_parser::get_input_as_lines;
use itertools::Itertools;
use std::mem::replace;
use std::time::Instant;
use std::collections::{HashSet, BTreeSet, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Springs(Vec<i8>);

impl Springs {
    fn get_counts(&self) -> Vec<usize>{
        let mut counts: Vec<usize> = Vec::new();
        let mut counter: usize = 0;
        for s in self.0.clone() {
            if s == 1 {
                counter += 1;
            }
            else {
                if counter > 0 {
                    counts.push(counter);
                    counter = 0;
                }
            }
        }
        if counter > 0 {
            counts.push(counter);
        }
        // dbg!(self.clone(), counts.clone());
        counts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PuzzleLine{
    springs: Springs,
    counts: Vec<usize>,
}
impl PuzzleLine {
    fn get_num_remaining_springs(&self) -> usize{
        self.counts.clone().iter().sum::<usize>() - self.springs.0.iter().filter(|&n| *n == 1).count()
    }
    fn get_num_blanks(&self) -> usize {
        self.springs.0.iter().filter(|&n| *n == 9).count()
    }
    fn get_possible_spring_arrangements(&self) -> Vec<Springs>{
        let mut arrangements: Vec<Springs> = Vec::new();
        let mut items: Vec<i8> = Vec::new();
        let num_springs: usize = self.get_num_remaining_springs();
        let num_blanks: usize = self.get_num_blanks();
        for _i in 0..num_springs {
            items.push(1);
        }
        while items.len() != num_blanks {
            items.push(0);
        }
        //let timer2: Instant = Instant::now();
        //dbg!(items.len());
        //let perms = items.iter().permutations(items.len()).collect::<HashSet<_>>();
        //for p in perms{
        //}
        //println!("Finding unique permutations took {:2?}", timer2.elapsed()); 
        // for set in perms {
        //     let timer3: Instant = Instant::now();
        //     arrangements.push(self.fill_line(set.clone()));
        //     println!("Filling line {:?} took {:2?}", set, timer3.elapsed()); 
        // }
        let mut stack: VecDeque<Vec<i8>> = VecDeque::from([self.springs.0.clone()]);
        while stack.len() > 0 {
            let spring_set = stack.pop_back().unwrap();
            let mut blanks_present: bool = false;
            for (i, element) in spring_set.clone().iter().enumerate() {
                if element == &9{
                    let mut new0 = spring_set.clone();
                    let mut new1 = spring_set.clone();
                    _ = replace(&mut new0[i], 0);
                    _ = replace(&mut new1[i], 1);
                    stack.push_front(new0);
                    stack.push_front(new1);
                    blanks_present = true;
                }                
            }
            if !blanks_present {
                arrangements.push(Springs(spring_set));
            }
        }
        arrangements
    }
    fn fill_line(&self, set: Vec<&i8>) -> Springs {
        let mut new_spring = Springs(Vec::new());
        let mut counter: usize = 0;
        for i in self.springs.0.iter() {
            if i == &9 {
                new_spring.0.push(*set[counter]);
                counter += 1;
            }
            else {
                new_spring.0.push(*i);
            }
        }
        // dbg!(new_spring.clone());
        new_spring
    }
    fn are_springs_valid(&self, springs: Springs) -> bool{
        self.counts == springs.get_counts()
    }
    fn get_num_valid_arrangements(&self) -> usize{
        let mut counter: usize = 0;
        let timer1 = Instant::now();
        let arrangements = self.get_possible_spring_arrangements();
        println!("Finding arrangements took {:2?}", timer1.elapsed()); 
        for arrangement in arrangements {
            if self.are_springs_valid(arrangement){
                counter += 1;
            }
        }
        counter
    }
}

// fn get_all_permutations(num_zeros: usize, num_ones: usize) -> Vec<Vec<i8>> {
//     let mut possibilities: Vec<Vec<i8>> = Vec::new();
//     for i in 0..num_zeros {
//         let mut possibility: Vec<i8> = Vec::new();
//         for (j, possibility) in possibilities.iter().enumerate(){
//             possibility.push(j);
//             for (k, possibility2) in possibilities.iter().enumerate(){
//                 possibility2.push(k);
//             }
//         }
// 
//     }   
//     possibilities
// }

fn parse_input() -> Vec<PuzzleLine> {
    let data = get_input_as_lines(include_str!("../example2.txt"));
    let mut lines: Vec<PuzzleLine> = Vec::new();
    for row in data{
        let binding: Vec<&str> = row.split_ascii_whitespace().collect();
        let spring_str: Vec<char> = binding[0].chars().collect();
        let counts: Vec<usize> = binding[1].split(',').map(|component| component.parse::<usize>().unwrap()).collect();
        let mut springs: Vec<i8> = Vec::new();
        for c in spring_str{
            match c {
                '.' => springs.push(0),
                '#' => springs.push(1),
                '?' => springs.push(9),
                _ => (),
            }
        }
        let p = PuzzleLine{springs: Springs(springs), counts};
        lines.push(p);
    }
    lines
}

fn part1() {
    let lines = parse_input();
    let mut total: usize = 0;
    for line in lines{
        let num = line.get_num_valid_arrangements();
        dbg!(num);
        total += num;
    }
    println!("Part 1 Answer: {}", total);
}

fn part2() {
    println!("Part 2 Answer: {}", 0);
}

#[test]
fn test_springs_counts(){
    let springs = Springs(vec![0,1,1,1,0,0,1,1,0,0,1,0]);
    assert_eq!(springs.get_counts(), vec![3, 2, 1]);
}

#[test]
fn test_combinations(){
    let springs = BTreeSet::from([0,1,1,1,0,0,1,1,0,0,1,0]);
    let sets = springs.0.iter().combinations_w(springs.0.len());
    dbg!(sets);

}

fn main() {
    part1();
    part2();
}
