use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day19.txt");

    let (towels, designs) = input.split_once("\r\n\r\n").unwrap();

    let towels: Vec<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    let mut count = 0;
    for design in designs {
        if can_make(design, &towels) {
            count += 1;
        }
    }

    count
}

fn can_make(target: &str, towels: &[&str]) -> bool {
    if target.is_empty() {
        return true;
    }
    
    for towel in towels {
        if target.starts_with(towel) && can_make(target.strip_prefix(towel).unwrap(), towels) {
            return true;
        }
    }
    
    false
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day19.txt");

    let (towels, designs) = input.split_once("\r\n\r\n").unwrap();

    let towels: HashSet<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();
    let mut memo: HashMap<&str, usize> = HashMap::new();
    
    // only one way to make the empty string
    memo.insert("", 1); 

    let mut ways = 0;
    for design in designs {
        ways += get_ways_to_make(design, &towels, &mut memo);   
    }

    ways
}

fn get_ways_to_make<'a>(target: &'a str, towels: &HashSet<&str>, memo: &mut HashMap<&'a str, usize>) -> usize {
    if memo.contains_key(target) {
        return memo[target];
    }

    let mut ways = 0;

    // check all prefixes of size 1 through 8
    // since the biggest towel is of size 8
    for i in 0..target.len().min(8) {
        let (prefix, suffix) = target.split_at(i + 1);

        if towels.contains(prefix) {
            ways += get_ways_to_make(suffix, towels, memo);
        }
    }

    memo.insert(target, ways);

    ways
}