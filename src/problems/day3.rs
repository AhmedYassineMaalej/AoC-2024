use regex::Regex;

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|capture| {
            let (_, [a, b]) = capture.extract();
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();
            a * b
        })
        .sum()
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut add_up = true;
    for m in re.captures_iter(input) {
        let instruction = m.get(0).unwrap().as_str();

        if instruction == "do()" {
            add_up = true;
        } else if instruction == "don't()" {
            add_up = false;
        } else if add_up {
            let a: u64 = m.get(1).unwrap().as_str().parse().unwrap();
            let b: u64 = m.get(2).unwrap().as_str().parse().unwrap();
            sum += a * b;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT2), 48);
    }
}
