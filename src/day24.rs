use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

struct Blizzard {
    col: i32,
    row: i32,
    direction: char,
}

impl Blizzard {
    fn at(&self, width: i32, height: i32, t: i32) -> (i32, i32) {
        match self.direction {
            '>' => {
                // (A + B) mod C = (A mod C + B mod C) mod C
                let a = self.col - 1;
                let b = t;
                let c = width - 2;
                let mut res = (a + (b % c)) % c;
                res += 1;
                if res >= width {
                    res -= c;
                }
                (res, self.row)
            }
            '^' => {
                // (A - B) mod C = (A mod C - B mod C) mod C
                let a = self.row - 1;
                let b = t;
                let c = height - 2;
                let mut res = (a - (b % c)) % c;
                res += 1;
                if res <= 0 {
                    res += c;
                }
                (self.col, res)
            }
            '<' => {
                // (A - B) mod C = (A mod C - B mod C) mod C
                let a = self.col - 1;
                let b = t;
                let c = width - 2;
                let mut res = (a - (b % c)) % c;
                res += 1;
                if res <= 0 {
                    res += c;
                }
                (res, self.row)
            }
            'v' => {
                // (A + B) mod C = (A mod C + B mod C) mod C
                let a = self.row - 1;
                let b = t;
                let c = height - 2;
                let mut res = (a + (b % c)) % c;
                res += 1;
                if res >= height {
                    res -= c;
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
    blizzards: Vec<Blizzard>,
}

#[derive(PartialEq, Eq)]
struct Item {
    position: (i32, i32),
    t: i32,
    end: (i32, i32),
}

impl Item {
    fn dist_to_end(&self) -> i32 {
        self.t + (self.end.0 - self.position.0).abs() + (self.end.1 - self.position.1).abs()
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
        let start = input[0].chars().position(|val| val == '.').unwrap();
        let end = input
            .last()
            .unwrap()
            .chars()
            .position(|val| val == '.')
            .unwrap();

        let blizzards = Graph::collect_blizzards(input);
        let height = input.len() as i32;

        Self {
            start: (start as i32, 0),
            end: (end as i32, height - 1),
            width: input[0].len() as i32,
            height,
            blizzards,
        }
    }

    fn collect_blizzards(input: &[&str]) -> Vec<Blizzard> {
        let mut blizzards = Vec::new();
        for (row, row_data) in input.iter().enumerate() {
            for (col, direction) in row_data.chars().enumerate() {
                if "<>v^".contains(direction) {
                    blizzards.push(Blizzard {
                        col: col as i32,
                        row: row as i32,
                        direction,
                    });
                }
            }
        }
        blizzards
    }

    fn search(&mut self) -> i32 {
        let mut queue = BinaryHeap::new();
        self.push(&mut queue, self.start, 0);

        let mut jumps = HashSet::new();
        let mut time_to_blizzards = Vec::new();

        while !queue.is_empty() {
            let (u, t) = self.pop(&mut queue);
            let mut adjacent = self.adjacent(u);
            let blizzards = self.cached_blizzards(&mut time_to_blizzards, t);
            adjacent.retain(|position| !blizzards.contains(position));
            for v in adjacent.iter() {
                if *v == self.end {
                    return t;
                }
                if jumps.insert((*v, t + 1)) {
                    self.push(&mut queue, *v, t + 1);
                }
            }
        }
        unreachable!()
    }

    fn pop(&self, queue: &mut BinaryHeap<Item>) -> ((i32, i32), i32) {
        let result = queue.pop().unwrap();
        (result.position, result.t)
    }

    fn push(&self, queue: &mut BinaryHeap<Item>, position: (i32, i32), t: i32) {
        queue.push(Item {
            position,
            t,
            end: self.end,
        });
    }

    fn cached_blizzards<'a>(
        &'a self,
        time_to_blizzards: &'a mut Vec<HashSet<(i32, i32)>>,
        t: i32,
    ) -> &'a HashSet<(i32, i32)> {
        if time_to_blizzards.len() == t as usize {
            let blizzards = self.blizzards_at(t);
            time_to_blizzards.push(blizzards);
        }
        &time_to_blizzards[t as usize]
    }

    fn blizzards_at(&self, t: i32) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new();
        for b in self.blizzards.iter() {
            let pos = b.at(self.width, self.height, t);
            result.insert(pos);
        }
        result
    }

    fn adjacent(&self, (col, row): (i32, i32)) -> Vec<(i32, i32)> {
        let adjacent = [
            (col - 1, row),
            (col, row),
            (col + 1, row),
            (col, row - 1),
            (col, row + 1),
        ];
        let result = adjacent
            .iter()
            .filter(|(c, r)| !self.is_perimiter((*c, *r)))
            .copied()
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
}

pub fn solve(input: &[&str]) -> i32 {
    let mut graph = Graph::new(input);
    graph.search()
}

pub fn part2(input: &[&str]) -> i32 {
    let mut graph = Graph::new(input);
    let forward = graph.search();

    let (start, end) = (graph.start, graph.end);
    (graph.start, graph.end) = (end, start);
    let backward = graph.search();

    (graph.start, graph.end) = (start, end);
    let again = graph.search();

    forward + backward + again + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = [
            "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
        ];

        assert_eq!(solve(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = [
            "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
        ];

        assert_eq!(part2(&input), 54);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day24");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 232);
    }
    
    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day24");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(part2(&input), 700);
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
