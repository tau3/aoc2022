use std::collections::VecDeque;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn min_wrap_col(&self, col: usize) -> (usize, usize) {
        let column: Vec<char> = self.col(col);
        for (i, point) in column.iter().enumerate() {
            if *point != ' ' {
                return (col, i);
            }
        }
        unreachable!()
    }

    fn max_wrap_col(&self, col: usize) -> (usize, usize) {
        let column: Vec<char> = self.col(col);
        for i in (0..column.len()).rev() {
            if column[i] != ' ' {
                return (col, i);
            }
        }
        unreachable!()
    }

    fn max_wrap_row(&self, row: usize) -> (usize, usize) {
        let full_row = &self.grid[row];
        for i in (0..full_row.len()).rev() {
            if full_row[i] != ' ' {
                return (i, row);
            }
        }
        unreachable!()
    }

    fn min_wrap_row(&self, row: usize) -> (usize, usize) {
        let full_row = &self.grid[row];
        for (i, point) in full_row.iter().enumerate() {
            if *point != ' ' {
                return (i, row);
            }
        }
        unreachable!()
    }

    fn col(&self, col: usize) -> Vec<char> {
        let mut result = Vec::new();
        for line in self.grid.iter() {
            if col >= line.len() {
                result.push(' ');
            } else {
                result.push(line[col]);
            }
        }
        result
    }

    fn new(raw: &[&str]) -> Self {
        let grid = raw
            .iter()
            .map(|line| line.chars())
            .map(|chars| chars.collect())
            .collect();
        Self { grid }
    }

    fn is_wall(&self, col: usize, row: usize) -> bool {
        if row >= self.grid.len() {
            return false;
        }
        if col >= self.grid[row].len() {
            return false;
        }
        self.grid[row][col] == '#'
    }

    fn is_out_of_bounds(&self, col: isize, row: isize) -> bool {
        if col < 0 || row < 0 {
            return true;
        }

        if row >= self.grid.len() as isize {
            return true;
        }

        let row = &self.grid[row as usize];
        if col as usize >= row.len() {
            return true;
        }
        row[col as usize] == ' '
    }

    fn find_start(&self) -> usize {
        let topmost = &self.grid[0];
        for (i, point) in topmost.iter().enumerate() {
            if *point == '.' {
                return i;
            }
        }
        panic!()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("{} direction is not supported", c),
        }
    }
}

impl Direction {
    fn turn(&self, to: &Direction) -> Self {
        match (self, to) {
            (Direction::Up, Direction::Left) => Direction::Left,
            (Direction::Up, Direction::Right) => Direction::Right,
            (Direction::Down, Direction::Left) => Direction::Right,
            (Direction::Down, Direction::Right) => Direction::Left,
            (Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Right, Direction::Right) => Direction::Down,
            (_, _) => panic!("turn from {:?} to {:?} is not supported", self, to),
        }
    }
}

enum Action {
    Go(usize),
    Turn(Direction),
}

#[derive(Debug)]
struct Cursor {
    col: usize,
    row: usize,
    direction: Direction,
}

impl Cursor {
    fn turn(&self, to: &Direction) -> Self {
        Self {
            col: self.col,
            row: self.row,
            direction: self.direction.turn(to),
        }
    }

    fn step(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (self.col as isize, self.row as isize - 1),
            Direction::Down => (self.col as isize, self.row as isize + 1),
            Direction::Left => (self.col as isize - 1, self.row as isize),
            Direction::Right => (self.col as isize + 1, self.row as isize),
        }
    }

    fn evaluate(&self) -> usize {
        let score = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        (self.row + 1) * 1000 + (self.col + 1) * 4 + score
    }
}

struct Trip {
    grid: Grid,
    cursor: Cursor,
}

impl Trip {
    fn new(input: &[&str]) -> Self {
        let grid = Grid::new(&input[0..input.len() - 2]);
        let cursor = Cursor {
            col: grid.find_start(),
            row: 0,
            direction: Direction::Right,
        };
        Self { grid, cursor }
    }

    fn action(&mut self, action: &Action) {
        match action {
            Action::Turn(to) => self.cursor = self.cursor.turn(to),
            Action::Go(distance) => {
                for _ in 0..*distance {
                    let (col, row) = self.cursor.step();
                    if self.grid.is_out_of_bounds(col, row) {
                        if let Some((c, r)) = self.wrap() {
                            self.cursor = Cursor {
                                col: c,
                                row: r,
                                direction: self.cursor.direction,
                            };
                            break;
                        }
                    }
                    if self.grid.is_wall(col as usize, row as usize) {
                        break;
                    }
                    self.cursor = Cursor {
                        col: col as usize,
                        row: row as usize,
                        direction: self.cursor.direction,
                    };
                }
            }
        }
    }

    fn wrap(&mut self) -> Option<(usize, usize)> {
        let target @ (col, row) = match self.cursor.direction {
            Direction::Down => self.grid.min_wrap_col(self.cursor.col),
            Direction::Up => self.grid.max_wrap_col(self.cursor.col),
            Direction::Left => self.grid.max_wrap_row(self.cursor.row),
            Direction::Right => self.grid.min_wrap_row(self.cursor.row),
        };

        if self.grid.is_wall(col, row) {
            return None;
        }
        Some(target)
    }

    fn password(&self) -> usize {
        self.cursor.evaluate()
    }
}

struct Commands {
    state: VecDeque<char>,
}

impl Commands {
    fn new(input: &Vec<&str>) -> Self {
        let raw = input[input.len() - 1];
        let state = raw.chars().collect();
        Self { state }
    }

    fn pop_command(&mut self) -> Option<Action> {
        match self.state.pop_front() {
            None => None,
            Some(x) if x.is_alphabetic() => Some(Action::Turn(x.into())),
            Some(x) => {
                let mut res = String::from(x);
                while let Some(y) = self.state.pop_front() {
                    if y.is_numeric() {
                        res.push(y);
                    } else {
                        self.state.push_front(y);
                        break;
                    }
                }
                Some(Action::Go(
                    res.parse()
                        .unwrap_or_else(|_| panic!("can't parse {}", res)),
                ))
            }
        }
    }
}

pub fn solve(input: &Vec<&str>) -> usize {
    let mut trip = Trip::new(input);
    let mut commands = Commands::new(input);

    while let Some(command) = commands.pop_command() {
        trip.action(&command);
    }

    trip.password()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = vec![
            "        ...#",
            "        .#..",
            "        #...",
            "        ....",
            "...#.......#",
            "........#...",
            "..#....#....",
            "..........#.",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ];
        assert_eq!(solve(&input), 6032);
    }

    // #[test]
    // fn test_with_real_data() {
        // let input = util::read_real_data("day22");
        // let input = input.iter().map(|line| line.as_str()).collect();
        // assert_eq!(solve(&input), 123);
    // }
}
