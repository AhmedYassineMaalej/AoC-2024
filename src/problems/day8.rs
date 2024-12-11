use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    col: i64,
    row: i64,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col - rhs.col,
            row: self.row - rhs.row,
        }
    }
}

impl Position {
    fn new(col: usize, row: usize) -> Self {
        Self {
            col: col.try_into().unwrap(),
            row: row.try_into().unwrap(),
        }
    }

    fn check_bounds(&self, width: i64, height: i64) -> bool {
        0 <= self.col && self.col < width && 0 <= self.row && self.row < height
    }
}

#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day8.txt");

    let height = input.lines().count().try_into().unwrap();
    let width = input.lines().next().unwrap().len().try_into().unwrap();

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.char_indices() {
            if char == '.' {
                continue;
            }

            antennas
                .entry(char)
                .or_default()
                .push(Position::new(col, row));
        }
    }

    let mut antinodes: HashSet<Position> = HashSet::new();
    for positions in antennas.values() {
        for pair in positions.iter().combinations(2) {
            let antenna_1 = *pair[0];
            let antenna_2 = *pair[1];

            let diff = antenna_2 - antenna_1;

            let antinode_1 = antenna_2 + diff;
            let antinode_2 = antenna_1 - diff;

            if antinode_1.check_bounds(width, height) {
                antinodes.insert(antinode_1);
            }

            if antinode_1.check_bounds(width, height) {
                antinodes.insert(antinode_2);
            }
        }
    }

    antinodes.len()
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day8.txt");

    let height = input.lines().count().try_into().unwrap();
    let width = input.lines().next().unwrap().len().try_into().unwrap();

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.char_indices() {
            if char == '.' {
                continue;
            }
            antennas
                .entry(char)
                .or_default()
                .push(Position::new(col, row));
        }
    }

    let mut antinodes: HashSet<Position> = HashSet::new();

    for positions in antennas.values() {
        for pair in positions.iter().combinations(2) {
            let antenna_1 = *pair[0];
            let antenna_2 = *pair[1];

            let diff = antenna_2 - antenna_1;

            let mut antinode = antenna_2;

            while antinode.check_bounds(width, height) {
                antinodes.insert(antinode);
                antinode = antinode + diff;
            }

            let mut antinode = antenna_1;

            while antinode.check_bounds(width, height) {
                antinodes.insert(antinode);
                antinode = antinode - diff;
            }
        }
    }

    antinodes.len()
}
