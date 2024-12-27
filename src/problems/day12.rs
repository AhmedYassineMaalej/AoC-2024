use std::collections::HashSet;

struct Garden {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

struct Measures {
    area: usize,
    perimeter: usize,
}

impl Measures {
    fn new() -> Self {
        Measures {
            area: 0,
            perimeter: 0,
        }
    }
}

#[derive(PartialEq, Clone, Hash, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Fence {
    direction: Direction,
    row: usize,
    col: usize,
}

impl Fence {
    fn next(&mut self) {
        match self.direction {
            Direction::North | Direction::South => self.col += 1,
            Direction::East | Direction::West => self.row += 1,
        };
    }

    fn prev(&mut self) {
        match self.direction {
            Direction::North | Direction::South => self.col = self.col.wrapping_sub(1),
            Direction::East | Direction::West => self.row = self.row.wrapping_sub(1),
        };
    }
}

impl Garden {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            width,
            height,
        }
    }

    fn mesure_region(
        &mut self,
        row: usize,
        col: usize,
        measures: &mut Measures,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if visited[row][col] {
            return;
        }
        // mark as visited
        visited[row][col] = true;

        let plant_type = self.grid[row][col];
        measures.area += 1;

        if row > 0 && self.grid[row - 1][col] == plant_type {
            self.mesure_region(row - 1, col, measures, visited);
        } else {
            measures.perimeter += 1;
        }

        if row + 1 < self.height && self.grid[row + 1][col] == plant_type {
            self.mesure_region(row + 1, col, measures, visited);
        } else {
            measures.perimeter += 1;
        }

        if col > 0 && self.grid[row][col - 1] == plant_type {
            self.mesure_region(row, col - 1, measures, visited);
        } else {
            measures.perimeter += 1;
        }

        if col + 1 < self.width && self.grid[row][col + 1] == plant_type {
            self.mesure_region(row, col + 1, measures, visited);
        } else {
            measures.perimeter += 1;
        }
    }

    fn mesure_area_and_fences(
        &mut self,
        row: usize,
        col: usize,
        area: &mut usize,
        visited: &mut Vec<Vec<bool>>,
        fences: &mut HashSet<Fence>,
    ) {
        if visited[row][col] {
            return;
        }

        // mark as visited
        visited[row][col] = true;

        let plant_type = self.grid[row][col];
        *area += 1;

        if row > 0 && self.grid[row - 1][col] == plant_type {
            self.mesure_area_and_fences(row - 1, col, area, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::North,
                row,
                col,
            });
        }

        if row + 1 < self.height && self.grid[row + 1][col] == plant_type {
            self.mesure_area_and_fences(row + 1, col, area, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::South,
                row,
                col,
            });
        }

        if col > 0 && self.grid[row][col - 1] == plant_type {
            self.mesure_area_and_fences(row, col - 1, area, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::West,
                row,
                col,
            });
        }

        if col + 1 < self.width && self.grid[row][col + 1] == plant_type {
            self.mesure_area_and_fences(row, col + 1, area, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::East,
                row,
                col,
            });
        }
    }
}

fn count_sides(fences: &mut HashSet<Fence>) -> usize {
    // counts fence sides and clears the hashset
    let mut sides = 0;

    while !fences.is_empty() {
        let fence_orig = fences.iter().next().unwrap().clone();

        let mut fence = fence_orig.clone();
        while fences.remove(&fence) {
            fence.next();
        }

        let mut fence = fence_orig;
        fence.prev();
        while fences.remove(&fence) {
            fence.prev();
        }

        sides += 1;
    }

    sides
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut garden = Garden::new(input);
    let mut visited = vec![vec![false; garden.width]; garden.height];

    let mut result = 0;

    for row in 0..garden.height {
        for col in 0..garden.width {
            if visited[row][col] {
                continue;
            }

            let mut measures = Measures::new();
            garden.mesure_region(row, col, &mut measures, &mut visited);
            result += measures.area * measures.perimeter;
        }
    }

    result
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut garden = Garden::new(input);
    let mut visited = vec![vec![false; garden.width]; garden.height];
    let mut fences = HashSet::new();

    let mut result = 0;

    for row in 0..garden.height {
        for col in 0..garden.width {
            if visited[row][col] {
                continue;
            }

            let mut area = 0;
            garden.mesure_area_and_fences(row, col, &mut area, &mut visited, &mut fences);

            result += area * count_sides(&mut fences);
        }
    }

    result
}
