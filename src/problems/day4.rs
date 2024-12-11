#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day4.txt");

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let target = "XMAS";
    let target_rev = "SAMX";
    let mut count = 0;

    // count horizontal
    for line in &lines {
        for i in 0..(line.len() - 3) {
            if &line[i..(i + 4)] == target || &line[i..(i + 4)] == target_rev {
                count += 1;
            }
        }
    }

    // count_vertical
    for row in 0..(height - 3) {
        for col in 0..width {
            let mut vertical_str = String::new();

            for i in 0..4 {
                let char = lines[row + i].chars().nth(col).unwrap();
                vertical_str.push(char);
            }

            if vertical_str == target || vertical_str == target_rev {
                count += 1;
            }
        }
    }

    // count diagonal down right
    for row in 0..(height - 3) {
        for col in 0..(width - 3) {
            let mut vertical_str = String::new();

            for i in 0..4 {
                let char = lines[row + i].chars().nth(col + i).unwrap();
                vertical_str.push(char);
            }

            if vertical_str == target || vertical_str == target_rev {
                count += 1;
            }
        }
    }

    // count diagonals down left
    for row in 0..(height - 3) {
        for col in 3..width {
            let mut vertical_str = String::new();

            for i in 0..4 {
                let char = lines[row + i].chars().nth(col - i).unwrap();
                vertical_str.push(char);
            }

            if vertical_str == target || vertical_str == target_rev {
                count += 1;
            }
        }
    }

    count
}

#[allow(dead_code)]
pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day4.txt");

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let target = "MAS";
    let target_rev = "SAM";
    let mut count = 0;

    for row in 0..(height - 2) {
        for col in 0..(width - 2) {
            let mut diagonal1 = String::new();
            let mut diagonal2 = String::new();

            for i in 0..3 {
                let char = lines[row + i].chars().nth(col + i).unwrap();
                diagonal1.push(char);

                let char = lines[row + i].chars().nth(col + 2 - i).unwrap();
                diagonal2.push(char);
            }

            if (diagonal1 == target || diagonal1 == target_rev)
                && (diagonal2 == target || diagonal2 == target_rev)
            {
                count += 1;
            }
        }
    }

    count
}
