struct Grid {
    grid: Vec<Vec<char>>
}

impl Grid {
    fn find_start(&self) -> usize {
	let topmost = grid[0];
	for i in 0..topmost.len() {
	    if grid[i] == '.' {
		return i;
	    }
	}
	panic!()
    }
}

enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn turn(&self, to: Direction) -> Self {
	match (self, to) {
	    (Up, Left) => Left,
	    (Up, Right) => Right,
	    (Down, Left) => Right,
	    (Down, Right) => Left,
	    (Left, Left) => Down,
	    (Left, Right) => Up,
	    (Right, Left) => Up,
	    (Right, Right) => Down,
	    (_, _) => panic!("turn from {} to {} is not supported", self, to)
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
    fn step(&self) -> Self {
	match direction {
	    Direction::Up => (self.col, self.row-1, self.direction),
	    Direction::Down => {self.col, self.row+1, self.direction},
	    Direction::Left => {self.col-1, self.row, self.direction},
	    Direction::Right => {self.col+1, self.row, self.direction},
	}
    }

    fn evaluate(&self) -> usize {
	let score = match self.direction {
	    Direction::Right => 0,
	    Direction::Down => 1,
	    Direction::Left => 2,
	    Direction:: Up => 3,
	}
	(self.row +1 )*1000 + (self.col+1)*4 +score
    }
}

struct Trip() {
    grid: Grid,
    cursor: (usize, usize, Direction),
}

impl Trip {
    fn new(grid: Grid) -> Self {
	let cursor = (grid.find_start(), 1, Direction::Right);
	Self { grid, cursor }
    }

    fn action(&mut self, action: &Action) {
	match action {
	    Action::Turn(to) => {
		let (col, row, direction) = self.cursor;
		self.cursor = (col, row, direction.turn(to))
	    },
	    Action::Move(distance) => {
		for i in 0..distance {
		    let updated@(col, row, _) = self.cursor.step();
		    if self.grid.is_out_of_bounds(col, row) || self.grid.is_void(col,row) {
			todo!();
		    }
		    if self.grid.is_wall(col, row) {
			break;
		    }
		    self.cursor = udpated;
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
    fn new(raw: String) -> Self {
	let state = raw.chars().collect();
	Self { state }
    }

    fn pop_command(&mut self) -> Option<Action> {
	match state.pop_front() {
	    None => None,
	    Some(x) && x.is_alpha() => { Some(Action::Turn(x.parse().unwrap())) },
	    Some(x) => {
		let mut res = String::from(x);
		while let Some(y) = self.state.pop_front() {
		    res.push(y);
		}
		Some(Action::Move(res.parse().unwrap()))
	    }
	}
	
    }
}

pub fn solve(input: &Vec<&str>) -> usize {
    let trip = Trip::new(&input);
    let commads = Commands::new(&input);

    while let Some(command) = commands.pop_command() {
	trip.action(command);
    }

    trip.password()
}


#[cfg(test)]
mod tests {
    use super:*;

    #[test]
    fn test_solve(){
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
"10R5L5R10L4R5L5"
	];
	assert_eq!(solve(input), 6032);
    }
}

