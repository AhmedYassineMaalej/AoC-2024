use std::fmt::Display;
use std::mem;
use std::ops::Add;

use itertools::Itertools;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Empty,
    Wall,
    Object,
    Robot,
    BoxLeft,
    BoxRight,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Object,
            '@' => Self::Robot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => Self::Empty,
        }
    }
}

impl Tile {
    fn double(self) -> impl Iterator<Item = Self> {
        match self {
            Tile::Empty => [Tile::Empty, Tile::Empty].into_iter(),
            Tile::Wall => [Tile::Wall, Tile::Wall].into_iter(),
            Tile::Object => [Tile::BoxLeft, Tile::BoxRight].into_iter(),
            Tile::Robot => [Tile::Robot, Tile::Empty].into_iter(),
            Tile::BoxLeft | Tile::BoxRight => panic!(),
        }
    }
}

impl From<Tile> for char {
    fn from(val: Tile) -> Self {
        match val {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Object => 'O',
            Tile::Robot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        }
    }
}

struct Warehouse {
    grid: Vec<Vec<Tile>>,
    robot: (usize, usize),
    width: usize,
    height: usize,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        let mut robot = (0, 0);

        for (row, line) in input.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                let tile = Tile::from(char);

                if tile == Tile::Robot {
                    robot = (row, col);
                }

                grid_row.push(tile);
            }

            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            robot,
            width,
            height,
        }
    }

    fn scale_width(&mut self) {
        let mut temp: Vec<Vec<Tile>> = Vec::new();
        mem::swap(&mut self.grid, &mut temp);

        self.grid = temp
            .into_iter()
            .map(|row| row.into_iter().flat_map(Tile::double).collect())
            .collect();

        self.width *= 2;
        self.robot.1 *= 2;
    }

    fn move_robot(&mut self, direction: Direction) {
        let (row, col) = self.robot;

        if self.can_move_tile(row, col, direction) {
            self.move_tile(row, col, direction);
            self.robot = self.robot + direction;
        }
    }

    fn move_tile(&mut self, row: usize, col: usize, direction: Direction) {
        let (next_row, next_col) = (row, col) + direction;
        let next_tile = self.grid[next_row][next_col];

        match next_tile {
            Tile::Empty => {
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::Object => {
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::BoxRight => {
                self.move_tile(next_row, next_col - 1, direction);
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::BoxLeft => {
                self.move_tile(next_row, next_col + 1, direction);
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::Wall => panic!(),
            Tile::Robot => panic!(),
        }
    }

    fn can_move_tile(&self, row: usize, col: usize, direction: Direction) -> bool {
        let (next_row, next_col) = (row, col) + direction;
        let next_tile = self.grid[next_row][next_col];

        match next_tile {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Object => self.can_move_tile(next_row, next_col, direction),
            Tile::BoxLeft => {
                if direction == Direction::Left {
                    self.can_move_tile(next_row, next_col, direction)
                } else if direction == Direction::Right {
                    self.can_move_tile(next_row, next_col + 1, direction)
                } else {
                    self.can_move_tile(next_row, next_col + 1, direction)
                        && self.can_move_tile(next_row, next_col, direction)
                }
            }
            Tile::BoxRight => {
                if direction == Direction::Right {
                    self.can_move_tile(next_row, next_col, direction)
                } else if direction == Direction::Left {
                    self.can_move_tile(next_row, next_col - 1, direction)
                } else {
                    self.can_move_tile(next_row, next_col - 1, direction)
                        && self.can_move_tile(next_row, next_col, direction)
                }
            }
            Tile::Robot => panic!(),
        }
    }

    fn gps_coordinate(row: usize, col: usize) -> usize {
        row * 100 + col
    }

    fn sum_gps_coordinates(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == Tile::Object || self.grid[row][col] == Tile::BoxLeft {
                    sum += Self::gps_coordinate(row, col);
                }
            }
        }

        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => panic!(),
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| row.iter().map(|&tile| char::from(tile)).collect::<String>())
            .join("\n");

        writeln!(f, "{string}")
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (warehouse, moves) = input.split_once("\r\n\r\n").unwrap();

    let mut warehouse = Warehouse::new(warehouse);
    let moves: Vec<Direction> = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    for direction in moves {
        warehouse.move_robot(direction);
    }

    warehouse.sum_gps_coordinates()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (warehouse, moves) = input.split_once("\r\n\r\n").unwrap();

    let mut warehouse = Warehouse::new(warehouse);
    let moves: Vec<Direction> = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    warehouse.scale_width();

    for direction in moves {
        warehouse.move_robot(direction);
    }

    warehouse.sum_gps_coordinates()
}
