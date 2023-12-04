use std::collections::HashSet;
use std::collections::BTreeMap;
use num_traits::pow;

#[derive(Clone, Debug)]
struct Deck {
    cards: BTreeMap<usize, Scratchcard>,
    numbers: BTreeMap<usize, usize>,
}

#[derive(Clone, Debug)]
struct Scratchcard {
    number: u32,
    player_numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}

impl Scratchcard {
    fn get_num_wins(&self) -> usize {
        let player_score: HashSet<usize> = self.player_numbers.intersection(&self.winning_numbers).copied().collect();
        player_score.len()
    }
    fn get_score(&self) -> usize {
        let exponent: usize = self.get_num_wins();
        if exponent == 0 {
            0
        }
        else {
            pow(2, exponent.saturating_sub(1))
        }
    }
}

impl Deck {
    fn get_size(&mut self) -> usize {
        self.evaluate();
        self.numbers.values().sum()
    }
    fn evaluate(&mut self) {
        let mut start_id: usize;
        let mut end_id: usize;
        let mut card_score: usize;
        for (id_, card) in &self.cards {
            card_score = card.get_num_wins();
            start_id = id_ + 1;
            end_id = id_ + card_score;
            for i in start_id..=end_id {
                *self.numbers.entry(i).or_insert(0) += self.numbers[id_];
            }
        }
    }
}


fn get_input_lines() -> Vec<&'static str>{
    let input: &str = include_str!("../example.txt");
    let mut data: Vec<&str> = Vec::new();
    for line in input.lines() {
        data.push(line)
    }
    data
}

fn parse_line(line: &str) -> Scratchcard {
    let mut player_nums: HashSet<usize> = HashSet::new();
    let mut winner_nums: HashSet<usize> = HashSet::new();
    let blocks1: Vec<&str> = line.split(':').collect();
    let id_: u32 = blocks1[0].replace("Card", "").trim().parse::<u32>().unwrap();
    let string_numbers: Vec<&str> = blocks1[1].split('|').collect();
    let winning_number_strings: Vec<&str> = string_numbers[0].split(' ').collect();
    let player_number_strings: Vec<&str> = string_numbers[1].split(' ').collect();
    for win in winning_number_strings {
        if !win.is_empty() {
            winner_nums.insert(win.parse::<usize>().unwrap());
        }
        
    }
    
    for play in player_number_strings {
        if !play.is_empty() {
            player_nums.insert(play.parse::<usize>().unwrap());
        }
    }

    Scratchcard { number: id_, player_numbers: player_nums, winning_numbers: winner_nums }
}

fn part1() {
    let data = get_input_lines();
    let mut total: usize = 0;
    let mut card: Scratchcard;
    for line in data {
        card = parse_line(line);
        total += card.get_score();
    }
    println!("Part1 Answer: {}", total)
}

fn part2() {
    let data = get_input_lines();
    let mut card: Scratchcard;
    let mut deck: Deck = Deck { cards: BTreeMap::new(), numbers: BTreeMap::new() };
    for line in data {
        card = parse_line(line);
        deck.cards.insert(card.number as usize, card.clone());
        deck.numbers.insert(card.number as usize, 1);
    }
    println!("Part2 Answer: {}", deck.get_size())
}

fn main() {
    part1();
    part2();
}
