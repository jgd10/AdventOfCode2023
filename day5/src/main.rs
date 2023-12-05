use itertools::izip;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Overlap {
    PartialLeft,
    PartialRight,
    PartialMiddle,
    Total,
    None,
}

#[derive(Clone, Debug)]
struct Range2 {
    start: usize,
    size: usize,
}

impl Range2 {
    fn overlap(&self, other: Range2) -> Overlap{
        let a0: usize = self.start;
        let a1: usize = self.start + self.size - 1;
        let b0: usize = other.start;
        let b1: usize = other.start + other.size - 1;
        // dbg!(a0, a1, b0, b1);
        if a0 > b1 || b0 > a1 {
            return Overlap::None;
        }
        else if a0 <= b0 && a1 < b1 {
            return Overlap::PartialLeft;
        }
        else if a0 > b0 && a1 >= b1 {
            return  Overlap::PartialRight;
        }
        else if a0 > b0 && a1 < b1 {
            return Overlap::PartialMiddle;
        }
        else if a0 <= b0 && a1 >= b1 {
            return Overlap::Total;
        }
        else {
            unreachable!();
        }
    }
    fn map_to_partial_left(&self, other: Range2, out: Range2) -> Vec<Range2> {
        let a0: usize = self.start;
        let a1: usize = self.start + self.size;
        let b0: usize = other.start;
        let b1: usize = other.start + other.size;
        let trunc_range: Range2 = Range2 { start: a1, size: b1-a1 };
        let mapped_range: Range2 = Range2 { start: b0-a0+out.start, size: a1-b0 };
        assert_eq!(trunc_range.size+mapped_range.size, other.size);
        return vec![trunc_range, mapped_range];
    }
    fn map_to_partial_right(&self, other: Range2, out: Range2) -> Vec<Range2> {
        let a0: usize = self.start;
        let b0: usize = other.start;
        let b1: usize = other.start + other.size;
        let trunc_range: Range2 = Range2 { start: b0, size: a0-b0 };
        let mapped_range: Range2 = Range2 { start: out.start, size: b1-a0 };
        assert_eq!(trunc_range.size+mapped_range.size, other.size);
        return vec![trunc_range, mapped_range];
    }
    fn map_to_partial_middle(&self, other: Range2, out: Range2) -> Vec<Range2> {
        let a0: usize = self.start;
        let a1: usize = self.start + self.size;
        let b0: usize = other.start;
        let b1: usize = other.start + other.size;
        let trunc_range1: Range2 = Range2 { start: b0, size: a0-b0 };
        let trunc_range2: Range2 = Range2 { start: a1, size: b1-a1 };
        let mapped_range: Range2 = Range2 { start: out.start, size: out.size };
        assert_eq!(trunc_range1.size+trunc_range2.size+mapped_range.size, other.size);
        return vec![trunc_range1, trunc_range2, mapped_range];
    }
    fn map_to_total(&self, other: Range2, out: Range2) -> Vec<Range2> {
        let a0: usize = self.start;
        let b0: usize = other.start;
        let mapped_range: Range2 = Range2 { start: b0-a0+out.start, size: other.size };
        assert_eq!(mapped_range.size, other.size);
        return vec![mapped_range];
    }
}

#[derive(Clone, Debug)]
struct MultiRange {
    ranges: Vec<Range2>,
}

#[derive(Clone, Debug)]
struct AlmanacMap2 {
    in_name: String,
    out_name: String,
    in_range: MultiRange,
    out_range: MultiRange,
}

#[derive(Clone, Debug)]
struct AlmanacMap {
    in_name: String,
    out_name: String,
    in_nums: Vec<usize>,
    out_nums: Vec<usize>,
    numbers: Vec<usize>,
}

impl MultiRange {
    fn get_lowest_value(&self) -> usize {
        let mut value: usize = usize::MAX;
        // if we filter out values equal to zero it works for the input
        // if not it only works for the example. Why??
        for range2 in self.ranges.clone() {
            value = value.min(range2.start);
        }
        value
    }
}

impl AlmanacMap {
    fn map_to(&self, num: usize) -> usize {
        let mut upper_range: usize;
        for (in_num, out_num, number) in izip!(&self.in_nums, &self.out_nums, &self.numbers) {
            upper_range = in_num+number;
            if &num >= in_num && num <= upper_range {
                return  num - in_num + out_num;
            }
        }
        return num
    }
    fn to_almanac2(&self) -> AlmanacMap2{
        let mut in_ranges: Vec<Range2> = Vec::new();
        let mut out_ranges: Vec<Range2> = Vec::new();
        for (in_num, out_num, number) in izip!(&self.in_nums, &self.out_nums, &self.numbers) {
            in_ranges.push(Range2{start: in_num.clone(), size: number.clone()});
            out_ranges.push(Range2{start: out_num.clone(), size: number.clone()});
        }
        return AlmanacMap2{in_name: self.in_name.clone(), out_name: self.out_name.clone(), in_range: MultiRange { ranges: in_ranges }, out_range: MultiRange { ranges: out_ranges }};
    }
    
}

impl AlmanacMap2 {
    fn map_ranges_to_ranges(&self, ranges: MultiRange) -> MultiRange{
        let mut new_ranges: Vec<Range2> = Vec::new();
        let mut overlap: Overlap;
        for range2 in ranges.ranges.clone(){
            for (in_range, out_range) in izip!(self.in_range.ranges.clone(), self.out_range.ranges.clone()) {
                overlap = in_range.overlap(range2.clone());
                match overlap {
                    Overlap::Total => new_ranges.append(&mut in_range.map_to_total(range2.clone(), out_range)),
                    Overlap::PartialLeft => new_ranges.append(&mut in_range.map_to_partial_left(range2.clone(), out_range)),
                    Overlap::PartialRight => new_ranges.append(&mut in_range.map_to_partial_right(range2.clone(), out_range)),
                    Overlap::PartialMiddle => new_ranges.append(&mut in_range.map_to_partial_middle(range2.clone(), out_range)),
                    Overlap::None => (),
                }
            }
            if new_ranges.len() == 0 {
                new_ranges.push(range2);
            }
        }
        MultiRange { ranges: new_ranges }
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

fn parse_input() -> (Vec<usize>, HashMap<String, AlmanacMap>){
    let lines = get_input_lines();
    let seed_line = lines[0].replace("seeds: ", "");
    let mut seeds: Vec<usize> = Vec::new();
    let mut blocks: Vec<Vec<&str>> = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    let mut almanac_maps: HashMap<String, AlmanacMap> = HashMap::new();
    let mut almanac: AlmanacMap;
    for line in lines[2..].into_iter(){
        if line.is_empty(){
            blocks.push(block);
            block = Vec::new();
        }
        else {
            block.push(line);
        }
    }
    blocks.push(block); 
    for block in blocks {
        almanac = parse_block(block);
        almanac_maps.insert(almanac.in_name.clone(), almanac);

    }
    for seed_string in seed_line.split(" ") {
        seeds.push(seed_string.parse::<usize>().unwrap());
    }
    return (seeds, almanac_maps);
}

fn get_seed_ranges(seeds: Vec<usize>) -> MultiRange {
    let mut ranges: Vec<Range2> = Vec::new();
    for pair in seeds.chunks(2){
        let single_range: Range2 = Range2 { start: pair[0], size: pair[1] };
        ranges.push(single_range);
    }
    MultiRange { ranges }
}

fn parse_block(block: Vec<&str>) -> AlmanacMap{
    let binding = block[0].replace(" map:", "");
    let names: Vec<&str> = binding.split("-to-").collect();
    let mut numbers: Vec<&str>;
    let mut in_number: Vec<usize> = Vec::new();
    let mut out_number: Vec<usize> = Vec::new();
    let mut range: Vec<usize> = Vec::new();
    for line in block[1..].into_iter() {
        numbers = line.split(" ").collect();
        out_number.push(numbers[0].parse::<usize>().unwrap());
        in_number.push(numbers[1].parse::<usize>().unwrap());
        range.push(numbers[2].parse::<usize>().unwrap());
    }
    return AlmanacMap{in_name: names[0].to_string(), out_name: names[1].to_string(), out_nums: out_number, in_nums: in_number, numbers: range}
}

fn find_lowest_location_number(seeds: Vec<usize>, almanacs: HashMap<String, AlmanacMap>) -> usize {
    let mut start: String = "seed".to_string();
    let mut next: usize;
    let mut lowest: usize = usize::MAX;
    for seed in seeds {
        next = seed;
        while almanacs.contains_key(&start){
            next = almanacs[&start].map_to(next);
            start = almanacs[&start].out_name.clone();
        }
        start = "seed".to_string();
        lowest = lowest.min(next);
    }
    lowest
}

fn convert_almanacs_to_ranges(almanacs: HashMap<String, AlmanacMap>) ->  HashMap<String, AlmanacMap2>{
    let mut new_almanacs: HashMap<String, AlmanacMap2> = HashMap::new();
    for (k, v) in almanacs {
            new_almanacs.insert(k, v.to_almanac2());
    }
    return new_almanacs;
}

fn find_lowest_location_number2(seeds: Vec<usize>, almanacs: HashMap<String, AlmanacMap>) -> usize {
    let almanacs2: HashMap<String, AlmanacMap2> = convert_almanacs_to_ranges(almanacs);
    let mut start: String = "seed".to_string();
    let seed_ranges: MultiRange = get_seed_ranges(seeds);
    let mut ranges: MultiRange = seed_ranges;
    while almanacs2.contains_key(&start){
        ranges = almanacs2[&start].map_ranges_to_ranges(ranges);
        start = almanacs2[&start].out_name.clone();
    }
    ranges.get_lowest_value()
}

fn part1() {
    let data: (Vec<usize>, HashMap<String, AlmanacMap>) = parse_input();
    println!("Part1 Answer: {}", find_lowest_location_number(data.0, data.1));
}

fn part2() {
    let data: (Vec<usize>, HashMap<String, AlmanacMap>) = parse_input();
    println!("Part2 Answer: {}", find_lowest_location_number2(data.0, data.1));
}

#[test]
pub fn test_partial_middle() {
    let a: Range2 = Range2 { start: 41, size: 3 };
    let b: Range2 = Range2 { start: 40, size: 6 };
    let out: Range2 = Range2 { start: 90, size: 3 };
    let results: Vec<Range2>;
    results = a.map_to_partial_middle(b, out);
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].start, 40);
    assert_eq!(results[0].size, 1);
    assert_eq!(results[1].start, 44);
    assert_eq!(results[1].size, 2);
    assert_eq!(results[2].start, 90);
    assert_eq!(results[2].size, 3);
}

#[test]
pub fn test_partial_right() {
    let a: Range2 = Range2 { start: 41, size: 2 };
    let b: Range2 = Range2 { start: 40, size: 3 };
    let out: Range2 = Range2 { start: 90, size: 2 };
    let results: Vec<Range2>;
    results = a.map_to_partial_right(b, out);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].start, 40);
    assert_eq!(results[0].size, 1);
    assert_eq!(results[1].start, 90);
    assert_eq!(results[1].size, 2);
}

#[test]
pub fn test_partial_right_zeros1() {
    let a: Range2 = Range2 { start: 41, size: 2 };
    let b: Range2 = Range2 { start: 40, size: 3 };
    let out: Range2 = Range2 { start: 0, size: 2 };
    let results: Vec<Range2>;
    results = a.map_to_partial_right(b, out);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].start, 40);
    assert_eq!(results[0].size, 1);
    assert_eq!(results[1].start, 0);
    assert_eq!(results[1].size, 2);
}

#[test]
pub fn test_partial_left_one_value() {
    let a: Range2 = Range2 { start: 3, size: 3 };
    let b: Range2 = Range2 { start: 5, size: 3 };
    let out: Range2 = Range2 { start: 0, size: 3 };
    let results: Vec<Range2>;
    results = a.map_to_partial_left(b, out);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].start, 6);
    assert_eq!(results[0].size, 2);
    assert_eq!(results[1].start, 2);
    assert_eq!(results[1].size, 1);
}

#[test]
pub fn test_partial_left() {
    let a: Range2 = Range2 { start: 41, size: 5 };
    let b: Range2 = Range2 { start: 42, size: 10 };
    let out: Range2 = Range2 { start: 90, size: 5 };
    let results: Vec<Range2>;
    results = a.map_to_partial_left(b, out);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].start, 46);
    assert_eq!(results[0].size, 6);
    assert_eq!(results[1].start, 91);
    assert_eq!(results[1].size, 4);
}

#[test]
pub fn test_total2() {
    let a: Range2 = Range2 { start: 0, size: 10 };
    let b: Range2 = Range2 { start: 0, size: 3 };
    let out: Range2 = Range2 { start: 90, size: 10 };
    let results: Vec<Range2>;
    results = a.map_to_total(b, out);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].start, 90);
    assert_eq!(results[0].size, 3);
}

#[test]
pub fn test_total3() {
    let a: Range2 = Range2 { start: 0, size: 10 };
    let b: Range2 = Range2 { start: 8, size: 3 };
    let out: Range2 = Range2 { start: 90, size: 10 };
    let results: Vec<Range2>;
    results = a.map_to_total(b, out);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].start, 98);
    assert_eq!(results[0].size, 3);
}

#[test]
pub fn test_total() {
    let a: Range2 = Range2 { start: 41, size: 5 };
    let b: Range2 = Range2 { start: 42, size: 3 };
    let out: Range2 = Range2 { start: 90, size: 5 };
    let results: Vec<Range2>;
    results = a.map_to_total(b, out);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].start, 91);
    assert_eq!(results[0].size, 3);
}

#[test]
pub fn test_overlap() {
    let a = Range2 { start: 3, size: 3 }; // 3, 4, 5
    let b = Range2 { start: 6, size: 3 }; // 6, 7, 8
    let c = Range2 { start: 3, size: 6 }; // 3, 4, 5, 6, 7, 8
    let d = Range2 { start: 4, size: 3 }; // 4, 5, 6
    let e: Range2 = Range2 { start: 6, size: 3 }; // 6, 7, 8

    // a has no overlap with b
    assert_eq!(a.overlap(b.clone()), Overlap::None);
    assert_eq!(b.overlap(a.clone()), Overlap::None);
    // a is fully contained in c
    assert_eq!(a.overlap(c.clone()), Overlap::PartialLeft);
    // d and e overlap by a single value
    assert_eq!(d.overlap(e.clone()), Overlap::PartialLeft);
    // d and e overlap by a single value reversed
    assert_eq!(e.overlap(d.clone()), Overlap::PartialRight);
    // b is fully contained in c
    assert_eq!(b.overlap(c.clone()), Overlap::PartialRight);
    // c overlaps fully with a but has more to the right
    assert_eq!(c.overlap(a.clone()), Overlap::Total);
    // c overlaps fully with b but has more to the left
    assert_eq!(c.overlap(b.clone()), Overlap::Total);
    // d overlaps partially with a but has more to the right
    assert_eq!(d.overlap(a.clone()), Overlap::PartialRight);
    // a overlaps partially with d but has more to the left
    assert_eq!(a.overlap(d.clone()), Overlap::PartialLeft);
    // d overlaps with c and produces 3 ranges
    assert_eq!(d.overlap(c.clone()), Overlap::PartialMiddle);
    // d overlaps with c reverse
    assert_eq!(c.overlap(d.clone()), Overlap::Total);
}

fn main() {
    part1();
    part2();
}
