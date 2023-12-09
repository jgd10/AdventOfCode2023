mod aoc_parser;
use aoc_parser::get_input_as_lines;

fn arithmetic_sequence_next_step(sequence: &Vec<i64>) -> i64  {
    let mut reduced_sequence: Vec<i64> = Vec::new();
    let n: usize = sequence.len();
    for i in 0..(n-1) {
        reduced_sequence.push(sequence[i+1] - sequence[i]);
    }
    if reduced_sequence.iter().all(|x| *x == 0) {
        reduced_sequence.last().unwrap() + sequence.last().unwrap()
    }
    else {
        arithmetic_sequence_next_step(&reduced_sequence) + sequence.last().unwrap()
    }
}

fn arithmetic_sequence_previous_step(sequence: &Vec<i64>) -> i64  {
    let mut reduced_sequence: Vec<i64> = Vec::new();
    let n: usize = sequence.len();
    for i in 0..(n-1) {
        reduced_sequence.push(sequence[i+1] - sequence[i]);
    }
    if reduced_sequence.iter().all(|x| *x == 0) {
        sequence[0] - reduced_sequence[0]
    }
    else {
        sequence[0] - arithmetic_sequence_previous_step(&reduced_sequence)
    }
}

fn part1(){
    let data = get_input_as_lines(include_str!("../input.txt"));
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    let mut total: i64 = 0;
    for row in data {
        let num_string: Vec<&str> = row.split_ascii_whitespace().collect();
        sequences.push(num_string.into_iter().map(|x: &str| x.parse::<i64>().unwrap()).collect::<Vec<i64>>())
    }
    for sequence in sequences{
        let result: i64 = arithmetic_sequence_next_step(&sequence);
        total += result;
    }
    println!("Part 1 Answer: {}", total);

}

fn part2(){
    let data = get_input_as_lines(include_str!("../input.txt"));
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    let mut total: i64 = 0;
    for row in data {
        let num_string: Vec<&str> = row.split_ascii_whitespace().collect();
        sequences.push(num_string.into_iter().map(|x: &str| x.parse::<i64>().unwrap()).collect::<Vec<i64>>())
    }
    for sequence in sequences{
        let result: i64 = arithmetic_sequence_previous_step(&sequence);
        total += result;
    }
    println!("Part 2 Answer: {}", total);
}

fn main() {
    part1();
    part2();
}
