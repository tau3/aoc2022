use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type Worry = u64;

struct Monkey {
    items: VecDeque<Worry>,
    operation: Operation,
    divider: u32,
    test_true_target: usize,
    test_false_target: usize,
    count: u32,
}

impl Monkey {
    fn parse(input: &[&str]) -> Self {
        let test_false_target = last_u32(input[5]) as usize;
        let test_true_target = last_u32(input[4]) as usize;
        let operation = Operation::parse(input[2]);
        let test_delimiter = last_u32(input[3]);

        let items = input[1]["  Starting items: ".len()..input[1].len()]
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        Monkey {
            items,
            operation,
            divider: test_delimiter,
            test_true_target,
            test_false_target,
            count: 0,
        }
    }

    fn accept_item(&mut self, worry: Worry) {
        self.items.push_back(worry);
    }

    fn update_worry(&mut self, worry: Worry) -> Worry {
        self.count += 1;
        self.operation.apply(worry)
    }

    fn receiver(&self, worry: Worry) -> usize {
        if worry % (self.divider as Worry) == 0 {
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
    Mul(Worry),
    MulOld,
    Plus(Worry),
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
            "*" => Operation::Mul(val),
            "+" => Operation::Plus(val),
            _ => panic!("unknown operation {}", val),
        }
    }

    fn apply(&self, worry: Worry) -> Worry {
        match self {
            Operation::Mul(x) => worry * x,
            Operation::MulOld => worry * worry,
            Operation::Plus(x) => worry + x,
        }
    }
}

struct Monkeys {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
    cooldown: u32,
    base: u32,
}

impl Monkeys {
    fn parse_monkeys(input: &[&str], cooldown: u32) -> Self {
	let block_size = 6;
        let input: Vec<&str> = input.iter().filter(|line| !line.is_empty()).copied().collect();
        let mut monkeys = Vec::new();
	let blocks = input.len()/6;
        for i in 0..blocks {
            let part: Vec<&str> = input[i * block_size..i * block_size + block_size].to_vec();
            let monkey = Monkey::parse(&part);
            monkeys.push(Rc::new(RefCell::new(monkey)));
        }
        let base = monkeys.iter().map(|monkey| monkey.borrow().divider).product();
        Self {
            monkeys,
            cooldown,
            base,
        }
    }

    fn turn(&mut self) {
        for monkey in self.monkeys.iter() {
            let mut monkey = monkey.borrow_mut();
            while let Some(worry) = monkey.items.pop_front() {
                let worry =
                    (monkey.update_worry(worry) / (self.cooldown as Worry)) % (self.base as Worry);
                let receiver = monkey.receiver(worry);
		let mut receiver = self.monkeys[receiver].borrow_mut();
		receiver.accept_item(worry);
            }
        }
    }

    fn monkey_business(&self) -> u128 {
        let mut counts: Vec<u32> = self.monkeys.iter().map(|monkey| monkey.borrow().count).collect();
        counts.sort_by(|a, b| b.cmp(a));
        (counts[0] as u128) * (counts[1] as u128)
    }
}

pub fn solve(input: &[&str], rounds: u32, cooldown: u32) -> u128 {
    let mut monkeys = Monkeys::parse_monkeys(input, cooldown);
    for _ in 0..rounds {
        monkeys.turn();
    }

    monkeys.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part2() {
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
        assert_eq!(solve(&input, 10000, 1), 2713310158);
    }

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
        assert_eq!(solve(&input, 20, 3), 10605);
    }

    #[test]
    fn test_solve_with_real_data() {
        let data = util::read_real_data("day11");
        let data: Vec<&str> = data.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&data, 20, 3), 54752);
    }

    #[test]
    fn test_solve_part2() {
        let data = util::read_real_data("day11");
        let data: Vec<&str> = data.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&data, 10000, 1), 13606755504);
    }
}
