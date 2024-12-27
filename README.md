## Advent of Code 2024

My personal solutions to [Advent of Code 2024](https://adventofcode.com/2024) in Rust.

## Notes
Libraries used include the [itertools](https://docs.rs/itertools/latest/itertools/) library as well as [regex](https://docs.rs/regex/latest/regex/)

Tried to make my code as readable as possible without taking a huge performance hit. It is a balance between speed and clarity.

Parts 1 and 2 are solved seperately.

I skipped day 21 (skill issue)

Performance does not include reading the input as I include it in the source code using the [`!include_str()`](https://doc.rust-lang.org/std/macro.include_str.html) macro

## Performace
| Day | Problem | Solution | Benchmark(part1\|part2) |
| :---: | :------- | :--------: | :---------: |
| 1 | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [Source](src/problems/day1.rs) | 90µs \| 145µs |
| 2 | [Red-Nosed Reports](https://adventofcode.com/2024/day/2) | [Source](src/problems/day2.rs) | 300µs \| 800µs |
| 3 | [Mull It Over](https://adventofcode.com/2024/day/3) | [Source](src/problems/day3.rs) | 1.12ms \| 1.13ms |
| 4 | [Ceres Search](https://adventofcode.com/2024/day/4) | [Source](src/problems/day4.rs) | 11.7ms \| 6.40ms |
| 5 | [Print Queue](https://adventofcode.com/2024/day/5) | [Source](src/problems/day5.rs) | 460µs \| 500µs |
| 6 | [Guard Gallivant](https://adventofcode.com/2024/day/6) | [Source](src/problems/day6.rs) | 430µs \| 20ms |
| 7 | [Bridge Repair](https://adventofcode.com/2024/day/7) | [Source](src/problems/day7.rs) | 1.03ms \| 17.1ms |
| 8 | [Resonant Collinearity](https://adventofcode.com/2024/day/8) | [Source](src/problems/day8.rs) | 88µs \| 182µs |
| 9 | [Disk Fragmenter](https://adventofcode.com/2024/day/9) | [Source](src/problems/day9.rs) | 18.5ms \| 77.4ms |
| 10 | [Hoof It](https://adventofcode.com/2024/day/10) | [Source](src/problems/day10.rs) | 152µs \| 108µs |
| 11 | [Plutonian Pebbles](https://adventofcode.com/2024/day/11) | [Source](src/problems/day11.rs) | 178µs \| 5.35ms |
| 12 | [Garden Groups](https://adventofcode.com/2024/day/12) | [Source](src/problems/day12.rs) | 660µs \| 2.50ms |
| 13 | [Claw Contraption](https://adventofcode.com/2024/day/13) | [Source](src/problems/day13.rs) | 56µs \| 46µs |
| 14 | [Restroom Redoubt](https://adventofcode.com/2024/day/14) | [Source](src/problems/day14.rs) | 264µs \| 140ms |
| 15 | [Warehouse Woes](https://adventofcode.com/2024/day/15) | [Source](src/problems/day15.rs) | 852µs \| 690µs |
| 16 | [Reindeer Maze](https://adventofcode.com/2024/day/16) | [Source](src/problems/day16.rs) | 518µs \| 7.81ms |
| 17 | [Chronospatial Computer](https://adventofcode.com/2024/day/17) | [Source](src/problems/day17.rs) | 10µs \| 2.10ms |
| 18 | [RAM Run](https://adventofcode.com/2024/day/18) | [Source](src/problems/day18.rs) | 190µs \| 515µs |
| 19 | [Linen Layout](https://adventofcode.com/2024/day/19) | [Source](src/problems/day19.rs) | 2.63ms \| 10.4ms |
| 20 | [Race Condition](https://adventofcode.com/2024/day/20) | [Source](src/problems/day20.rs) | 116ms \| 105ms |
| 22 | [Monkey Market](https://adventofcode.com/2024/day/22) | [Source](src/problems/day22.rs) | 10ms \| 825ms |
| 23 | [LAN Party](https://adventofcode.com/2024/day/23) | [Source](src/problems/day23.rs) | 6.34ms \| 3.22ms |
| 24 | [Crossed Wires](https://adventofcode.com/2024/day/24) | [Source](src/problems/day24.rs) | 723µs \| 4µs* |
| 25 | [Code Chronicle](https://adventofcode.com/2024/day/25) | [Source](src/problems/day25.rs) | 920µs |

*: I solved day 24 part 2 using pen and paper and just hard-coded the solutions
