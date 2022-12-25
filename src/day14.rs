use std::{cmp::Ordering, collections::HashSet};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Point {
    col: usize,
    row: usize,
}

const START_POINT: Point = Point { col: 500, row: 0 };

impl Point {
    fn parse(input: &str) -> Self {
        let mut tokens = input.split(',');
        let col = tokens.next().unwrap().parse().unwrap();
        let row = tokens.next().unwrap().parse().unwrap();
        Self { col, row }
    }

    fn bottom(&self) -> Self {
        Point {
            col: self.col,
            row: self.row + 1,
        }
    }

    fn bottom_left(&self) -> Self {
        Point {
            col: self.col - 1,
            row: self.row + 1,
        }
    }

    fn bottom_right(&self) -> Self {
        Point {
            col: self.col + 1,
            row: self.row + 1,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((col, row): (usize, usize)) -> Self {
        Self { col, row }
    }
}

#[derive(Debug)]
struct Grid {
    sand: Point,
    state: SandState,
    counter: u32,
    occupied: HashSet<Point>,
    floor: Option<usize>,
}

// state for NEXT move
#[derive(Debug)]
enum SandState {
    Down,
    Left,
    Right,
    Abyss,
    Blocked,
    Filled,
}

pub fn solve(input: &Vec<&str>, with_floor: bool) -> u32 {
    let mut lines = Vec::new();
    for line in input {
        let current = Line::parse(line);
        lines.extend(current);
    }
    let mut grid = Grid::from(lines, with_floor);
    grid.run()
}

impl Grid {
    fn from(lines: Vec<Line>, with_floor: bool) -> Self {
        let mut occupied = HashSet::new();
        for line in lines.iter() {
            occupied.extend(line.points());
        }

        let mut floor = None;
        if with_floor {
            let floor_row = occupied.iter().map(|point| point.row).max().unwrap() + 2;
            floor = Some(floor_row);
        }

        Self {
            counter: 0,
            sand: START_POINT,
            occupied,
            state: SandState::Down,
            floor,
        }
    }

    fn run(&mut self) -> u32 {
        while self.one_step() {}
        self.counter
    }

    fn is_occupied(&self, point: &Point) -> bool {
        let mut result = self.occupied.contains(point);
        if let Some(row) = self.floor {
            result |= point.row == row;
        }
        result
    }

    fn is_over_abyss(&self, point: Point) -> bool {
        if self.floor.is_some() {
            return false;
        }
        self.occupied.iter().all(|p| p.row < point.row)
    }

    fn one_step(&mut self) -> bool {
        match self.state {
            SandState::Down => self.on_down(),
            SandState::Left => self.on_left(),
            SandState::Right => self.on_right(),
            SandState::Blocked => self.on_blocked(),
            SandState::Abyss | SandState::Filled => false,
        }
    }

    fn on_down(&mut self) -> bool {
                let bottom = self.sand.bottom();
                if self.is_over_abyss(self.sand) {
                    self.state = SandState::Abyss;
                    return true;
                }
                if !self.is_occupied(&bottom) {
                    self.sand = bottom;
                    true
                } else {
                    self.state = SandState::Left;
                    true
                }
    }

    fn on_left(&mut self) -> bool {
        let bottom_left = self.sand.bottom_left();
        if !self.is_occupied(&bottom_left) {
            self.sand = bottom_left;
            self.state = SandState::Down;
        } else {
            self.state = SandState::Right;
        }
            true
    }

    fn on_right(&mut self) -> bool {
        let bottom_right = self.sand.bottom_right();
        if !self.is_occupied(&bottom_right) {
            self.sand = bottom_right;
            self.state = SandState::Down;
        } else {
            self.state = SandState::Blocked;
        }
        true
    }

    fn on_blocked(&mut self) -> bool {
        if self.sand == START_POINT {
            self.state = SandState::Filled;
            self.counter += 1;
            return true;
        }
        self.occupied.insert(self.sand);
        self.sand = START_POINT;
        self.state = SandState::Down;
        self.counter += 1;
        true
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse(input: &str) -> Vec<Self> {
        let tokens: Vec<&str> = input.split(" -> ").collect();
        let len = tokens.len() - 1;
        let mut result = Vec::new();
        for i in 0..len {
            let mut start = Point::parse(tokens[i]);
            let mut end = Point::parse(tokens[i + 1]);
            match start.col.cmp(&end.col) {
                Ordering::Equal if end.row < start.row => {
                    (start, end) = (end, start);
                }
                Ordering::Greater => {
                    (start, end) = (end, start);
                }
                _ => {}
            };
            let line = Self { start, end };
            result.push(line);
        }
        result
    }

    fn points(&self) -> Vec<Point> {
        let mut result = Vec::new();
        if self.start.col == self.end.col {
            let end = self.end.row + 1;
            for i in self.start.row..end {
                result.push(Point {
                    col: self.start.col,
                    row: i,
                });
            }
        } else {
            let end = self.end.col + 1;
            for i in self.start.col..end {
                result.push(Point {
                    col: i,
                    row: self.start.row,
                });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_parse_line() {
        let actual = Line::parse("503,4 -> 502,4 -> 502,9 -> 494,9");
        let expected = vec![
            Line {
                start: (502, 4).into(),
                end: (503, 4).into(),
            },
            Line {
                start: (502, 4).into(),
                end: (502, 9).into(),
            },
            Line {
                start: (494, 9).into(),
                end: (502, 9).into(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve() {
        let input = vec![
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];

        assert_eq!(solve(&input, false), 24);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day14");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input, false), 768);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day14");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input, true), 26686);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];

        assert_eq!(solve(&input, true), 93);
    }
}
