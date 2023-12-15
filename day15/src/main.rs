mod aoc_parser;
use std::collections::{VecDeque, HashMap};

use aoc_parser::get_input_as_lines;


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Lens{
    label: String,
    focal_length: u32,
}


#[derive(Clone, Debug, Hash)]
struct Box{
    number: u32,
    lenses: VecDeque<Lens>,
}

impl Box {
    fn insert_lens(&mut self, new_lens: &Lens){
        for i in 0..self.lenses.len() {
            if self.lenses[i].label == new_lens.label {
                self.lenses[i] = new_lens.clone();
                break;
            }
        }
        if !self.lenses.contains(new_lens){
            self.lenses.push_back(new_lens.clone());
        }
    }
    fn remove_lens(&mut self, label: &str){
        for i in 0..self.lenses.len() {
            if self.lenses[i].label == label {
                self.lenses.remove(i);
                break;
            }
        }
    }
    fn get_box_power(&self) -> u32 {
        let mut total: u32 = 0;
        for (i, lens) in self.lenses.iter().enumerate(){
            total += (self.number + 1)*(i as u32+1)*lens.focal_length;
        }
        total
    }
}

fn hash_char(data: char) -> u32 {
    return data as u32
}


fn hash_char_vector(data: Vec<char>) -> u32{
    let mut current: u32 = 0;
    for c in data{
        let value = hash_char(c);
        current += value;
        current *= 17;
        current = current % 256
    }
    current
}


fn parse_input(mut string: &'static str) -> Vec<Vec<char>>{
    if string.is_empty(){
        string = include_str!("../input.txt");
    }
    let line = get_input_as_lines(string).pop().unwrap();
    let entries: Vec<&str> = line.split(',').collect();
    let mut data: Vec<Vec<char>> = Vec::new();
    for entry in entries{
        data.push(entry.chars().collect());
    }
    data
}


#[test]
fn test_parse() {
    let s = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let data = parse_input(s);
    assert_eq!(data.len(), 11);
}

#[test]
fn test_hash() {
    let char_vector = Vec::from(['H', 'A', 'S', 'H']);
    assert_eq!(hash_char_vector(char_vector), 52);
}

#[test]
fn test_hash2() {
    let char_vector = Vec::from(['r', 'n']);
    assert_eq!(hash_char_vector(char_vector), 0);
}

#[test]
fn test_part1_example() {
    let s = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(s), 1320);
}


#[test]
fn test_part2_example() {
    let s = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part2(s), 145);
}


fn part1(string: &'static str) -> u32{
    let data = parse_input(&string);
    let mut total: u32 = 0;
    for cvector in data{
        total += hash_char_vector(cvector);
    }
    println!("Part 1 Answer: {}", total);
    total
}

fn get_label_details(characters: Vec<char>) -> (Vec<char>, String){
    let index: usize;
    if characters.contains(&'-'){
        index = 1;
    }
    else if characters.contains(&'='){
        index = 2;
    }
    else {
        unreachable!();
    }
    let label_chars: Vec<char> = characters[..(characters.len()-index)].to_vec();
    let mut label: String = String::new();
    for c in label_chars.clone(){
        label.push(c);
    }
    (label_chars, label)
}


fn part2(string: &'static str) -> u32{
    let mut boxes: HashMap<u32, Box> = HashMap::new();
    let data: Vec<Vec<char>> = parse_input(&string);
    let mut total: u32 = 0;
    for instruction in data{
        let label_details = get_label_details(instruction.clone());
        let box_num: u32 = hash_char_vector(label_details.0);
        if !boxes.contains_key(&box_num){
            boxes.insert(box_num, Box { number: box_num, lenses: VecDeque::new() });
        }
        if instruction.contains(&'='){
            let focal_length: u32 = instruction[instruction.len()-1].to_digit(10).unwrap();
            let lens: Lens = Lens{ label: label_details.1, focal_length: focal_length};
            boxes.get_mut(&box_num).unwrap().insert_lens(&lens);

        }
        else if instruction.contains(&'-'){
            boxes.get_mut(&box_num).unwrap().remove_lens(&label_details.1);
        }
    }
    for (_k, box_) in boxes{
        total += box_.get_box_power();
    }
    println!("Part 2 Answer: {}", total);
    total
}

fn main() {
    part1("");
    part2("");
}
