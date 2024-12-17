use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, Sub},
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
    East,
    West,
    North,
    South,
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Moves {
    steps: usize,
    turns: usize,
}

impl Moves {
    fn cost(&self) -> usize {
        self.steps + self.turns * 1000
    }
}

impl PartialOrd for Moves {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Moves {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.turns.abs_diff(other.turns) <= 1 {
            return self.steps.cmp(&other.steps);
        }

        self.turns.cmp(&other.turns)
    }
}

#[derive(Debug)]
struct Node {
    position: (usize, usize),
    direction: Direction,
    moves: Moves,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.moves.eq(&other.moves)
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
        self.moves.cmp(&other.moves).reverse()
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
        let mut to_visit: BinaryHeap<Node> = BinaryHeap::new();

        to_visit.push(Node {
            position: self.start,
            direction: Direction::East,
            moves: Moves { steps: 0, turns: 0 },
        });

        while let Some(node) = to_visit.pop() {
            let Node {
                position,
                direction,
                moves,
            } = node;

            let cost = moves.cost();
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
                to_visit.push(Node {
                    position: (next_row, next_col),
                    direction,
                    moves: Moves {
                        steps: moves.steps + 1,
                        turns: moves.turns,
                    },
                });
            }

            let (right_row, right_col) = position + direction.turn_right();

            if self.tiles[right_row][right_col] != Tile::Wall {
                to_visit.push(Node {
                    position: (right_row, right_col),
                    direction: direction.turn_right(),
                    moves: Moves {
                        steps: moves.steps + 1,
                        turns: moves.turns + 1,
                    },
                });
            }

            let (left_row, left_col) = position + direction.turn_left();

            if self.tiles[left_row][left_col] != Tile::Wall {
                to_visit.push(Node {
                    position: (left_row, left_col),
                    direction: direction.turn_left(),
                    moves: Moves {
                        steps: moves.steps + 1,
                        turns: moves.turns + 1,
                    },
                });
            }
        }

        None
    }

    fn get_min_path_tiles(&self) -> Option<usize> {
        let mut moves_to_visit: Vec<Vec<Moves>> = vec![
            vec![
                Moves {
                    steps: 100_000,
                    turns: 100_000,
                };
                self.width
            ];
            self.height
        ];
        let mut to_visit: BinaryHeap<Node> = BinaryHeap::new();
        let mut prev: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        to_visit.push(Node {
            position: self.start,
            direction: Direction::East,
            moves: Moves { steps: 0, turns: 0 },
        });

        while let Some(node) = to_visit.pop() {
            let Node {
                position,
                direction,
                moves,
            } = node;

            if position == self.end {
                // reached target
                prev.entry(self.end).or_default().push(position - direction);
                println!("{}", self.count_tiles(&prev));
                return Some(moves.cost());
            }

            let (row, col) = position;
            // already found a better way
            if moves > moves_to_visit[row][col] {
                continue;
            }

            moves_to_visit[row][col] = moves;

            prev.entry(position).or_default().push(position - direction);

            let (next_row, next_col) = position + direction;

            if self.tiles[next_row][next_col] != Tile::Wall {
                to_visit.push(Node {
                    position: (next_row, next_col),
                    direction,
                    moves: Moves {
                        steps: moves.steps + 1,
                        turns: moves.turns,
                    },
                });
            }

            let (right_row, right_col) = position + direction.turn_right();

            if self.tiles[right_row][right_col] != Tile::Wall {
                to_visit.push(Node {
                    position: (right_row, right_col),
                    direction: direction.turn_right(),
                    moves: Moves {
                        steps: moves.steps + 1,
                        turns: moves.turns + 1,
                    },
                });
            }

            let (left_row, left_col) = position + direction.turn_left();

            if self.tiles[left_row][left_col] != Tile::Wall {
                to_visit.push(Node {
                    position: (left_row, left_col),
                    direction: direction.turn_left(),
                    moves: Moves {
                        steps: moves.steps + 1,
                        turns: moves.turns + 1,
                    },
                });
            }
        }

        None
    }

    fn count_tiles(&self, prev: &HashMap<(usize, usize), Vec<(usize, usize)>>) -> usize {
        let mut tiles = HashSet::new();
        tiles.insert(self.start);

        let mut to_add = Vec::new();
        to_add.push(self.end);

        while let Some(position) = to_add.pop() {
            if !tiles.insert(position) {
                continue;
            }

            if let Some(prev_positions) = prev.get(&position) {
                to_add.extend(prev_positions);
            }
        }

        tiles.len()
    }
}

pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day16.txt");

    let grid = Grid::parse(input);

    grid.min_path_cost().unwrap()
}

pub fn part_two() -> usize {
    let input: &str = include_str!("../../input/day16.txt");

    let grid = Grid::parse(input);

    grid.get_min_path_tiles().unwrap()
}

#[test]
fn test() {
    let x = Moves {
        steps: 10,
        turns: 1,
    };

    let y = Moves {
        steps: 5,
        turns: 0,
    };

    println!("{}", x > y);
}
