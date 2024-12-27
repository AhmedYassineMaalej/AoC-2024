#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .filter(|report| is_monotone(report))
        .count()
}

fn parse_report(line: &str) -> Vec<u64> {
    line.split(' ').map(|n| n.parse().unwrap()).collect()
}

fn is_monotone(numbers: &[u64]) -> bool {
    if numbers.len() <= 1 {
        return true;
    }

    let ordering = numbers[0].cmp(&numbers[1]);

    for window in numbers.windows(2) {
        let curr = window[0];
        let next = window[1];

        if !(1..=3).contains(&curr.abs_diff(next)) {
            return false;
        }

        if curr.cmp(&next) != ordering {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .filter(|report| is_monotone_with_tolerance(report))
        .count()
}

fn is_monotone_with_tolerance(numbers: &[u64]) -> bool {
    // check if we remove the 1st element
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

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 4);
    }
}
