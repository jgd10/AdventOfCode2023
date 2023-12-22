mod aoc_parser;
use aoc_parser::get_input_as_chars;
use std::{time::Instant, collections::{BTreeSet, HashMap}, borrow::BorrowMut};


fn get_count_per_square(coords: BTreeSet<Coord32>) -> HashMap<(i32, i32), BTreeSet<Coord32>> {
    let mut counts = HashMap::new();
    for coord in coords{
        let key = (coord.x_iteration, coord.y_iteration);
        if !counts.contains_key(&key){
            counts.insert(key, BTreeSet::from([coord.get_base()]));
        }
        else {
            counts.entry(key).and_modify(|coords2: &mut BTreeSet<Coord32>| {coords2.insert(coord);}).or_insert(BTreeSet::from([coord.get_base()]));
        }
    }
    counts
}

fn get_iteration_states(coords: BTreeSet<Coord32>) -> HashMap<(i32, i32), BTreeSet<Coord32>> {
    let mut counts = HashMap::new();
    for coord in coords{
        let key = (coord.x_iteration, coord.y_iteration);
        if !counts.contains_key(&key){
            counts.insert(key, BTreeSet::from([coord.get_base()]));
        }
        else {
            counts.entry(key).and_modify(|coords2: &mut BTreeSet<Coord32>| {coords2.insert(coord);}).or_insert(BTreeSet::from([coord.get_base()]));
        }
    }
    counts
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Coord32 {
    pub x: i32,
    pub y: i32,
    x_iteration: i32, 
    y_iteration: i32, 
}

impl Coord32 {
    fn get_base(&self) -> Coord32{
        Coord32 { x: self.x, y: self.y, x_iteration: 0, y_iteration: 0 }
    }
}

struct Garden {
    edges: BTreeSet<Coord32>,
    ground: BTreeSet<Coord32>,
    start: Coord32,
    xmax: i32,
    ymax: i32,
}

impl Garden {
    fn unique_squares_after_n_steps(&self, n: usize) -> BTreeSet<Coord32>{
        let mut queue: BTreeSet<Coord32> = BTreeSet::from([self.start]);
        for _k in 0..n{
            let mut new_queue: BTreeSet<Coord32> = BTreeSet::new();
            for current_square in queue.clone(){
                let next_coords: BTreeSet<Coord32> = self.get_next_coords(current_square);
                for coord in next_coords{
                    if self.ground.contains(&coord.get_base()){
                        new_queue.insert(coord);
                    }
                }
            }
            queue = new_queue;
        }

        queue
    }
    fn unique_squares_after_many_steps(&self, n: usize) -> (BTreeSet<Coord32>, HashMap<(i32, i32), usize>){
        let mut queue: BTreeSet<Coord32> = BTreeSet::from([self.start]);
        let mut states: HashMap<(i32, i32), BTreeSet<usize>> = HashMap::new();
        let mut stabilised: HashMap<(i32, i32), usize> = HashMap::new();
        for _k in 0..n{
            let iteration_states = get_iteration_states(queue.clone());
            for (k, v) in iteration_states{
                if !states.contains_key(&k){
                    states.insert(k, v.len());
                    stabilised.insert(k, v.len());
                }
                else {
                    if states.get(&k).unwrap().contains(&v.len()) && *stabilised.get(&k).unwrap() == 0 {
                        dbg!(_k+1);
                        stabilised.entry(k).and_modify(|x| {let x = _k+1;}).or_insert(0);
                    }
                }
            }
            let mut new_queue: BTreeSet<Coord32> = BTreeSet::new();
            for current_square in queue.clone(){
                let next_coords: BTreeSet<Coord32> = self.get_next_coords(current_square);
                for coord in next_coords{
                    if self.ground.contains(&coord.get_base()){
                        new_queue.insert(coord);
                    }
                }

            }
            queue = new_queue;

        }
        (queue, stabilised)

    }
    fn get_next_coords(&self, coordinate: Coord32) -> BTreeSet<Coord32> {
        let mut west_new = Coord32{x: coordinate.x - 1, y: coordinate.y, x_iteration: coordinate.x_iteration, y_iteration: coordinate.y_iteration};
        let mut east_new = Coord32{x: coordinate.x + 1, y: coordinate.y, x_iteration: coordinate.x_iteration, y_iteration: coordinate.y_iteration};
        let mut north_new = Coord32{x: coordinate.x, y: coordinate.y - 1, x_iteration: coordinate.x_iteration, y_iteration: coordinate.y_iteration};
        let mut south_new = Coord32{x: coordinate.x, y: coordinate.y + 1, x_iteration: coordinate.x_iteration, y_iteration: coordinate.y_iteration};
        if self.edges.contains(&coordinate.get_base()){
            if west_new.x < 0 {
                west_new.x = self.xmax;
                west_new.x_iteration -= 1;
            }
            else if west_new.x > self.xmax {
                west_new.x = 0;
                west_new.x_iteration += 1;
            }
            if east_new.x < 0 {
                east_new.x = self.xmax;
                east_new.x_iteration -= 1;
            }
            else if east_new.x > self.xmax {
                east_new.x = 0;
                east_new.x_iteration += 1;
            }
            //
            if north_new.y < 0 {
                north_new.y = self.ymax;
                north_new.y_iteration -= 1;
            }
            else if north_new.y > self.ymax {
                north_new.y = 0;
                north_new.y_iteration += 1;
            }
            if south_new.y < 0 {
                south_new.y = self.ymax;
                south_new.y_iteration -= 1;
            }
            else if south_new.y > self.ymax {
                south_new.y = 0;
                south_new.y_iteration += 1;
            }
            
        }

        BTreeSet::from([east_new, west_new, north_new, south_new])
    }
}


fn parse_input(input: &str) -> Garden{
    let data = get_input_as_chars(input);
    let mut ground: BTreeSet<Coord32> = BTreeSet::new();
    let mut rocks: BTreeSet<Coord32> = BTreeSet::new();
    let mut start: BTreeSet<Coord32> = BTreeSet::new();
    let mut edges: BTreeSet<Coord32> = BTreeSet::new();
    let xmax: i32 = (data[0].len() - 1) as i32;
    let ymax: i32 = (data.len() - 1) as i32;
    for (i, row) in data.iter().enumerate() {
        edges.insert(Coord32{x: 0, y: i as i32, x_iteration: 0, y_iteration: 0});
        edges.insert(Coord32{x: (data.len() - 1) as i32, y: i as i32, x_iteration: 0, y_iteration: 0});
        for (j, c) in row.iter().enumerate(){
            edges.insert(Coord32{x: j as i32, y: 0, x_iteration: 0, y_iteration: 0});
            edges.insert(Coord32{x: j as i32, y: (row.len()-1) as i32, x_iteration: 0, y_iteration: 0});
            match c {
                '.' => ground.insert(Coord32{x: j as i32, y: i as i32, x_iteration: 0, y_iteration: 0}),
                '#' => rocks.insert(Coord32{x: j as i32, y: i as i32, x_iteration: 0, y_iteration: 0}),
                'S' => start.insert(Coord32{x: j as i32, y: i as i32, x_iteration: 0, y_iteration: 0}),
                _ => unreachable!(),
            };
        }
    }

    ground.append(&mut start.clone());
    Garden{edges, ground, start: start.pop_first().unwrap(), xmax, ymax}

}

#[test]
fn test_example_part1(){
    let garden: Garden = parse_input(include_str!("../example.txt"));
    let result: BTreeSet<Coord32> = garden.unique_squares_after_n_steps(6);
    assert_eq!(result.len(), 16);
}

#[test]
fn test_example_part2a(){
    let garden: Garden = parse_input(include_str!("../example.txt"));
    let result: BTreeSet<Coord32> = garden.unique_squares_after_n_steps(10);
    assert_eq!(result.len(), 50);
}

#[test]
fn test_example_part2b(){
    let garden: Garden = parse_input(include_str!("../example.txt"));
    let result: BTreeSet<Coord32> = garden.unique_squares_after_n_steps(200);
    dbg!(get_count_per_square(result.clone()));
    assert_eq!(result.len(), 6536);
}

fn part1(){
    let garden: Garden = parse_input(include_str!("../input.txt"));
    let result: BTreeSet<Coord32> = garden.unique_squares_after_n_steps(64);
    println!("Part 1 Answer: {}", result.len());
}


fn part2(){
    let garden: Garden = parse_input(include_str!("../input.txt"));
    let result: (BTreeSet<Coord32>, HashMap<(i32, i32), usize>) = garden.unique_squares_after_many_steps(100);
    dbg!(result.1);
    println!("Part 2 Answer: {}", result.0.len());}

fn main() {
    let start = Instant::now();
    part1();
    println!("*** Part 1 Took {:.2?} ***", start.elapsed());
    let start2 = Instant::now();
    part2();
    println!("*** Part 2 Took {:.2?} ***", start2.elapsed());
}
