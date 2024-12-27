use std::collections::HashSet;

struct Grid {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            width,
            height,
        }
    }

    fn get_guard_position(&self) -> (usize, usize) {
        for row in 0..self.width {
            for col in 0..self.height {
                if self.tiles[row][col] == '^' {
                    return (row, col);
                }
            }
        }
        panic!("guard no found in grid")
    }

    fn get_next_pos(
        &self,
        (guard_row, guard_col): (usize, usize),
        direction: &mut Direction,
    ) -> Option<(usize, usize)> {
        let (next_row, next_col) = match direction {
            Direction::Up => (guard_row.checked_sub(1)?, guard_col),
            Direction::Right => (guard_row, guard_col + 1),
            Direction::Down => (guard_row + 1, guard_col),
            Direction::Left => (guard_row, guard_col.checked_sub(1)?),
        };

        let char = self.tiles.get(next_row).and_then(|row| row.get(next_col))?;

        if char == &'#' {
            direction.turn_right();
            return Some((guard_row, guard_col));
        }

        Some((next_row, next_col))
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let (mut guard_row, mut guard_col) = grid.get_guard_position();
    let mut direction = Direction::Up; // UP

    let mut visited = HashSet::new();
    visited.insert((guard_row, guard_col));

    while let Some((next_row, next_col)) = grid.get_next_pos((guard_row, guard_col), &mut direction)
    {
        guard_row = next_row;
        guard_col = next_col;

        visited.insert((guard_row, guard_col));
    }

    visited.len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let (mut guard_row, mut guard_col) = grid.get_guard_position();
    let mut direction = Direction::Up;

    let mut visited = HashSet::new();
    let mut count = 0;

    while let Some((next_row, next_col)) = grid.get_next_pos((guard_row, guard_col), &mut direction)
    {
        visited.insert((guard_row, guard_col));

        if !visited.contains(&(next_row, next_col)) {
            grid.tiles[next_row][next_col] = '#';
            if gets_in_loop(&grid, (guard_row, guard_col), direction) {
                count += 1;
            }
            grid.tiles[next_row][next_col] = '.';
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    count
}

fn gets_in_loop(
    grid: &Grid,
    (start_row, start_col): (usize, usize),
    start_direction: Direction,
) -> bool {
    // only need to keep track of the times the guard turned
    // if the guard made the same turn at the same obstacle twice then we have a cycle
    let mut visited_obstacles: Vec<(usize, usize, Direction)> = Vec::new();

    let mut direction = start_direction;
    let (mut guard_row, mut guard_col) = (start_row, start_col);

    while let Some((next_row, next_col)) = grid.get_next_pos((guard_row, guard_col), &mut direction)
    {
        if (guard_row, guard_col) == (next_row, next_col) {
            if visited_obstacles.contains(&(guard_row, guard_col, direction)) {
                return true;
            }

            visited_obstacles.push((guard_row, guard_col, direction));
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    false
}
