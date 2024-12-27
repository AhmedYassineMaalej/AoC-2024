use std::{cmp::Ordering, collections::HashSet};

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (orderings, updates) = input.split_once("\r\n\r\n").unwrap();

    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &usize, y: &usize| !orderings.contains(&(*y, *x));

    let mut count = 0;
    for update in updates.lines() {
        let update: Vec<usize> = update.split(',').map(|x| x.parse().unwrap()).collect();

        if update.is_sorted_by(compare) {
            count += update[update.len() / 2];
        }
    }

    count
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (orderings, updates) = input.split_once("\r\n\r\n").unwrap();

    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &usize, y: &usize| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            Ordering::Less
        } else if orderings.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    updates
        .lines()
        .map(|update| {
            let mut update: Vec<usize> = update.split(',').map(|x| x.parse().unwrap()).collect();

            if update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
                0
            } else {
                update.sort_by(compare);
                update[update.len() / 2]
            }
        })
        .sum()
}
