use itertools::Itertools;
use std::ops::AddAssign;

struct Dimension {
    usize: usize,
    i64: i64,
}

const WIDTH: Dimension = Dimension {
    usize: 101,
    i64: 101,
};

const HEIGHT: Dimension = Dimension {
    usize: 103,
    i64: 103,
};

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl From<&str> for Vec2 {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = (self.x + rhs.x).rem_euclid(WIDTH.i64);
        self.y = (self.y + rhs.y).rem_euclid(HEIGHT.i64);
    }
}

struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (position, velocity) = value.split_once(' ').unwrap();
        Self {
            position: Vec2::from(position.strip_prefix("p=").unwrap()),
            velocity: Vec2::from(velocity.strip_prefix("v=").unwrap()),
        }
    }
}

impl Robot {
    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn get_quadrant(&self) -> Option<usize> {
        let mid_x = WIDTH.i64 / 2;
        let mid_y = HEIGHT.i64 / 2;

        if self.position.x > mid_x && self.position.y > mid_y {
            return Some(0);
        }

        if self.position.x > mid_x && self.position.y < mid_y {
            return Some(1);
        }

        if self.position.x < mid_x && self.position.y > mid_y {
            return Some(2);
        }

        if self.position.x < mid_x && self.position.y < mid_y {
            return Some(3);
        }

        None
    }
}

fn calculate_safety_factor(robots: &[Robot]) -> usize {
    let mut quadrants = [0; 4];

    for robot in robots {
        if let Some(quadrant_idx) = robot.get_quadrant() {
            quadrants[quadrant_idx] += 1;
        }
    }

    quadrants.iter().product()
}

fn contains_block(grid: &[Vec<u8>]) -> bool {
    for row in 0..(HEIGHT.usize - 3) {
        for col in 0..(WIDTH.usize - 3) {
            if is_block(grid, row, col) {
                return true;
            }
        }
    }

    false
}

fn is_block(grid: &[Vec<u8>], row: usize, col: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if grid[row + i][col + j] == 0 {
                return false;
            }
        }
    }

    true
}

fn form_grid(robots: &[Robot]) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; WIDTH.usize]; HEIGHT.usize];

    for robot in robots {
        let row = usize::try_from(robot.position.y).unwrap();
        let col = usize::try_from(robot.position.x).unwrap();
        grid[row][col] += 1;
    }

    grid
}

fn grid_to_string(grid: Vec<Vec<u8>>) -> String {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|tile| {
                    if tile == 0 {
                        '.'
                    } else {
                        (tile + b'0') as char
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut robots = Vec::new();

    for line in input.lines() {
        robots.push(Robot::from(line));
    }

    for _ in 0..100 {
        for robot in &mut robots {
            robot.update();
        }
    }

    calculate_safety_factor(&robots)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> String {
    let mut robots = Vec::new();

    for line in input.lines() {
        robots.push(Robot::from(line));
    }

    let mut second = 1;
    loop {
        for robot in &mut robots {
            robot.update();
        }

        let grid = form_grid(&robots);
        if contains_block(&grid) {
            let mut result = format!("grid after {second} seconds\n");
            result.push_str(&grid_to_string(grid));
            result.push('\n');

            return result;
        }

        second += 1;
    }
}
