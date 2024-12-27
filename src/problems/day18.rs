use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Free,
    Byte,
}
struct Grid {
    tiles: Vec<Vec<Space>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(height: usize, width: usize) -> Self {
        let tiles = vec![vec![Space::Free; width]; height];

        Self {
            tiles,
            height,
            width,
        }
    }

    fn corrupt_space(&mut self, row: usize, col: usize) {
        self.tiles[row][col] = Space::Byte;
    }

    fn shortest_path(&self) -> usize {
        let start = (0, 0);
        let exit = (self.height - 1, self.width - 1);

        // (row, col, dist)
        let mut to_visit: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];

        to_visit.push_back((start.0, start.1, 0));

        while let Some((row, col, dist)) = to_visit.pop_front() {
            if (row, col) == exit {
                return dist;
            }

            if visited[row][col] {
                continue;
            }

            visited[row][col] = true;

            if row > 0 && self.tiles[row - 1][col] == Space::Free {
                to_visit.push_back((row - 1, col, dist + 1));
            }

            if row + 1 < self.height && self.tiles[row + 1][col] == Space::Free {
                to_visit.push_back((row + 1, col, dist + 1));
            }

            if col > 0 && self.tiles[row][col - 1] == Space::Free {
                to_visit.push_back((row, col - 1, dist + 1));
            }
            if col + 1 < self.height && self.tiles[row][col + 1] == Space::Free {
                to_visit.push_back((row, col + 1, dist + 1));
            }
        }

        usize::MAX
    }

    fn path_exists(&self) -> bool {
        let start = (0, 0);
        let exit = (self.height - 1, self.width - 1);

        // (row, col, dist)
        let mut to_visit: Vec<(usize, usize)> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];

        to_visit.push((start.0, start.1));

        while let Some((row, col)) = to_visit.pop() {
            if (row, col) == exit {
                return true;
            }

            if visited[row][col] {
                continue;
            }

            visited[row][col] = true;

            if row > 0 && self.tiles[row - 1][col] == Space::Free {
                to_visit.push((row - 1, col));
            }

            if row + 1 < self.height && self.tiles[row + 1][col] == Space::Free {
                to_visit.push((row + 1, col));
            }

            if col > 0 && self.tiles[row][col - 1] == Space::Free {
                to_visit.push((row, col - 1));
            }
            if col + 1 < self.height && self.tiles[row][col + 1] == Space::Free {
                to_visit.push((row, col + 1));
            }
        }

        false
    }

    fn clear_bytes(&mut self) {
        for row in &mut self.tiles {
            row.fill(Space::Free);
        }
    }

    fn corrupt_spaces(&mut self, spaces: &[(usize, usize)]) {
        for &(row, col) in spaces {
            self.corrupt_space(row, col);
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut grid = Grid::new(71, 71);

    for byte in input.lines().take(1024) {
        let (row, col) = byte.split_once(',').unwrap();

        let row = row.parse().unwrap();
        let col = col.parse().unwrap();

        grid.corrupt_space(row, col);
    }

    grid.shortest_path()
}

#[allow(clippy::similar_names, dead_code)]
pub fn part2(input: &str) -> String {
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (row, col) = line.split_once(',').unwrap();
            (row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
        })
        .collect();

    let mut grid = Grid::new(71, 71);

    let mut max = bytes.len() - 1;
    let mut min = 0;

    while min < max {
        let mid = (min + max) / 2;

        grid.corrupt_spaces(&bytes[..=mid]);

        if grid.path_exists() {
            min = mid + 1;
        } else {
            max = mid;
        }

        grid.clear_bytes();
    }

    let (x, y) = bytes[max];
    format!("{x},{y}")
}
