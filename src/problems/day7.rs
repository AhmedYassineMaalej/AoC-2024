#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day7.txt");

    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in input.lines() {
        let (result, numbers) = line.split_once(':').unwrap();
        let result = result.parse().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push((result, numbers));
    }

    let mut result = 0;
    for (target, numbers) in equations {
        let (start, numbers) = numbers.split_first().unwrap();
        if can_reach(*start, target, numbers) {
            result += target;
        }
    }

    result
}

fn can_reach(start: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return start == target;
    }

    if start > target {
        return false;
    }

    let (first, rest) = numbers.split_first().unwrap();
    can_reach(start * first, target, rest) || can_reach(start + first, target, rest)
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day7.txt");

    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in input.lines() {
        let (result, numbers) = line.split_once(':').unwrap();
        let result = result.parse().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push((result, numbers));
    }

    let mut result = 0;
    for (target, numbers) in equations {
        let (start, numbers) = numbers.split_first().unwrap();
        if can_reach_concat(*start, target, numbers) {
            result += target;
        }
    }

    result
}

fn can_reach_concat(start: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return start == target;
    }

    if start > target {
        return false;
    }

    let (first, rest) = numbers.split_first().unwrap();
    can_reach_concat(start * first, target, rest)
        || can_reach_concat(start + first, target, rest)
        || can_reach_concat(concat(start, *first), target, rest)
}

fn concat(a: usize, b: usize) -> usize {
    let mut offset = 1;

    while offset <= b {
        offset *= 10;
    }

    
    a * offset + b
}