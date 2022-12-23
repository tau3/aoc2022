#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
    sands: Vec<Point>,
    borders: Vec<Line>,
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
    println!("grid {:?}", grid);
    grid.run()
}

impl Grid {
    fn from(lines: Vec<Line>) -> Self {
        Self {
            counter: 0,
            borders: lines,
            sand: (500, 0).into(),
            sands: Vec::new(),
            state: SandState::Down,
        }
    }

    fn run(&mut self) -> u32 {
        while self.one_step() {}
        self.counter
    }

    fn is_occupied(&self, point: &Point) -> bool {
        if self.sands.contains(point) {
            return true;
        }
        self.borders.iter().any(|border| border.has_point(point))
    }

    fn is_over_abyss(&self, point: Point) -> bool {
        !self
            .borders
            .iter()
            .any(|border| border.contains_col(point.col))
    }

    fn one_step(&mut self) -> bool {
        match self.state {
            SandState::Down => {
                let bottom = self.sand.bottom();
                if self.is_over_abyss(bottom) {
                    println!("{:?} is over abyss!", self.sand);
                    self.state = SandState::Abyss;
                    return true;
                }
                if !self.is_occupied(&bottom) {
                    println!("move down to {:?}", bottom);
                    self.sand = bottom;
                    true
                } else {
                    println!("move left next time");
                    self.state = SandState::Left;
                    true
                }
            }
            SandState::Left => {
                let bottom_left = self.sand.bottom_left();
                if !self.is_occupied(&bottom_left) {
                    self.sand = bottom_left;
                    println!("moved left to {:?}", bottom_left);
                    self.state = SandState::Down;
                    true
                } else {
                    println!("try move right next time");
                    self.state = SandState::Right;
                    true
                }
            }
            SandState::Right => {
                let bottom_right = self.sand.bottom_right();
                if !self.is_occupied(&bottom_right) {
                    self.sand = bottom_right;
                    println!("move right to {:?}", bottom_right);
                    self.state = SandState::Down;
                    true
                } else {
                    println!("blocked on {:?}", self.sand);
                    self.state = SandState::Blocked;
                    true
                }
            }
            SandState::Blocked => {
                println!("start new sand");
                self.sands.push(self.sand);
                self.sand = START_POINT.into();
                self.state = SandState::Down;
                self.counter += 1;
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

    fn contains_col(&self, col: usize) -> bool {
        col >= self.start.col && col <= self.end.col
    }

    fn has_point(&self, point: &Point) -> bool {
        let (col, row) = (point.col, point.row);
        if self.start.col == self.end.col {
            self.start.col == col && (row >= self.start.row && row <= self.end.row)
        } else {
            self.start.row == row && self.contains_col(col)
        }
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
        assert_eq!(solve(&input), 123);
    }
}
