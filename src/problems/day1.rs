use itertools::Itertools;

#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day1.txt");

    let pairs = input.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: usize = num1.parse().unwrap();
        let num2: usize = num2.parse().unwrap();

        (num1, num2)
    });

    let (mut list1, mut list2): (Vec<_>, Vec<_>) = pairs.unzip();

    list1.sort_unstable();
    list2.sort_unstable();

    let mut result = 0;
    for (n1, n2) in list1.into_iter().zip(list2.into_iter()) {
        result += n1.abs_diff(n2);
    }

    result
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input = include_str!("../../input/day1.txt");

    let pairs = input.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: usize = num1.parse().unwrap();
        let num2: usize = num2.parse().unwrap();

        (num1, num2)
    });

    let (list1, list2): (Vec<_>, Vec<_>) = pairs.unzip();

    let counts = list2.into_iter().counts();

    let result = list1
        .into_iter()
        .map(|n| counts.get(&n).unwrap_or(&0) * n)
        .sum();

    result
}
