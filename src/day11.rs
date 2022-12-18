use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test_delimiter: u32,
    test_true_target: usize,
    test_false_target: usize,
    count: u32,
}

impl Monkey {
    fn parse(input: &Vec<&str>) -> Self {
        let test_false_target = last_u32(input[5]) as usize;
        let test_true_target = last_u32(input[4]) as usize;
        let operation = Operation::parse(input[2]);
        let test_delimiter = last_u32(input[3]);

        let items = input[1]["  Starting items: ".len()..input[1].len()]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        Monkey {
            items,
            operation,
            test_delimiter,
            test_true_target,
            test_false_target,
            count: 0,
        }
    }

    fn take_item(&mut self, worry: u32) {
        self.items.push_back(worry);
    }

    fn update_worry(&mut self, worry: u32) -> u32 {
        self.count += 1;
        self.operation.apply(worry) / 3
    }

    fn receiver(&self, worry: u32) -> usize {
        if worry % self.test_delimiter == 0 {
            self.test_true_target
        } else {
            self.test_false_target
        }
    }
}

fn last_u32(line: &str) -> u32 {
    line.split(' ').last().unwrap().parse().unwrap()
}

enum Operation {
    Mul(u32),
    MulOld,
    Plus(u32),
}

impl Operation {
    fn parse(line: &str) -> Self {
        if line.ends_with("* old") {
            return Operation::MulOld;
        }

        let mut tokens = line.split(' ').rev();
        let val = tokens.next().unwrap().parse().unwrap();
        let operation = tokens.next().unwrap();
        match operation {
            "*" => return Operation::Mul(val),
            "+" => return Operation::Plus(val),
            _ => panic!("unknown operaation {}", val),
        }
    }

    fn apply(&self, worry: u32) -> u32 {
        match self {
            Operation::Mul(x) => worry * x,
            Operation::MulOld => worry * worry,
            Operation::Plus(x) => worry + x,
        }
    }
}

struct Monkeys {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
}

impl Monkeys {
    fn parse_monkeys(input: &Vec<&str>) -> Self {
        let input: Vec<&str> = input.iter().filter(|l| !l.is_empty()).map(|x| *x).collect();
        let mut monkeys = Vec::new();
        for i in 0..input.len() / 6 {
            let part: Vec<&str> = input[i * 6..i * 6 + 6].into_iter().map(|x| *x).collect();
            let monkey = Monkey::parse(&part);
            monkeys.push(Rc::new(RefCell::new(monkey)));
        }
        Self { monkeys }
    }

    fn turn(&mut self) {
        for monkey in self.monkeys.iter() {
	    // println!("Monkey _:");
            let mut monkey = monkey.borrow_mut();
            while let Some(worry) = monkey.items.pop_front() {
		// println!("  Monkey inspects an item with a worry level of {}.", worry);
                let worry = monkey.update_worry(worry);
		// println("    Worry level is multiplied to {}.", worry);
                let receiver = monkey.receiver(worry);
		// println!("    Monkeys gets bored with item. Worry level is divided by 3 to {}.", worry);
		// println!("    Iter with worry level {} is thrown to monkey {}", worry, receiver);
                self.monkeys[receiver].borrow_mut().take_item(worry);
            }
        }
    }

    fn monkey_business(&self) -> u32 {
        let mut counts: Vec<u32> = self.monkeys.iter().map(|m| m.borrow().count).collect();
        counts.sort_by(|a, b| b.cmp(a));
        counts[0] * counts[1]
    }
}

pub fn solve(input: &Vec<&str>) -> u32 {
    let mut monkeys = Monkeys::parse_monkeys(input);
    for _ in 0..20 {
        monkeys.turn();
    }

    monkeys.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ];
        assert_eq!(solve(&input), 10605);
    }

    #[test]
    fn test_solve_with_real_data() {
        let data = util::read_real_data("day11");
        let data = data.iter().map(|line|line.as_str()).collect();
        assert_eq!(solve(&data), 54752);
    }
}       
