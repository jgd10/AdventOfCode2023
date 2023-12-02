use std::cmp::max;


#[derive(Clone, Copy, Debug)]
struct Hand {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}


impl Hand {
    fn compatible_with_hand(&self, other: Hand) -> bool{
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green 
    }
    fn get_hand_power(&self) -> usize {
        self.red*self.blue*self.green
    }
}

impl Game {
    fn get_minimum_hand(&self) -> Hand {
        let mut red: usize = 0;
        let mut blue: usize = 0;
        let mut green: usize = 0;
        for hand in self.hands.clone() {
            red = max(red, hand.red);
            blue = max(blue, hand.blue);
            green = max(green, hand.green);
        }
        Hand { red, blue, green }
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

fn parse_hand(string: &str) -> Hand {
    let cube_sets: Vec<&str> = string.split(',').collect();
    let mut details: Vec<&str>;
    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;
    let mut amount: usize;
    for cube_set in cube_sets {
        details = cube_set.split_whitespace().collect();
        amount = details[0].parse::<usize>().unwrap();
        match details[1] {
            "red" => red = amount,
            "green" => green = amount,
            "blue" => blue = amount,
            _ => (),
        } 
    }
    Hand{red, blue, green}
}

fn parse_line(line: &str) -> Game {
    let mut hands: Vec<Hand> = Vec::new();
    let blocks: Vec<&str> = line.split(':').collect();
    let id_: u32 = blocks[0].replace("Game ", "").parse::<u32>().unwrap();
    let string_hands: Vec<&str> = blocks[1].split(';').collect();
    for hand_string in string_hands {
        hands.push(parse_hand(hand_string))
    }
    Game{id: id_, hands}
}


fn parse_input() -> Vec<Game> {
    let data = get_input_lines();
    let mut games: Vec<Game> = Vec::new();
    for line in data {
        games.push(parse_line(line))
    }
    games
}

fn is_possible_game(game: Game, min_hand: Hand) -> bool {
    for hand in game.hands {
        if !hand.compatible_with_hand(min_hand) {
            return false;
        }
    }
    true
}


fn part1() {
    let games: Vec<Game> = parse_input();
    let minimum_hand: Hand = Hand { red: 12, blue: 14, green: 13 };
    let mut total: u32 = 0;
    for game in games {
        if is_possible_game(game.clone(), minimum_hand) {
            total += game.id;
        }
    }
    println!("Part1 Answer: {}", total);
}


fn part2() {
    let games: Vec<Game> = parse_input();
    let mut total: usize = 0;
    for game in games {
        total += game.get_minimum_hand().get_hand_power();
    }
    println!("Part2 Answer: {}", total);
}

fn main() {
    part1();
    part2();
}
