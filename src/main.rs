#![warn(clippy::pedantic)]

use std::time::Instant;

mod problems;

fn main() {
    let start = Instant::now();

    let result = problems::day18::part_two();

    let duration = start.elapsed();

    println!("{result} in {duration:?}");
}
