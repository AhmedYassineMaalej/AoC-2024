use regex::Regex;

#[allow(dead_code)]
pub fn part_one() -> u64 {
    let input: &str = include_str!("../../input/day3.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|cap| cap.extract()) {
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();
        sum += a * b;
    }

    sum
}

#[allow(unused)]
pub fn part_two() -> u64 {
    let input: &str = include_str!("../../input/day3.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut add_up = true;
    for m in re.captures_iter(input) {
        let instruction = m.get(0).unwrap().as_str();

        match instruction {
            "do()" => add_up = true,
            "don't()" => add_up = false,
            _ => {
                if add_up {
                    let a: u64 = m.get(1).unwrap().as_str().parse().unwrap();
                    let b: u64 = m.get(2).unwrap().as_str().parse().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    sum
}
