mod aoc_parser;
use std::collections::HashSet;

use aoc_parser::{get_input_lines, InputType};

#[derive(Clone, Debug)]
struct Hand {
    cards: String,
    bid: usize,
}

impl Hand {
    fn get_value(&self) -> String {
        let suffix = self.get_suffix2();
        let mut prefix: String;
        let mut set_cards: HashSet<char> = self.cards.chars().collect();
        set_cards.remove(&'J');
        if set_cards.len() == 0 {
            prefix = "7".to_string();
        }
        else {
            match set_cards.len() {
                // all same
                1 => prefix = "7".to_string(),
                // full house/4 of a kind
                2 => prefix = self.fullhouse_or_4ofakind(),
                // 3 of a kind/2 pair
                3 => prefix = self.ofakind3_or_2pair(),
                // one pair
                4 => prefix = "2".to_string(),
                // high card/nothing
                5 => prefix = "1".to_string(),
                // uh-oh
                _ => unreachable!(),
            }
        }
        prefix.push_str(&suffix);
        prefix
    }
    fn fullhouse_or_4ofakind(&self) -> String{
        let mut set_cards: HashSet<char> = self.cards.chars().collect();
        let elem = set_cards.iter().next().unwrap().clone();
        let mut result: usize = 0;
        if set_cards.contains(&'J') {
            set_cards.remove(&'J');
            for c in set_cards {
                match self.cards.matches(c).count() {
                    1 => result = result.max(6),
                    2 => result = result.max(5),
                    3 => result = result.max(6),
                    _ => unreachable!(),
                }
            }

        }
        else {
            match self.cards.matches(elem).count() {
                1 | 4 => result = result.max(6),
                2 | 3 => result = result.max(5),
                _ => unreachable!(),
            }
        }
        result.to_string()
    }
    fn ofakind3_or_2pair(&self) -> String{
        let set_cards: HashSet<char> = self.cards.chars().collect();
        let mut result: usize = 0;
        if set_cards.contains(&'J') {
            result = result.max(4);
        }
        else {
            for c in set_cards{
                match self.cards.matches(c).count() {
                    1 => (),
                    2 => result = result.max(3),
                    3 => result = result.max(4),
                    _ => unreachable!(),
                }
            }
        }
        result.to_string()
    }
    fn get_suffix(&self) -> String{
        let mut suffix: String = "".to_owned();
        for c in self.cards.chars(){
            match c {
                '2' => suffix.push_str("a"),
                '3' => suffix.push_str("b"),
                '4' => suffix.push_str("c"),
                '5' => suffix.push_str("d"),
                '6' => suffix.push_str("e"),
                '7' => suffix.push_str("f"),
                '8' => suffix.push_str("g"),
                '9' => suffix.push_str("h"),
                'T' => suffix.push_str("i"),
                'J' => suffix.push_str("j"),
                'Q' => suffix.push_str("k"),
                'K' => suffix.push_str("l"),
                'A' => suffix.push_str("m"),
                _ => suffix.push_str("z"),
            }
        }
        suffix
    }
    fn get_suffix2(&self) -> String{
        let mut suffix: String = "".to_owned();
        for c in self.cards.chars(){
            match c {
                '2' => suffix.push_str("n"),
                '3' => suffix.push_str("o"),
                '4' => suffix.push_str("p"),
                '5' => suffix.push_str("q"),
                '6' => suffix.push_str("r"),
                '7' => suffix.push_str("s"),
                '8' => suffix.push_str("t"),
                '9' => suffix.push_str("u"),
                'T' => suffix.push_str("v"),
                'J' => suffix.push_str("b"),
                'Q' => suffix.push_str("x"),
                'K' => suffix.push_str("y"),
                'A' => suffix.push_str("z"),
                _ => suffix.push_str("a"),
            }
        }
        suffix
    }
}


fn parse_input() -> Vec<Hand>{
    let data = get_input_lines(InputType::Input);
    let mut hands: Vec<Hand> = Vec::new();
    for row in data {
        let binding: Vec<&str> = row.split_ascii_whitespace().collect();
        hands.push(Hand { cards: binding[0].to_string(), bid: binding[1].parse::<usize>().unwrap() })
    }
    hands
}

fn part1() {
    let mut hands: Vec<Hand> = parse_input();
    let mut total: usize = 0;
    hands.sort_by(|a, b| a.get_value().cmp(&b.get_value()));
    for (i, hand) in hands.iter().enumerate() {
        total += (i+1)*hand.bid;
    }
    println!("Part 1 Answer: {}", total);
}

fn part2() {
    let mut hands: Vec<Hand> = parse_input();
    let mut total: usize = 0;
    hands.sort_by(|a, b| a.get_value().cmp(&b.get_value()));
    // dbg!(hands.clone());
    for (i, hand) in hands.iter().enumerate() {
        total += (i+1)*hand.bid;
    }
    println!("Part 2 Answer: {}", total);
}

#[test]
pub fn test_rust_sorting() {
    let mut a = vec!["1aaa", "1aab", "1caa", "3ddd", "3cdef", "5abcd"];
    a.sort();
    dbg!(a);
}

#[test]
pub fn test_rust_char_count() {
    let a = "JAJ66";
    dbg!(a.chars().count());
}




fn main() {
    // part1();
    part2();
}
