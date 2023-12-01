
use permutation::Permutation;

fn parse_input() -> Vec<Vec<char>>{
    let input: &str = include_str!("../input.txt");
    let mut char_vec: Vec<char>;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_vec = line.chars().collect();
        data.push(char_vec)
    }
    return data;
}


fn part1(){
    let data: Vec<Vec<char>> = parse_input();
    let mut total: u32 = 0;
    let mut string_num: String;
    let mut num_row: Vec<char>;
    for row in data {
        num_row = Vec::new();
        for character in row.iter() {
            if character.is_ascii_digit() {
                num_row.push(*character)
            }
        }
        string_num = String::new();
        string_num.push(num_row[0]);
        string_num.push(num_row[num_row.len() - 1]);
        total += string_num.parse::<u32>().unwrap();
    }
    println!("{}", total);
}

fn map_word(word: &str) -> &str{
    match word {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        &_ => todo!(),
    }
}

fn part2(){
    let data: Vec<Vec<char>> = parse_input();
    let mut total: u32 = 0;
    let mut string_num: String;
    let mut num_row: Vec<&str>;
    let mut indices: Vec<usize>;
    let mut rnum_row: Vec<&str>;
    let mut rindices: Vec<usize>;
    let mut permutation: Permutation;
    let mut rpermutation: Permutation;
    let mut string_row: String;
    let mut numbers_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for row in data {
        indices = Vec::new();
        num_row = Vec::new();
        rindices = Vec::new();
        rnum_row = Vec::new();
        string_row = row.iter().collect();
        for substring in numbers_strings.iter_mut() {
            if string_row.contains(*substring) {
                num_row.push(map_word(substring));
                indices.push(string_row.find(*substring).unwrap());
                rnum_row.push(map_word(substring));
                rindices.push(string_row.rfind(*substring).unwrap());
            }
        }
        permutation = permutation::sort(indices.clone());
        rpermutation = permutation::sort(rindices.clone());
        num_row = permutation.apply_slice(&num_row);
        rnum_row = rpermutation.apply_slice(&rnum_row);
        string_num = String::new();
        string_num.push_str(num_row[0]);
        string_num.push_str(rnum_row[rnum_row.len() - 1]);
        total += string_num.parse::<u32>().unwrap();
    }
    println!("{}", total);
}


fn part2_improved(){
    let data: Vec<Vec<char>> = parse_input();
    let mut total: u32 = 0;
    let mut string_num: String;
    let mut num_row: Vec<&str>;
    let mut indices: Vec<usize>;
    let mut permutation: Permutation;
    let mut string_row: String;
    let mut numbers_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for row in data {
        indices = Vec::new();
        num_row = Vec::new();
        string_row = row.iter().collect();
        for substring in numbers_strings.iter_mut() {
            let matching_indices: Vec<_> = string_row.match_indices(*substring).collect();
            for pair in matching_indices {
                num_row.push(map_word(substring));
                indices.push(pair.0);
            }
            
        }
        permutation = permutation::sort(indices.clone());
        num_row = permutation.apply_slice(&num_row);
        string_num = String::new();
        string_num.push_str(num_row[0]);
        string_num.push_str(num_row[num_row.len() - 1]);
        total += string_num.parse::<u32>().unwrap();
    }
    println!("{}", total);
}


fn main() {
    part1();
    part2();
    part2_improved();
}
