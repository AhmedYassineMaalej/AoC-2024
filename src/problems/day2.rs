fn is_monotone(numbers: &[u64]) -> bool {
    if numbers.len() <= 1 {
        return true;
    }

    let order = numbers[0].cmp(&numbers[1]);

    for window in numbers.windows(2) {
        let curr = window.first().unwrap();
        let next = window.get(1).unwrap();

        if !(1..=3).contains(&curr.abs_diff(*next)) {
            return false;
        }

        if curr.cmp(next) != order {
            return false;
        }
    }

    true
}

fn is_safe_with_tolerance(numbers: &[u64]) -> bool {
    if is_monotone(&numbers[1..]) {
        return true;
    }

    // suppose the 1st and 3rd are increasing but the array is decreasing
    // if the 1st element is wrong then the case is already covered with the guard clause
    // so the 3rd element is wrong
    // prev = numbers[0] < numbers[2] = next: miss 1
    // prev = numbers[2] > numbers[0] > numbers[1] > numbers[3] = curr: miss 2
    let order = numbers[0].cmp(&numbers[2]);

    let mut misses = 0;
    let mut prev = numbers[0];

    for &curr in numbers.iter().skip(1) {
        if curr == prev || curr.abs_diff(prev) > 3 {
            misses += 1;
            continue;
        }

        if prev.cmp(&curr) != order {
            misses += 1;
            continue;
        }

        prev = curr;
    }

    misses <= 1
}

#[allow(unused)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day2.txt");

    let mut reports: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        let numbers = line.split(' ').map(|n| n.parse().unwrap()).collect();

        reports.push(numbers);
    }

    reports
        .into_iter()
        .filter(|numbers| is_monotone(numbers))
        .count()
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day2.txt");

    let mut count = 0;
    for line in input.lines() {
        let numbers: Vec<u64> = line.split(' ').map(|n| n.parse().unwrap()).collect();

        if is_safe_with_tolerance(&numbers) {
            count += 1;
        }
    }

    count
}
