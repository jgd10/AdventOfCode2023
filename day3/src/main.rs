use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Grid {
    chars: Vec<Vec<char>>,
    gears: HashMap<(usize, usize), Vec<usize>>,
}


impl Grid {
    fn is_symbol_adjacent(&self, i: usize, j: usize) -> (bool, HashSet<(usize, usize)>) {
        let mut iplus: usize = i + 1;
        let mut iminus_temp: i32 = i as i32;
        let mut jplus: usize = j + 1;
        let mut jminus_temp: i32 = j as i32;
        let iminus: usize;
        let jminus: usize;
        iminus_temp -= 1;
        jminus_temp -= 1;
        if iplus >= self.chars.len() {
            iplus = self.chars.len() - 1;
        }
                
        if jplus >= self.chars[0].len() {
            jplus = self.chars[0].len() - 1;
        }

        if iminus_temp < 0 {
            iminus = 0;
        }
        else {
            iminus = i - 1;
        }
                
        if jminus_temp < 0 {
            jminus = 0;
        }
        else {
            jminus = j - 1;
        }

        let pairs: [(usize, usize); 8] = [
            (iminus, j), 
            (iplus, j), 
            (iminus, jminus), 
            (iplus, jminus), 
            (iminus, jplus), 
            (iplus, jplus), 
            (i, jminus), 
            (i, jplus)
            ];

        let mut stars: HashSet<(usize, usize)> = HashSet::new();
        let mut value: bool = false;
        for pair in pairs.into_iter() {
            if !(self.chars[pair.0][pair.1] == '.' || self.chars[pair.0][pair.1].is_ascii_digit()){
                value = true
            }
            if self.chars[pair.0][pair.1]  == '*' {
                stars.insert(pair);
            }
        }
        return (value, stars)

    }
    fn get_number(&self, i: usize, j: usize) -> (usize, bool, usize, HashSet<(usize, usize)>) {
        let ii: usize = i;
        let mut jj: usize = j;
        let mut number: String = String::new();
        let mut is_adj_symb: (bool, HashSet<(usize, usize)>) = (false, HashSet::new());
        let mut stars: HashSet<(usize, usize)> = HashSet::new();
        
        while ii < self.chars.len() && jj < self.chars[0].len() && self.chars[ii][jj].is_ascii_digit(){
            if !is_adj_symb.0 {
                is_adj_symb = self.is_symbol_adjacent(ii, jj);
                stars.extend(&mut is_adj_symb.1.iter());
            }
            if self.chars[ii][jj].is_ascii_digit() {
                number.push(self.chars[ii][jj])
            }
            else {
                return (number.parse::<usize>().unwrap(), is_adj_symb.0, jj, stars);
            }
            jj += 1;
        }
        return (number.parse::<usize>().unwrap(), is_adj_symb.0, jj, stars);
    }
}

fn parse_input() -> Grid {
    let input: &str = include_str!("../input.txt");
    let mut char_vec: Vec<char>;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_vec = line.chars().collect();
        data.push(char_vec)
    }
    return Grid{chars: data, gears: HashMap::new()};
}

fn part1(){
    let grid: Grid = parse_input();
    let mut info: (usize, bool, usize, HashSet<(usize, usize)>);
    let mut total: usize = 0;
    for (i, row) in grid.chars.iter().enumerate() {
        info = (0, false, 0, HashSet::new());
        for (j, c) in row.iter().enumerate() {
            if c.is_ascii_digit() && (j > info.2 || j == 0){
                info = grid.get_number(i, j);
                if info.1 {
                    total += info.0;
                }
            }
    }
    }
    println!("Part1 Answer: {}", total);
}

fn part2() {
    let mut grid: Grid = parse_input();
    let mut info: (usize, bool, usize, HashSet<(usize, usize)>);
    let mut total: usize = 0;
    for (i, row) in grid.chars.iter().enumerate() {
        info = (0, false, 0, HashSet::new());
        for (j, c) in row.iter().enumerate() {
            if c.is_ascii_digit() && (j > info.2 || j == 0){
                info = grid.get_number(i, j);
                for star in info.3 {
                    grid.gears.entry(star).or_insert(Vec::new()).push(info.0);
                }
            }
    }
    }
    for (star, nums) in grid.gears{
        //dbg!(star, nums);
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }
    println!("Part2 Answer: {}", total);
}

fn main() {
    part1();
    part2();
}
