use itertools::izip;
use std::time::Instant;


#[derive(Clone, Debug, PartialEq, Eq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn will_time_win(&self, input_time: usize) -> bool {
        self.distance < input_time * self.time - input_time * input_time
    }
}


fn get_input_lines() -> Vec<&'static str>{
    let input: &str = include_str!("../input.txt");
    let mut data: Vec<&str> = Vec::new();
    for line in input.lines() {
        data.push(line)
    }
    data
}

fn parse_input1() -> Vec<Race>{
    let mut races: Vec<Race> = Vec::new();
    let data = get_input_lines();
    let times_binding = data[0].replace("Time:", "");
    let distance_binding = data[1].replace("Distance:", "");
    let mut times_strings: Vec<&str> = times_binding.trim().split(' ').collect();
    let mut distances_strings: Vec<&str> = distance_binding.trim().split(' ').collect();
    times_strings.retain(|&x| !x.is_empty());
    distances_strings.retain(|&x| !x.is_empty());
    for (time, distance) in izip!(times_strings, distances_strings) {
        races.push(Race { time: time.parse::<usize>().unwrap(), distance: distance.parse::<usize>().unwrap() })
    }
    races

}

fn parse_input2() -> Race{
    let data = get_input_lines();
    let times_binding = data[0].replace("Time:", "");
    let distance_binding = data[1].replace("Distance:", "");
    let time_string: String = times_binding.replace(' ', "");
    let distance_string: String = distance_binding.replace(' ', "");
    Race{ time: time_string.parse::<usize>().unwrap(), distance: distance_string.parse::<usize>().unwrap() }

}

fn part1() {
    let races: Vec<Race> = parse_input1();
    let mut counter: usize;
    let mut total: usize = 1;
    for race in races {
        counter = 0;
        for t in 0..race.time {
            if race.will_time_win(t) {
                counter += 1;
            }
        }
        total *= counter;
    }
    println!("Part1 Answer: {}", total);
}

fn part2() {
    let race: Race = parse_input2();
    let mut counter: usize = 0;
    for t in 0..race.time {
        if race.will_time_win(t) {
            counter += 1;
        }
    }
    println!("Part2 Answer: {}", counter);
}


fn main() {
    
    let mut before = Instant::now();
    part1();
    println!("Elapsed time: {:.2?}", before.elapsed());
    before = Instant::now();
    part2();
    println!("Elapsed time: {:.2?}", before.elapsed());
}
