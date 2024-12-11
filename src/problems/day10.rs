#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day10.txt");

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|char| char as u8 - b'0').collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    let mut result = 0;
    for row in 0..height {
        for col in 0..width {
            if map[row][col] == 9 {
                let mut visited: Vec<(usize, usize)> = Vec::new();
                result += get_trailheads((row, col), 9, &map, &mut visited);
            }
        }
    }

    result
}

fn get_trailheads(
    (row, col): (usize, usize),
    height: u8,
    map: &Vec<Vec<u8>>,
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    if height == 0 && !visited.contains(&(row, col)) {
        visited.push((row, col));
        return 1;
    }

    let mut result = 0;

    if row > 0 && map[row - 1][col] == height - 1 {
        result += get_trailheads((row - 1, col), height - 1, map, visited);
    }

    if row + 1 < map.len() && map[row + 1][col] == height - 1 {
        result += get_trailheads((row + 1, col), height - 1, map, visited);
    }

    if col > 0 && map[row][col - 1] == height - 1 {
        result += get_trailheads((row, col - 1), height - 1, map, visited);
    }

    if col + 1 < map[0].len() && map[row][col + 1] == height - 1 {
        result += get_trailheads((row, col + 1), height - 1, map, visited);
    }

    result
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day10.txt");

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|char| char as u8 - b'0').collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    let mut result = 0;
    for row in 0..height {
        for col in 0..width {
            if map[row][col] == 9 {
                result += count_paths((row, col), 9, &map);
            }
        }
    }

    result
}

fn count_paths((row, col): (usize, usize), height: u8, map: &Vec<Vec<u8>>) -> usize {
    if height == 0 {
        return 1;
    }

    let mut result = 0;

    if row > 0 && map[row - 1][col] == height - 1 {
        result += count_paths((row - 1, col), height - 1, map);
    }

    if row + 1 < map.len() && map[row + 1][col] == height - 1 {
        result += count_paths((row + 1, col), height - 1, map);
    }

    if col > 0 && map[row][col - 1] == height - 1 {
        result += count_paths((row, col - 1), height - 1, map);
    }

    if col + 1 < map[0].len() && map[row][col + 1] == height - 1 {
        result += count_paths((row, col + 1), height - 1, map);
    }

    result
}
