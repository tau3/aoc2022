use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Hash, Eq, PartialEq)]
struct Blizzard {
    col: i32,
    row: i32,
    direction: Direction,
}

impl Blizzard {
    fn at(&self, width: i32, height: i32, t: i32) -> Vertice {
        match self.direction {
            Direction::Up => Vertice(self.col, (self.row - t) % (height - 2)),
            Direction::Down => Vertice(self.col, (self.row + t) % (height - 2)),
            Direction::Left => Vertice((self.col - t) % (width - 2), self.row),
            Direction::Right => Vertice((self.col + t) % (width - 2), self.row),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(direction: char) -> Self {
        match direction {
            'v' => Direction::Down,
            '>' => Direction::Right,
            '^' => Direction::Up,
            '<' => Direction::Left,
            _ => panic!("unknown direction '{}'", direction),
        }
    }
}

struct Graph {
    colors: HashMap<Vertice, char>,
    distance: HashMap<Vertice, i32>,
    parent: HashMap<Vertice, Option<Vertice>>,
    start: Vertice,
    end: Vertice,
    width: i32,
    height: i32,
    blizzards: HashSet<Blizzard>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Vertice(i32, i32);

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
        for r in 0..input.len() {
            for (c, x) in input[r].chars().enumerate() {
                if "<>v^".contains(x) {
                    blizzards.insert(Blizzard {
                        col: c as i32,
                        row: r as i32,
                        direction: Direction::parse(x),
                    });
                }
            }
        }

        Self {
            colors: HashMap::new(),
            distance: HashMap::new(),
            parent: HashMap::new(),
            start: Vertice(start as i32, 0),
            end: Vertice(end as i32, input.len() as i32 - 1),
            width: input[0].len() as i32,
            height: input.len() as i32,
            blizzards,
        }
    }

    fn bfs(&mut self) -> i32 {
        for u in self.vertices() {
            self.colors.insert(u, 'w');
            self.distance.insert(u, i32::MAX);
            self.parent.insert(u, None);
        }

        self.colors.insert(self.start, 'g');
        self.distance.insert(self.start, 0);
        self.parent.insert(self.start, None);

        let mut queue = VecDeque::from([(self.start.clone(), 0)]);
        while !queue.is_empty() {
            let (u, t) = queue.pop_front().unwrap();
            let adjacent = self.adjacent(&u, t);
            for v in adjacent.iter() {
                if *v == self.end {
                    return t + 1;
                }
                println!("v={:?}", v);
                if self.colors[&v] == 'w' {
                    self.colors.insert(*v, 'g');
                    self.distance.insert(*v, 1);
                    self.parent.insert(*v, Some(u.clone()));
                    queue.push_back((*v, t + 1));
                }
            }
            self.colors.insert(u.clone(), 'b');
        }
        unreachable!()
    }

    fn adjacent(&self, vertice: &Vertice, t: i32) -> Vec<Vertice> {
        let (col, row) = (vertice.0, vertice.1);
        let adjacent = [
            (col - 1, row),
            (col, row),
            (col + 1, row),
            (col, row - 1),
            (col, row + 1),
        ];

        println!("w={}", self.width);
        adjacent
            .iter()
            .copied()
            .filter(|(c, r)| {
                *c > 0
                    && *r > 0
                    && *c < self.width - 1
                    && (r < &self.height || self.end == Vertice(*c, *r))
            })
            .filter(|(c, r)| !self.is_blizzard(*c, *r, t))
            .map(|(c, r)| Vertice(c, r))
            .collect()
    }

    fn is_blizzard(&self, c: i32, r: i32, t: i32) -> bool {
        for blizzard in self.blizzards.iter() {
            if Vertice(c, r) == blizzard.at(self.width, self.height, t) {
                return true;
            }
        }
        return false;
    }

    fn vertices(&self) -> Vec<Vertice> {
        let mut result = Vec::new();
        for c in 0..self.width {
            for r in 0..self.height {
                result.push(Vertice(c, r));
            }
        }
        // println!("all: {:?}", result);
        result
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
            direction: Direction::Right,
        };
        assert_eq!(blizzard.at(7, 7, 5), Vertice(1, 2));
    }

    #[test]
    fn test_down_blizzard_at() {
        let blizzard = Blizzard {
            col: 4,
            row: 4,
            direction: Direction::Down,
        };
        assert_eq!(blizzard.at(7, 7, 3), Vertice(4, 2));
    }
}
