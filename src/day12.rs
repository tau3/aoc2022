use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let (start, end) = find_start(&grid);
        Self {
            grid,
            width,
            height,
            start,
            end,
        }
    }

    fn solve(&self) -> u32 {
        let mut memo = HashMap::new();
        let mut visited = HashSet::new();
        self.shortest_path(self.end, &mut memo, &mut visited)
            .expect("no path found")
    }

    fn shortest_path(
        &self,
        current: Point,
        memo: &mut HashMap<Point, Option<u32>>,
        visited: &mut HashSet<Point>,
    ) -> Option<u32> {
        if current == self.start {
            return Some(0);
        }
        visited.insert(current);

        let mut candidates = Vec::new();
        for point in self.not_visited_adjacent(visited, current) {
            if !self.can_jump(point, current) {
                continue;
            }
            if let Some(path_length) = self.calc_shortest(memo, visited, point) {
                candidates.push(path_length);
            }
        }
        candidates.iter().min().map(|result| result + 1)
    }

    fn not_visited_adjacent(&self, visited: &HashSet<Point>, current: Point) -> Vec<Point> {
        let adjacent = self.adjacent(current);
        adjacent
            .iter()
            .filter(|point| !visited.contains(point))
            .map(|(col, row)| (*col, *row))
            .collect()
    }

    fn calc_shortest(
        &self,
        memo: &mut HashMap<Point, Option<u32>>,
        visited: &mut HashSet<Point>,
        point: Point,
    ) -> Option<u32> {
        let mut result = None;
        let maybe_path_length = memo.get(&point);
        if maybe_path_length.is_none() {
            result = self.shortest_path(point, memo, visited);
            memo.insert(point, result);
        }
        result
    }

    fn at(&self, (col, row): Point) -> char {
        self.grid[row][col]
    }

    fn can_jump(&self, from: Point, to: Point) -> bool {
        let from = self.at(from);
        let to = self.at(to);
        can_jump(from, to)
    }

    fn adjacent(&self, (col, row): Point) -> Vec<Point> {
        let (col, row) = (col as i32, row as i32);
        let result = vec![
            (col, row + 1),
            (col + 1, row),
            (col, row - 1),
            (col - 1, row),
        ];

        result
            .iter()
            .filter(|(col, row)| self.fits_grid((col, row)))
            .map(|(col, row)| (*col as usize, *row as usize))
            .collect()
    }

    fn fits_grid(&self, (col, row): (&i32, &i32)) -> bool {
        *col >= 0 && *row >= 0 && *col < self.width as i32 && *row < self.height as i32
    }
}

fn add1_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap()
}

fn can_jump(from: char, to: char) -> bool {
    if from == 'S' || from == to {
        return true;
    };
    if to == 'E' {
        return from == 'z' || from == 'y';
    }

    add1_char(from) == to
}

pub fn solve(input: Vec<Vec<char>>) -> u32 {
    let grid = Grid::new(input);
    grid.solve()
}

fn find_start(input: &[Vec<char>]) -> (Point, Point) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row, line) in input.iter().enumerate() {
        for col in 0..line.len() {
            if input[row][col] == 'S' {
                start = (col, row);
            } else if input[row][col] == 'E' {
                end = (col, row);
            }
        }
    }
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::util;

    #[test]
    fn test_solve() {
        let input = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

        let input = input.iter().map(|line| line.chars().collect()).collect();
        assert_eq!(solve(input), 31);
    }

    #[test]
    fn test_can_jump() {
        assert!(!can_jump('u', 'z'));
    }

    // #[test]
    // fn test_with_real_data() {
        // let input = util::read_real_data("day12");
        // let input = input.iter().map(|line| line.chars().collect()).collect();
        // assert_eq!(solve(input), 123);
    // }
}
