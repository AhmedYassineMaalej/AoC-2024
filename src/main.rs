#![warn(clippy::pedantic)]

use std::time::Instant;

mod problems;

fn main() {
    let start = Instant::now();

    let result = problems::day15::part_one();

    let duration = start.elapsed();

    println!("{result} in {duration:?}");
}
