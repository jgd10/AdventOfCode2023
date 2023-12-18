mod aoc_parser;
use aoc_parser::{get_input_as_lines, Direction, Coord64};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Vertex{
    coord: Coord64,
    direction: Direction,
    amount: i64,
}

fn get_vertex_from_row(row: &str, prev_coord: Coord64) -> (Vertex, Coord64){
    let parts: Vec<&str> = row.split_ascii_whitespace().collect();
    let amount: i64 = parts[1].parse::<i64>().unwrap();
    let direction: Direction = match parts[0] {
        "R" => Direction::East,
        "U" => Direction::North,
        "L" => Direction::West,
        "D" => Direction::South,
        _ => unreachable!(),
    };
    let new_coord: Coord64 = match direction {
        Direction::East => Coord64{x: prev_coord.x + amount, y: prev_coord.y},
        Direction::North => Coord64{x: prev_coord.x, y: prev_coord.y + amount},
        Direction::West => Coord64{x: prev_coord.x - amount, y: prev_coord.y},
        Direction::South => Coord64{x: prev_coord.x, y: prev_coord.y - amount},
    };
    (Vertex { coord: prev_coord, direction, amount }, new_coord)
}

fn get_vertex_from_row2(row: &str, prev_coord: Coord64) -> (Vertex, Coord64){
    let parts: Vec<&str> = row.split_ascii_whitespace().collect();
    let mut instruction: String = parts[2].replace(&['(', ')', '#'][..], "");
    let direction_char = instruction.pop().unwrap();
    let amount = i64::from_str_radix(&instruction, 16).unwrap();
    let direction: Direction = match direction_char {
        '0' => Direction::East,
        '3' => Direction::North,
        '2' => Direction::West,
        '1' => Direction::South,
        _ => unreachable!(),
    };
    let new_coord: Coord64 = match direction {
        Direction::East => Coord64{x: prev_coord.x + amount, y: prev_coord.y},
        Direction::North => Coord64{x: prev_coord.x, y: prev_coord.y + amount},
        Direction::West => Coord64{x: prev_coord.x - amount, y: prev_coord.y},
        Direction::South => Coord64{x: prev_coord.x, y: prev_coord.y - amount},
    };
    (Vertex { coord: prev_coord, direction, amount }, new_coord)
}


fn parse_input() -> Vec<Vertex> {
    let data = get_input_as_lines(include_str!("../input.txt"));
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut coord = Coord64{x: 0, y: 0};
    for row in data {
        let vertexs = get_vertex_from_row(row, coord);
        coord = vertexs.1;
        vertices.push(vertexs.0);
    }
    vertices
}

fn parse_input2() -> Vec<Vertex> {
    let data = get_input_as_lines(include_str!("../input.txt"));
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut coord = Coord64{x: 0, y: 0};
    for row in data {
        let vertexs = get_vertex_from_row2(row, coord);
        coord = vertexs.1;
        vertices.push(vertexs.0);
    }
    vertices
}

fn calculate_area_polygon(vertices: Vec<Vertex>) -> i64{
    let mut total: i64 = 0;
    for i in 0..(vertices.len()-1){
        let amount = (vertices[i].coord.y + vertices[i+1].coord.y)*(vertices[i].coord.x - vertices[i+1].coord.x);
        total += amount;
    }
    total += (vertices[vertices.len()-1].coord.y + vertices[0].coord.y)*(vertices[vertices.len()-1].coord.x - vertices[0].coord.x);
    total.abs() / 2 + calculate_perimeter(vertices) / 2 + 1
}

fn calculate_perimeter(vertices: Vec<Vertex>) -> i64 {
    let mut total: i64 = 0;
    for v in vertices{
        total += v.amount;
    }
    total
}

fn part1(){
    let vertices = parse_input();
    println!("Part 1 Answer: {}", calculate_area_polygon(vertices.clone()));
}


fn part2(){
    let vertices = parse_input2();
    println!("Part 2 Answer: {}", calculate_area_polygon(vertices.clone()));
}

fn main() {
    let start = Instant::now();
    part1();
    println!("*** Part 1 Took {:.2?} ***", start.elapsed());
    let start2 = Instant::now();
    part2();
    println!("*** Part 2 Took {:.2?} ***", start2.elapsed());
}
