#![warn(clippy::pedantic)]

use std::time::Instant;
mod problems;

#[allow(unused)]
macro_rules! run {
    ($x: ident, 1) => {
        problems::$x::part_one()
    };
    ($x: ident, 2) => {
        problems::$x::part_two()
    };
}

fn main() {
    let start = Instant::now();

    let result = problems::day20::part_one();

    let duration = start.elapsed();

    println!("{result} in {duration:?}");
}
