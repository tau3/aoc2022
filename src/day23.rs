use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

type Position = (i32, i32);

fn adjacent((col, row): &(i32, i32)) -> HashSet<Position> {
    HashSet::from([
        (*col - 1, *row - 1),
        (*col, *row - 1),
        (*col + 1, *row - 1),
        (*col + 1, *row),
        (*col + 1, *row + 1),
        (*col, *row + 1),
        (*col - 1, *row + 1),
        (*col - 1, *row),
    ])
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn go(&self, (col, row): &Position) -> Position {
        match self {
            Direction::North => (*col, *row - 1),
            Direction::East => (*col + 1, *row),
            Direction::South => (*col, *row + 1),
            Direction::West => (*col - 1, *row),
        }
    }

    fn check_points(&self, (col, row): &Position) -> HashSet<Position> {
        let result = match self {
            Direction::North => [(*col - 1, *row - 1), (*col, *row - 1), (*col + 1, *row - 1)],
            Direction::East => [(*col + 1, *row - 1), (*col + 1, *row), (col + 1, row + 1)],
            Direction::South => [(*col - 1, *row + 1), (*col, *row + 1), (*col + 1, *row + 1)],
            Direction::West => [(*col - 1, *row - 1), (*col - 1, *row), (*col - 1, *row + 1)],
        };
        HashSet::from(result)
    }
}

struct State {
    elves: HashSet<Position>,
    directions: VecDeque<Direction>,
}

impl State {
    fn new(input: &[&str]) -> Self {
        let mut elves = HashSet::new();
        for (row_index, row) in input.iter().enumerate() {
            for (col, val) in row.chars().enumerate() {
                if val == '#' {
                    elves.insert((col as i32, row_index as i32));
                }
            }
        }
        let directions = VecDeque::from([
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]);
        Self { elves, directions }
    }

    fn get_moving_elves(&self) -> HashSet<Position> {
        let mut moving_elves = HashSet::new();
        for elf @ (col, row) in self.elves.iter() {
            let adjacent = adjacent(elf);
            let mut occupied = adjacent.intersection(&self.elves);
            if occupied.next().is_some() {
                moving_elves.insert((*col, *row));
            }
        }
        moving_elves
    }

    fn group_target_moves(
        &self,
        moving_elves: HashSet<Position>,
    ) -> HashMap<Position, Vec<Position>> {
        let mut result = HashMap::new();
        for elf in moving_elves {
            for direction in self.directions.iter() {
                let check_points = direction.check_points(&elf);
                if check_points.intersection(&self.elves).next().is_some() {
                    continue;
                }
                let destination = direction.go(&elf);
                match result.get_mut(&destination) {
                    None => {
                        result.insert(destination, Vec::from([elf]));
                    }
                    Some(elves) => {
                        elves.push(elf);
                    }
                };
                break;
            }
        }
        result
    }

    fn move_elves(&mut self, target_to_elves: HashMap<Position, Vec<Position>>) {
        for (target, elves) in target_to_elves.iter() {
            if elves.len() > 1 {
                continue;
            }

            let elf = elves[0];
            self.elves.remove(&elf);
            self.elves.insert(*target);
        }
    }

    fn step(&mut self) {
        let moving_elves = self.get_moving_elves();
        let target_to_elves = self.group_target_moves(moving_elves);
        self.move_elves(target_to_elves);
        self.swap_direction();
    }

    fn swap_direction(&mut self) {
        let direction = self.directions.pop_front().unwrap();
        self.directions.push_back(direction);
    }

    fn count_empty_ground(&self) -> i32 {
        let mut north = i32::MAX;
        let mut west = i32::MAX;
        let mut east = i32::MIN;
        let mut south = i32::MIN;

        for (col, row) in self.elves.iter() {
            north = min(north, *row);
            west = min(west, *col);
            east = max(east, *col);
            south = max(south, *row);
        }

        (south - north + 1) * (east - west + 1) - self.elves.len() as i32
    }
}

pub fn solve(input: &[&str]) -> i32 {
    let mut state = State::new(input);
    for _ in 0..10 {
        state.step();
    }
    state.count_empty_ground()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = [
            "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
        ];
        assert_eq!(solve(&input), 110);
    }

    #[test]
    fn tet_with_real_data() {
        let input = util::read_real_data("day23");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 3788);
    }
}
