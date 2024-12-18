use std::cmp::Reverse;
use std::collections::BinaryHeap;

// output depends only on bits that are less significant
// to customize the output values just start customizing a from the most signicant bits
// each 3 bits in a corresponds to an output
// for example to make sure the program's last output is 0,
// the most significant 3 bits of A should be 011 (in order)
// having this be fixed this won't affect any of the next outputs

// the following function is hand-compiled from my input:
// 0: B <- A & 7
// 1: B <- B ^ 5
// 2: C <- A >> B
// 3: B <- B ^ 6
// 4: A <- A >> 3
// 5: B <- B ^ C
// 6: OUT B & 7
// 7: IF A != 0: GOTO 0

// which is equivalent to:
// WHILE A != 0:
//    B <- ((A & 7) ^ 3) ^ (A >> ((A & 7) ^ 5))
//    A <- A >> 3
//    OUT: B & 8

fn compute_output(a: u64) -> Vec<u8> {
    let mut output = Vec::new();

    let mut a = a;
    let mut b;

    while a != 0 {
        b = ((a & 7) ^ 3) ^ (a >> ((a & 7) ^ 5));
        a >>= 3;

        output.push((b & 7) as u8);
    }

    output
}

#[allow(dead_code)]
pub fn part_one() -> String {
    let input: &str = include_str!("../../input/day17.txt");

    let a = input.lines().next().unwrap()[12..].parse().unwrap();

    let output = compute_output(a);
    let output_str: Vec<String> = output.into_iter().map(|digit| digit.to_string()).collect();

    output_str.join(",")
}

#[allow(dead_code)]
pub fn part_two() -> u64 {
    let mut candidates: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    let target = [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 1, 5, 5, 3, 0];

    for i in 1..8 {
        candidates.push(Reverse(i));
    }

    while let Some(Reverse(candidate)) = candidates.pop() {
        let output = compute_output(candidate);

        if output == target {
            return candidate;
        }

        let len = output.len();

        if output == target[target.len() - len..] {
            for i in 0..8 {
                candidates.push(Reverse((candidate << 3) + i));
            }
        }
    }

    panic!("no such value of A found")
}