// TODO memo

use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
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

    fn shortest_path(
        &self,
        current: (usize, usize),
        prev: (usize, usize),
        memo: &mut HashMap<(usize, usize), Option<u32>>,
    ) -> Option<u32> {
        if current == self.start {
            println!("BINGO");
            return Some(0);
        }
        let mut adjacent = self.adjacent(current);
        let position = adjacent.iter().position(|x| *x == prev);
        if let Some(pos) = position {
            adjacent.remove(pos);
        }

        println!("point {:?}, adjacent {:?}", current, adjacent);
        let mut cand = Vec::new();
        // let mut m1 = memo.borrow_mut();
        for point in adjacent {
            if !self.can_jump(point, current) {
                continue;
            }
            if let Some(dest) = self.get_dest(memo, point, current) {
                cand.push(dest);
            }
        }
        cand.iter().min().map(|x| x + 1)
    }

    fn get_dest(
        &self,
        memo: &mut HashMap<(usize, usize), Option<u32>>,
        point: (usize, usize),
        current: (usize, usize),
    ) -> Option<u32> {
        let mut maybe_path = None;
        // let mut m1 = memo;
        let maybe_dest = memo.get(&point);
        if maybe_dest.is_none() {
            maybe_path = self.shortest_path(point, current, memo);
            memo.insert(point, maybe_path);
        }

        // m1.drop();
        maybe_path
    }

    fn at(&self, (col, row): (usize, usize)) -> char {
        self.grid[row][col]
    }

    fn can_jump(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let from = self.at(from);
        let to = self.at(to);
        can_jump(from, to)
    }

    fn adjacent(&self, (col, row): (usize, usize)) -> Vec<(usize, usize)> {
        let (col, row) = (col as i32, row as i32);
        let result = vec![
            (col, row + 1),
            (col + 1, row),
            (col, row - 1),
            (col - 1, row),
        ];

        result
            .iter()
            .filter(|(col, row)| {
                *col >= 0 && *row >= 0 && *col < self.width as i32 && *row < self.height as i32
            })
            .map(|(col, row)| (*col as usize, *row as usize))
            .collect()
    }
}

fn add1_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap()
}

fn can_jump(from: char, to: char) -> bool {
    if from == 'S' || to == 'E' || from == to {
        return true;
    };

    add1_char(from) == to
}

pub fn solve(input: Vec<Vec<char>>) -> u32 {
    let grid = Grid::new(input);
    let mut x = HashMap::new();
    grid.shortest_path(grid.end, grid.end, &mut x).unwrap()
}

fn find_start(input: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == 'S' {
                start = (col, row);
            } else if input[row][col] == 'E' {
                end = (col, row);
            }
        }
    }
    println!("start {:?}, end {:?}", start, end);

    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
