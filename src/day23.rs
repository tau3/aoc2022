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
            Direction::East => [
                (*col + 1, *row - 1),
                (*col + 1, *row - 1),
                (col + 1, row + 1),
            ],
            Direction::South => [(*col - 1, *row + 1), (*col, *row + 1), (*col - 1, *row + 1)],
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
        for row_index in 0..input.len() {
            let row = input[row_index];
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
            if !occupied.next().is_some() {
                moving_elves.insert((*col, *row));
            }
        }
        moving_elves
    }

    fn step(&mut self) {
        let moving_elves = self.get_moving_elves();

        let mut pos_to_elves = HashMap::new();
        for elf in moving_elves {
            for direction in self.directions.iter() {
                let check_points = direction.check_points(&elf);
                if !check_points.intersection(&self.elves).next().is_some() {
                    continue;
                }
                let new_pos = direction.go(&elf);
                match pos_to_elves.get_mut(&new_pos) {
                    None => {
                        pos_to_elves.insert(new_pos, Vec::from([elf]));
                    }
                    Some(xs) => {
                        xs.push(elf);
                    }
                };
            }
        }

        for (k, xs) in pos_to_elves.iter() {
            if xs.len() > 1 {
                continue;
            }

            let new_pos = xs[0];
            self.elves.remove(&k);
            self.elves.insert(new_pos);
        }

        self.swap_direction();
    }

    fn swap_direction(&mut self) {
        let direction = self.directions.pop_front().unwrap();
        self.directions.push_back(direction);
    }

    fn count_empty_ground(&self) -> i32 {
        let mut n = i32::MAX;
        let mut w = i32::MAX;
        let mut e = i32::MIN;
        let mut s = i32::MIN;

        for (col, row) in self.elves.iter() {
            n = min(n, *row);
            w = min(w, *col);
            e = max(e, *col);
            s = max(s, *row);
        }

        (s - n) * (e - w) - self.elves.len() as i32
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

    #[test]
    fn test_solve() {
        let input = [
            "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
        ];
        assert_eq!(solve(&input), 110);
    }
}