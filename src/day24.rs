use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Hash, Eq, PartialEq)]
struct Blizzard {
    col: i32,
    row: i32,
    direction: char,
}

impl Blizzard {
    fn at(&self, width: i32, height: i32, t: i32) -> (i32, i32) {
        match self.direction {
            '>' => {
                let mut res = self.col;
                for _ in 0..t {
                    res += 1;
                    if res == width - 1 {
                        res = 1;
                    }
                }
                (res, self.row)
            }
            '^' => {
                let mut res = self.row;
                for _ in 0..t {
                    res -= 1;
                    if res == 0 {
                        res = height - 2;
                    }
                }
                (self.col, res)
            }
            '<' => {
                let mut res = self.col;
                for _ in 0..t {
                    res -= 1;
                    if res == 0 {
                        res = width - 2;
                    }
                }
                (res, self.row)
            }
            'v' => {
                let mut res = self.row;
                for _ in 0..t {
                    res += 1;
                    if res == height - 1 {
                        res = 1;
                    }
                }
                (self.col, res)
            }
            _ => panic!("incorrect direction {}", self.direction),
        }
    }
}

struct Graph {
    start: (i32, i32),
    end: (i32, i32),
    width: i32,
    height: i32,
    blizzards: HashSet<Blizzard>,
}

#[derive(PartialEq, Eq)]
struct Item {
    v: (i32, i32),
    t: i32,
    end: (i32, i32),
}

impl Item {
    fn dist_to_end(&self) -> i32 {
        self.t + (self.end.0 - self.v.0).abs() + (self.end.1 - self.v.1).abs()
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist_to_end().cmp(&self.dist_to_end())
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn new(input: &[&str]) -> Self {
        let start = input[0].chars().position(|x| x == '.').unwrap();
        let end = input
            .last()
            .unwrap()
            .chars()
            .position(|x| x == '.')
            .unwrap();

        let mut blizzards = HashSet::new();
        for (r, row) in input.iter().enumerate() {
            for (c, x) in row.chars().enumerate() {
                if "<>v^".contains(x) {
                    blizzards.insert(Blizzard {
                        col: c as i32,
                        row: r as i32,
                        direction: x,
                    });
                }
            }
        }

        Self {
            start: (start as i32, 0),
            end: (end as i32, input.len() as i32 - 1),
            width: input[0].len() as i32,
            height: input.len() as i32,
            blizzards,
        }
    }

    fn bfs(&mut self) -> i32 {
        let mut queue = BinaryHeap::new();
        queue.push(Item {
            v: self.start,
            t: 0,
            end: self.end,
        });
        let mut jumps = HashSet::new();
        while !queue.is_empty() {
            let item = queue.pop().unwrap();
            let (u, t) = (item.v, item.t);
            let adjacent = self.adjacent(u, t);
            for v in adjacent.iter() {
                if *v == self.end {
                    return t;
                }
                if jumps.insert((*v, t + 1)) {
                    queue.push(Item {
                        v: *v,
                        t: t + 1,
                        end: self.end,
                    });
                }
            }
        }
        unreachable!()
    }

    fn adjacent(&self, (col, row): (i32, i32), t: i32) -> Vec<(i32, i32)> {
        let adjacent = [
            (col - 1, row),
            (col, row),
            (col + 1, row),
            (col, row - 1),
            (col, row + 1),
        ];
        let result = adjacent
            .iter()
            .copied()
            .filter(|(c, r)| !self.is_perimiter((*c, *r)))
            .filter(|(c, r)| !self.is_blizzard(*c, *r, t))
            .collect();
        result
    }

    fn is_perimiter(&self, vertice: (i32, i32)) -> bool {
        let (col, row) = (vertice.0, vertice.1);
        if vertice == self.start || vertice == self.end {
            return false;
        }
        col < 1 || col >= self.width - 1 || row < 1 || row >= self.height - 1
    }

    fn is_blizzard(&self, c: i32, r: i32, t: i32) -> bool {
        for blizzard in self.blizzards.iter() {
            if (c, r) == blizzard.at(self.width, self.height, t) {
                return true;
            }
        }
        false
    }
}

pub fn solve(input: &[&str]) -> i32 {
    let mut graph = Graph::new(input);
    graph.bfs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = [
            "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
        ];

        assert_eq!(solve(&input), 18);
    }

    #[test]
    fn test_right_blizzard_at() {
        let blizzard = Blizzard {
            col: 1,
            row: 2,
            direction: '>',
        };
        assert_eq!(blizzard.at(7, 7, 5), (1, 2));
    }

    #[test]
    fn test_down_blizzard_at() {
        let blizzard = Blizzard {
            col: 4,
            row: 4,
            direction: 'v',
        };
        assert_eq!(blizzard.at(7, 7, 10), (4, 4));
    }

    #[test]
    fn test_left_blizzard_at() {
        let blizzard = Blizzard {
            col: 3,
            row: 3,
            direction: '<',
        };
        assert_eq!(blizzard.at(7, 7, 4), (4, 3));
        assert_eq!(blizzard.at(7, 7, 10), (3, 3));
        assert_eq!(blizzard.at(7, 7, 14), (4, 3));
    }

    #[test]
    fn test_up_blizzard_at() {
        let blizzard = Blizzard {
            col: 5,
            row: 1,
            direction: '^',
        };
        assert_eq!(blizzard.at(8, 6, 9), (5, 4));
    }
}
