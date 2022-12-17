const CHECKPOINTS:[u32;6] = [20, 60, 100, 140, 180, 220];

struct Counter{
    cycle: u32,
    x: i32,
    result : i32,
}

impl Counter {
    fn new () -> Self {
        Self {cycle:0, x:1, result:0}
    }

    fn next(&mut self){
        self.cycle +=1;
        if CHECKPOINTS.contains(&self.cycle) {
            self.result += (self.cycle as i32) * self.x;
        }
    }

    fn add(&mut self, val: i32) {
        self.x += val;
    }
}

pub fn solve(input: &Vec<String>) -> i32 {
    let mut counter = Counter::new();
    for line in input.iter() {
        if line == "noop" {
            counter.next();
            continue;
        }
        let y: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();
        counter.next();
        counter.next();

        counter.add(y);
    }
    counter.result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = util::read_real_data("day10");
        assert_eq!(solve(&input), 13140);
    }

    #[test]
    fn test_solve_with_real_data() {
        let input = util::read_real_data("day10_big");
        assert_eq!(solve(&input), 14520);
    }
}
