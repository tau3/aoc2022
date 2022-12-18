use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test_delimiter: u32,
    test_true_target: usize,
    test_false_target: usize,
}

impl Monkey {
    fn parse(input: &Vec<&str>) -> Self {
        let test_false_target = input[5].split(' ').last().unwrap().parse().unwrap();
        let test_true_target = input[4].split(' ').last().unwrap().parse().unwrap();
        let operation = Operation::parse(input[2]);
        let test_delimiter = input[3].split(' ').last().unwrap().parse().unwrap();

        let items = input[1]["  Starting items: ".len()..input.len()]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        Monkey {
            items,
            operation,
            test_delimiter,
            test_true_target,
            test_false_target,
        }
    }

    fn update_worry(&self, worry: u32) -> u32 {
        self.operation.apply(worry) / 3
    }
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
        let operation = tokens.nth_back(1).unwrap();
        match operation {
            "*" => return Operation::Mul(val),
            "+" => return Operation::Plus(val),
            _ => panic!("unknown opetaion {}", val),
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
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn parse_monkeys(input: &Vec<&str>) -> Self {
        let input: Vec<&str> = input.iter().filter(|l| !l.is_empty()).map(|x| *x).collect();
        let mut monkeys = Vec::new();
        for i in 0..input.len() / 6 {
            let part: Vec<&str> = input[i * 6..i * 6 + 6].into_iter().map(|x| *x).collect();
            let monkey = Monkey::parse(&part);
            monkeys.push(monkey);
        }
        Self { monkeys }
    }

    fn turn(&mut self) {
        for mut monkey in self.monkeys.iter_mut() {
            if let Some(worry) = monkey.items.pop_front() {
                let worry = monkey.update_worry(worry);
                let receiver = if worry % monkey.test_delimiter == 0 {
                    monkey.test_true_target
                } else {
                    monkey.test_false_target
                };
                self.monkeys[receiver].items.push_back(worry);
            }
        }
    }

    fn monkey_business(&self) -> u32 {
        0
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
        assert_eq!(solve(&input), 1123);
    }
}
