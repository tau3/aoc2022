const CHECKPOINTS: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub struct Counter {
    cycle: u32,
    x: i32,
    xs: Vec<i32>,
}

impl Counter {
    fn new() -> Self {
        Self {
            cycle: 0,
            x: 1,
            xs: vec![1],
        }
    }

    fn tick(&mut self) {
        self.xs.push(self.x);
        self.cycle += 1;
    }

    fn add(&mut self, val: i32) {
        self.x += val;
    }

    pub fn result(&self) -> i32 {
        let mut result = 0;
        for i in CHECKPOINTS {
            result += (i as i32) * self.xs[i];
        }
        result
    }

    fn get(&self, i: usize) -> i32 {
        self.xs[i]
    }
}

pub fn solve(input: &Vec<String>) -> Counter {
    let mut counter = Counter::new();
    for line in input.iter() {
        if line == "noop" {
            counter.tick();
            continue;
        }
        let y: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();
        counter.tick();
        counter.tick();

        counter.add(y);
    }
    counter
}

pub fn part2(counter: &Counter) {
    let width = 40;
    let height = 6;
    let mut screen = Vec::new();
    for _ in 0..height {
        let line = vec!['.'; width];
        screen.push(line);
    }

    for i in 0..240 {
        let x = counter.get(i+1);
        let (row, col) = (i / width, i % width);
        if x - 1 == col as i32 || x == col as i32 || x + 1 == col as i32 {
            screen[row][col] = '#';
        }
    }

    for row in 0..height {
        for col in 0..width {
            print!("{}", screen[row][col]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = util::read_real_data("day10");
        assert_eq!(solve(&input).result(), 13140);
    }

    #[test]
    fn test_solve_with_real_data() {
        let input = util::read_real_data("day10_big");
        assert_eq!(solve(&input).result(), 14520);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day10_big");
        let counter = solve(&input);
        part2(&counter);
    }
}
