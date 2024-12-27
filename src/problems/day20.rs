#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Empty,
            '#' => Self::Wall,
            'E' => Self::End,
            c => panic!("cannot create tile from {c}"),
        }
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
    path: Vec<(usize, usize)>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = value
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        let height = tiles.len();
        let width = tiles[0].len();

        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, tile_row) in tiles.iter().enumerate() {
            for (col, tile) in tile_row.iter().enumerate() {
                if let Tile::Start = tile {
                    start = (row, col);
                } else if let Tile::End = tile {
                    end = (row, col);
                }
            }
        }

        let mut grid = Self {
            tiles,
            start,
            end,
            width,
            height,
            path: Vec::new(),
        };

        grid.build_path();
        grid
    }
}

impl Grid {
    fn build_path(&mut self) {
        let mut current = self.start;

        loop {
            self.path.push(current);

            if current == self.end {
                break;
            }

            current = self.next_tile(current);
        }
    }

    fn next_tile(&self, position: (usize, usize)) -> (usize, usize) {
        let (row, col) = position;

        let mut neighbors = Vec::new();
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        if row + 1 < self.height {
            neighbors.push((row + 1, col));
        }
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        if col + 1 < self.width {
            neighbors.push((row, col + 1));
        }

        for neighbor_position in neighbors {
            if self.is_empty(neighbor_position) && !self.is_visited(neighbor_position) {
                return neighbor_position;
            }
        }

        panic!("no neighbour of {position:?} found")
    }

    fn is_empty(&self, position: (usize, usize)) -> bool {
        self.tiles[position.0][position.1] == Tile::Empty
            || self.tiles[position.0][position.1] == Tile::End
    }

    fn is_visited(&self, position: (usize, usize)) -> bool {
        self.path.contains(&position)
    }

    fn count_cheats(&self, max_distance: usize) -> usize {
        let mut count = 0;

        for (start_distance, cheat_start) in self.path.iter().enumerate() {
            for (end_distance, cheat_end) in self.path.iter().enumerate().skip(start_distance + 100)
            {
                if start_distance > end_distance {
                    continue;
                }

                let cheat_distance = manhattan_distance(cheat_start, cheat_end);

                if cheat_distance > max_distance {
                    continue;
                }

                if end_distance - start_distance >= cheat_distance + 100 {
                    count += 1;
                }
            }
        }
        count
    }
}

fn manhattan_distance(start: &(usize, usize), end: &(usize, usize)) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.count_cheats(2)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let grid = Grid::from(input);

    grid.count_cheats(20)
}
