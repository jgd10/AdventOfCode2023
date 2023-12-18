
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum InputType {
    Input,
    Example,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub struct Coord32 {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub struct Coord64 {
    pub x: i64,
    pub y: i64,
}

#[allow(dead_code)]
pub fn get_input_as_lines(input: &'static str) -> Vec<&'static str>{
    let mut data: Vec<&str> = Vec::new();
    for line in input.lines() {
        data.push(line)
    }
    data
}

#[allow(dead_code)]
pub fn get_input_as_chars(input: &str) -> Vec<Vec<char>>{
    let mut char_vec: Vec<char>;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_vec = line.chars().collect();
        data.push(char_vec)
    }
    data
}