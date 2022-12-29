use std::collections::VecDeque;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(raw:&Vec<&str>)->Self {
	todo!()
    }
    
    fn is_wall(&self, col:usize, row:usize) -> bool {
	self.grid[row][col] == '#'
    }

    fn is_out_of_bounds(&self, col:usize, row:usize)->bool{
	if col < 0 || row < 0 {
	    return true;
	}

	if row >= self.grid.len() {
	    return true;
	}

	let row = &self.grid[row];
	if col >= row.len() {
	    return true;
	}
	return row[col] == ' ';
    }
    
    fn find_start(&self) -> usize {
        let topmost = &self.grid[0];
        for i in 0..topmost.len() {
            if topmost[i] == '.' {
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
	    _ => panic!("{} direction is not supported", c)
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

struct Cursor {
    col: usize,
    row: usize,
    direction: Direction,
}

impl Cursor {
    fn position(&self) -> (usize, usize) {
	(self.col, self.row)
    }
    
    fn turn(&self, to:&Direction) -> Self {
	Self{col:self.col, row:self.row, direction:self.direction.turn(to)}
    }
    
    fn step(&self) -> Self {
        return match self.direction {
            Direction::Up => Self {
                col: self.col,
                row: self.row - 1,
                direction: self.direction,
            },
            Direction::Down => Self {
                col: self.col,
                row: self.row + 1,
                direction: self.direction,
            },
            Direction::Left => Self {
                col: self.col - 1,
                row: self.row,
                direction: self.direction,
            },
            Direction::Right => Self {
                col: self.col + 1,
                row: self.row,
                direction: self.direction,
            },
        };
    }

    fn evaluate(&self) -> usize {
        let score = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        return (self.row + 1) * 1000 + (self.col + 1) * 4 + score;
    }
}

struct Trip {
    grid: Grid,
    cursor: Cursor,
}

impl Trip {
    fn new(input:&Vec<&str>) -> Self {
	let grid = Grid::new(input);
        let cursor = Cursor {
            col: grid.find_start(),
            row: 1,
            direction: Direction::Right,
        };
        Self { grid, cursor }
    }

    fn action(&mut self, action: &Action) {
        match action {
            Action::Turn(to) => self.cursor = self.cursor.turn(to),
            Action::Go(distance) => {
                for _ in 0..*distance {
                    let updated  = self.cursor.step();
		    let (col, row) = updated.position();
                    if self.grid.is_out_of_bounds(col, row) {
                        todo!();
                    }
                    if self.grid.is_wall(col, row){
                        break;
                    }
                    self.cursor = updated;
                }
            }
        }
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
	let raw = input[input.len()-1];
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
                    res.push(y);
                }
                Some(Action::Go(res.parse().unwrap()))
            }
        }
    }
}

pub fn solve(input: &Vec<&str>) -> usize {
    let mut trip = Trip::new(&input);
    let mut  commands = Commands::new(&input);

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
}
