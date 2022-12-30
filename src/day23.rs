type Position = (i32, i32);

fn adjacent((col, row): (i32, i32)) -> [Position; 8] {
    [
	(col-1, row-1),
	(col, row-1),
	(col+1, row-1),
	(col+1, row),
	(col+1, row+1),
	(col, row+1),
	(col-1, row+1),
	(col-1, row)
    ]
}

enum Directions {
    North, East, South, West
}

impl Directions{
    fn check_points(&self, (col,row): &Position) -> [Position; 3]{
	match self {
	    Directions::North => [(col-1, row-1, (col, row-1), (col+1, row -1) )],
	    Directions::East => [(col+1, row-1), (col+1, row-1),( col+1, row +1 )],
	    Directions::South => [(col-1, row+1),( col, row+1),( col-1, row +1 )],
	    Directions::West => [(col-1, row-1), (col-1, row), (col-1, row +1 )],
	}
    }
}
    
struct State {
    elves: Vec<Position>,
    directions: VecDeque<Direction>
}


impl State {
    fn step(&mut self) {
	let moving_elves = HashSet::new();
	for elf in elves {
	    let adjacet = adjacent(elf);
	    let occupied = adjacent.intersection(elves);
	    if !occupied.is_empty() {
		moving_elves.insert(elf);
	    }
	}

	let pos_to_elves = HashMap::new();
	for elf in moving_elves {
	    for direction in directions {
		let check_points = direction.check_points(elf);
		if !check_points.intersection(elves).is_empty(){
		    continue;
		}
		let new_pos = elf.go_to(direction);
		let xs = pos_to_elves.put_if_absent(new_pos, Vec::new());
		xs.push(elf);
	    }
	}
	

	for(k,xs) in pos_to_elves {
	    if xs.len() > 1 {
		continue;
	    }

	    let new_pos = xs[0];
	    elves.remove(k);
	    elves.insert(new_pos);
	}

	let direction = self.directions.pop_front();
	self.directions.push_back(direction);
    }

    fn count_empty_ground(&self) -> u32 {
	let mut n = u32::MAX_VALUE;
	let mut w = u32::MAX_VALUE;
	let mut e = u32::MIN_VALUE;
	let mut s = u32::MIN_VALUE;
	
	for (col, row) in self.elves {
	    n = min(n, row);
	    w = min(w, col);
	    e = max(e, col);
	    s = max(s, row);
	}

	(s-n)*(e-w) - self.elves.size()
    }
}

pub fn solve(input: &Vec<&str>) -> u32 {
    let state = State::new(input);
    for i in 0..10 {
	state.step();
    }
    state.count_empty_ground()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solve(){
	let input = [
"....#..",
"..###.#",
"#...#.#",
".#...##",
"#.###..",
"##.#.##",
".#..#..",
	];
	assert_eq!(solve(input), 110);
    }
    
}
