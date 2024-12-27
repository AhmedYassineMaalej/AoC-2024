use itertools::Itertools;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (mut list1, mut list2) = parse_lists(input);

    list1.sort_unstable();
    list2.sort_unstable();

    list1.iter().zip(list2).map(|(x, y)| x.abs_diff(y)).sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (list1, list2) = parse_lists(input);

    let freqs = list2.into_iter().counts();

    list1
        .iter()
        .filter_map(|n| freqs.get(n).map(|freq| freq * n))
        .sum()
}

/// parses the input into two columns of integers
fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let (num1, num2) = line.split_once("   ").unwrap();
            (
                num1.parse::<usize>().unwrap(),
                num2.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 31);
    }
}
