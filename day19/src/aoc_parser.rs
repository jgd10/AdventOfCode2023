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

// Uses a rule I found by accident: get the total area using the shoelace algorithm (the trapezoid formula in this case)
// Then add that to half the total perimeter and add 1.
// Taken from 2023 day 18 - only works for polygon with either horizontal or vertical edges (i.e. squares)
#[allow(dead_code)]
fn calculate_area_polygon32(coordinates: Vec<Coord32>) -> i32{
    let mut total: i32 = 0;
    for i in 0..(coordinates.len()-1){
        let amount = (coordinates[i].y + coordinates[i+1].y)*(coordinates[i].x - coordinates[i+1].x);
        total += amount;
    }
    total += (coordinates[coordinates.len()-1].y + coordinates[0].y)*(coordinates[coordinates.len()-1].x - coordinates[0].x);
    total.abs() / 2 + calculate_perimeter32(coordinates) / 2 + 1
}


// Taken from 2023 day 18 - only works for polygon with either horizontal or vertical edges
#[allow(dead_code)]
fn calculate_area_polygon64(coordinates: Vec<Coord64>) -> i64{
    let mut total: i64 = 0;
    for i in 0..(coordinates.len()-1){
        let amount = (coordinates[i].y + coordinates[i+1].y)*(coordinates[i].x - coordinates[i+1].x);
        total += amount;
    }
    total += (coordinates[coordinates.len()-1].y + coordinates[0].y)*(coordinates[coordinates.len()-1].x - coordinates[0].x);
    total.abs() / 2 + calculate_perimeter64(coordinates) / 2 + 1
}

// Taken from 2023 day 18 - only works for polygon with either horizontal or vertical edges
#[allow(dead_code)]
fn calculate_perimeter32(coordinates: Vec<Coord32>) -> i32 {
    let mut total: i32 = 0;
    for i in 0..(coordinates.len()-1){
        let amount = (coordinates[i].y - coordinates[i+1].y)+(coordinates[i].x - coordinates[i+1].x);
        total += amount;
    }
    total += (coordinates[coordinates.len()-1].y + coordinates[0].y)*(coordinates[coordinates.len()-1].x - coordinates[0].x);
    total
}

// Taken from 2023 day 18 - only works for polygon with either horizontal or vertical edges
#[allow(dead_code)]
fn calculate_perimeter64(coordinates: Vec<Coord64>) -> i64 {
    let mut total: i64 = 0;
    for i in 0..(coordinates.len()-1){
        let amount = (coordinates[i].y - coordinates[i+1].y)+(coordinates[i].x - coordinates[i+1].x);
        total += amount;
    }
    total += (coordinates[coordinates.len()-1].y + coordinates[0].y)*(coordinates[coordinates.len()-1].x - coordinates[0].x);
    total
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