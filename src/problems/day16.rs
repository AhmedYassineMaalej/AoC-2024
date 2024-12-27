use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    ops::{Add, Index, IndexMut, Sub},
};

#[derive(PartialEq, Debug)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            'E' => Self::End,
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Direction {
    East = 0,
    West = 1,
    North = 2,
    South = 3,
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::East => (row, col + 1),
            Direction::West => (row, col - 1),
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
        }
    }
}

impl Sub<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn sub(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::East => (row, col - 1),
            Direction::West => (row, col + 1),
            Direction::North => (row + 1, col),
            Direction::South => (row - 1, col),
        }
    }
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
        }
    }
}

#[derive(Debug)]
struct Node {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Index<Direction> for [bool; 4] {
    type Output = bool;

    fn index(&self, index: Direction) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Direction> for [bool; 4] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        let height = tiles.len();
        let width = tiles[0].len();
        let start = (height - 2, 1);
        let end = (1, width - 2);

        Self {
            tiles,
            width,
            height,
            start,
            end,
        }
    }

    fn min_path_cost(&self) -> Option<usize> {
        let mut moves_to_visit = vec![vec![usize::MAX; self.width]; self.height];
        let mut to_visit: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

        to_visit.push(Reverse(Node {
            position: self.start,
            direction: Direction::East,
            cost: 0,
        }));

        while let Some(Reverse(node)) = to_visit.pop() {
            let Node {
                position,
                direction,
                cost,
            } = node;

            let (row, col) = position;

            if position == self.end {
                // reached target
                return Some(cost);
            }

            // already found a better way
            if cost > moves_to_visit[row][col] {
                continue;
            }

            moves_to_visit[row][col] = cost;

            let (next_row, next_col) = position + direction;

            if self.tiles[next_row][next_col] != Tile::Wall {
                to_visit.push(Reverse(Node {
                    position: (next_row, next_col),
                    direction,
                    cost: cost + 1,
                }));
            }

            let (right_row, right_col) = position + direction.turn_right();

            if self.tiles[right_row][right_col] != Tile::Wall {
                to_visit.push(Reverse(Node {
                    position: (right_row, right_col),
                    direction: direction.turn_right(),
                    cost: cost + 1001,
                }));
            }

            let (left_row, left_col) = position + direction.turn_left();

            if self.tiles[left_row][left_col] != Tile::Wall {
                to_visit.push(Reverse(Node {
                    position: (left_row, left_col),
                    direction: direction.turn_left(),
                    cost: cost + 1001,
                }));
            }
        }

        None
    }

    fn get_min_path_tiles(&self) -> Option<usize> {
        let mut cost_to_visit: Vec<Vec<usize>> = vec![vec![usize::MAX; self.width]; self.height];

        // Reverse<T> so it's a min-heap (the top element is always the one with least cost)
        let mut to_visit: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

        let mut prev: Vec<Vec<[bool; 4]>> = vec![vec![[false; 4]; self.width]; self.height];

        to_visit.push(Reverse(Node {
            position: self.start,
            direction: Direction::East,
            cost: 0,
        }));

        while let Some(node) = to_visit.pop() {
            let Reverse(Node {
                position,
                direction,
                cost,
            }) = node;

            let (row, col) = position;
            if position == self.end {
                // reached target
                prev[row][col][direction] = true;
                return Some(self.count_tiles(&prev));
            }

            // already found a better way
            let (turns, steps) = (cost / 1000, cost % 1000);

            let min = cost_to_visit[row][col];
            let (min_turns, min_steps) = (min / 1000, min % 1000);

            if turns > min_turns + 1 {
                continue;
            }

            if steps > min_steps {
                continue;
            }

            if steps < min_steps {
                prev[row][col].fill(false);
            }

            cost_to_visit[row][col] = cost;

            prev[row][col][direction] = true;

            let (next_row, next_col) = position + direction;

            if self.tiles[next_row][next_col] != Tile::Wall {
                to_visit.push(Reverse(Node {
                    position: (next_row, next_col),
                    direction,
                    cost: cost + 1,
                }));
            }

            let (right_row, right_col) = position + direction.turn_right();

            if self.tiles[right_row][right_col] != Tile::Wall {
                to_visit.push(Reverse(Node {
                    position: (right_row, right_col),
                    direction: direction.turn_right(),
                    cost: cost + 1001,
                }));
            }

            let (left_row, left_col) = position + direction.turn_left();

            if self.tiles[left_row][left_col] != Tile::Wall {
                to_visit.push(Reverse(Node {
                    position: (left_row, left_col),
                    direction: direction.turn_left(),
                    cost: cost + 1001,
                }));
            }
        }

        None
    }

    fn count_tiles(&self, prev: &[Vec<[bool; 4]>]) -> usize {
        let mut tiles = HashSet::new();
        tiles.insert(self.start);

        let mut to_add = Vec::new();
        to_add.push(self.end);

        while let Some((row, col)) = to_add.pop() {
            if !tiles.insert((row, col)) {
                continue;
            }

            let directions = prev[row][col];

            if directions[Direction::East] {
                to_add.push((row, col) - Direction::East);
            }

            if directions[Direction::West] {
                to_add.push((row, col) - Direction::West);
            }

            if directions[Direction::North] {
                to_add.push((row, col) - Direction::North);
            }

            if directions[Direction::South] {
                to_add.push((row, col) - Direction::South);
            }
        }

        tiles.len()
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);

    grid.min_path_cost().unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);

    grid.get_min_path_tiles().unwrap()
}
