#![warn(clippy::pedantic)]

use std::time::Instant;
mod problems;

#[allow(unused)]
const PROBLEM_NAMES: [&str; 25] = [
    "Historian Hysteria",
    "Red-Nosed Reports",
    "Mull It Over",
    "Ceres Search",
    "Print Queue",
    "Guard Gallivant",
    "Bridge Repair",
    "Resonant Collinearity",
    "Disk Fragmenter",
    "Hoof It",
    "Plutonian Pebbles",
    "Garden Groups",
    "Claw Contraption",
    "Restroom Redoubt",
    "Warehouse Woes",
    "Reindeer Maze",
    "Chronospatial Computer",
    "RAM Run",
    "Linen Layout",
    "Race Condition",
    "Keypad Conundrum",
    "Monkey Market",
    "LAN Party",
    "Crossed Wires",
    "Code Chronicle",
];

macro_rules! run_day {
    ($x: ident) => {
        let input = include_str!("../input/day24.txt");

        let start1 = Instant::now();
        let result1 = problems::$x::part1(input);
        let duration1 = start1.elapsed();

        let start2 = Instant::now();
        let result2 = problems::$x::part2(input);
        let duration2 = start2.elapsed();

        println!("Part 1: solution: {} || time: {:?}", result1, duration1);
        println!("Part 2: solution: {} || time: {:?}", result2, duration2);
    };
}

#[allow(unused_macros)]
macro_rules! table_row {
    ($x: ident) => {
        let day = stringify!($x);
        let problem_number: usize = day[3..].parse().unwrap();
        let problem_name = PROBLEM_NAMES[problem_number - 1];

        let input = include_str!("../input/day24.txt");

        let start1 = Instant::now();
        let result1 = problems::$x::part1();
        let duration1 = start1.elapsed();

        let start2 = Instant::now();
        let result2 = problems::$x::part2();
        let duration2 = start2.elapsed();

        // println!("solutions: {result1} || {result2}");
        println!("| {problem_number} | [{problem_name}](https://adventofcode.com/2024/day/{problem_number}) | [Source](src/problems/{day}.rs) | {duration1:?} \\| {duration2:?} |");
    };
}

fn main() {
    run_day!(day24);
}
