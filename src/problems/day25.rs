use std::collections::HashSet;

fn fits(key: &[usize; 5], lock: &[usize; 5]) -> bool {
    key.iter()
        .zip(lock)
        .all(|(k_pin, l_pin)| k_pin + l_pin <= 5)
}

fn get_columns(body: Vec<Vec<char>>) -> [usize; 5] {
    let mut columns = [0; 5];

    for row in body {
        for (idx, char) in row.into_iter().enumerate() {
            if char == '#' {
                columns[idx] += 1;
            }
        }
    }

    columns
}

fn parse(input: &str) -> (HashSet<[usize; 5]>, HashSet<[usize; 5]>) {
    let mut locks = HashSet::new();
    let mut keys = HashSet::new();

    for block in input.split("\r\n\r\n") {
        let mut lines = block.lines();

        let first = lines.next().unwrap();
        let body: Vec<Vec<char>> = lines.take(5).map(|line| line.chars().collect()).collect();

        let columns = get_columns(body);
        // let hash = hash_columns(columns);

        if first == "....." {
            keys.insert(columns);
        } else {
            locks.insert(columns);
        }
    }

    (locks, keys)
}

#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day25.txt");

    let (locks, keys) = parse(input);

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if fits(key, lock) {
                count += 1;
            }
        }
    }

    count
}