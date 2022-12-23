use std::collections::HashSet;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Point {
    col: usize,
    row: usize,
}

const START_POINT: (usize, usize) = (500, 0);

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
    sands: HashSet<Point>,
}

// state for NEXT move
#[derive(Debug)]
enum SandState {
    Down,
    Left,
    Right,
    Abyss,
    Blocked,
}

pub fn solve(input: &Vec<&str>) -> u32 {
    let mut lines = Vec::new();
    for line in input {
        let current = Line::parse(line);
        lines.extend(current);
    }
    let mut grid = Grid::from(lines);
    grid.run()
}

impl Grid {
    fn from(lines: Vec<Line>) -> Self {
        let mut sands = HashSet::new();
        for line in lines.iter() {
            sands.extend(line.points());
        }
        Self {
            counter: 0,
            sand: (500, 0).into(),
            sands,
            state: SandState::Down,
        }
    }

    fn run(&mut self) -> u32 {
        while self.one_step() {}
        self.counter
    }

    fn is_occupied(&self, point: &Point) -> bool {
        self.sands.contains(point)
    }

    fn is_over_abyss(&self, point: Point) -> bool {
        self.sands.iter().all(|p| p.row < point.row)
    }

    fn one_step(&mut self) -> bool {
        match self.state {
            SandState::Down => {
                let bottom = self.sand.bottom();
                if self.is_over_abyss(self.sand) {
                    // println!("{:?} is over abyss!", self.sand);
                    self.state = SandState::Abyss;
                    return true;
                }
                if !self.is_occupied(&bottom) {
                    // println!("move down to {:?}", bottom);
                    self.sand = bottom;
                    true
                } else {
                    // println!("move left next time");
                    self.state = SandState::Left;
                    true
                }
            }
            SandState::Left => {
                let bottom_left = self.sand.bottom_left();
                if !self.is_occupied(&bottom_left) {
                    self.sand = bottom_left;
                    // println!("moved left to {:?}", bottom_left);
                    self.state = SandState::Down;
                    true
                } else {
                    // println!("try move right next time");
                    self.state = SandState::Right;
                    true
                }
            }
            SandState::Right => {
                let bottom_right = self.sand.bottom_right();
                if !self.is_occupied(&bottom_right) {
                    self.sand = bottom_right;
                    // println!("move right to {:?}", bottom_right);
                    self.state = SandState::Down;
                    true
                } else {
                    // println!("blocked on {:?}", self.sand);
                    self.state = SandState::Blocked;
                    true
                }
            }
            SandState::Blocked => {
                // println!("start new sand");
                self.sands.insert(self.sand);
                self.sand = START_POINT.into();
                self.state = SandState::Down;
                self.counter += 1;
                // println!("{}", self.counter);
                true
            }
            SandState::Abyss => false,
        }
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
            if start.col == end.col {
                if end.row < start.row {
                    (start, end) = (end, start);
                }
            } else if end.col < start.col {
                (start, end) = (end, start);
            }
            let line = Self { start, end };
            result.push(line);
        }
        result
    }

    fn points(&self) -> Vec<Point> {
        let mut result = Vec::new();
        if self.start.col == self.end.col {
            for i in self.start.row..(self.end.row + 1) {
                result.push(Point {
                    col: self.start.col,
                    row: i,
                });
            }
        } else {
            for i in self.start.col..self.end.col + 1 {
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

        assert_eq!(solve(&input), 24);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day14");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 768);
    }
}
