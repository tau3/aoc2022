use std::collections::HashMap;

type Int = i128;

enum Expr<'a> {
    Calc(&'a str, Operation, &'a str),
    Number(Int),
}

impl<'a> Expr<'a> {
    fn eval(&self, registry: &HashMap<&str, Expr>) -> Int {
        match self {
            Expr::Number(x) => *x,
            Expr::Calc(left, operation, right) => {
                let left = registry.get(left).unwrap();
                let right = registry.get(right).unwrap();
                operation.calc(left.eval(registry), right.eval(registry))
            }
        }
    }

    fn new(expr: &'a str) -> Expr {
        if expr.contains(' ') {
            let mut tokens = expr.split(' ');
            let left = tokens.next().unwrap();
            let operation = tokens.next().unwrap();
            let operation = Operation::new(operation);
            let right = tokens.next().unwrap();
            return Self::Calc(left, operation, right);
        }
        Self::Number(expr.parse().unwrap())
    }

    fn is_depend_on_human(&self, registry: &HashMap<&str, Expr>) -> bool {
        match self {
            Expr::Number(_) => false,
            Expr::Calc("humn", _, _) => true,
            Expr::Calc(_, _, "humn") => true,
            Expr::Calc(left, _, right) => {
                let left = registry.get(left).unwrap();
                let right = registry.get(right).unwrap();
                left.is_depend_on_human(registry) || right.is_depend_on_human(registry)
            }
        }
    }

    fn adjust_human(&self, registry: &HashMap<&str, Expr>, operation_result: Int) -> Int {
        match self {
            Expr::Number(val) => panic!("number {}, target {}", val, operation_result),
            Expr::Calc("humn", oper, right_name) => {
                let right = registry.get(right_name).unwrap();
                oper.reverse_left_var(right.eval(registry), operation_result)
            }
            Expr::Calc(left_name, oper, "humn") => {
                let left = registry.get(left_name).unwrap();
                oper.reverse_right_var(left.eval(registry), operation_result)
            }
            Expr::Calc(left_name, oper, right_name) => {
                let left = registry.get(left_name).unwrap();
                let right = registry.get(right_name).unwrap();
                if left.is_depend_on_human(registry) {
                    let right = right.eval(registry);
                    let operation_result = oper.reverse_left_var(right, operation_result);
                    left.adjust_human(registry, operation_result)
                } else {
                    let left = left.eval(registry);
                    let operation_result = oper.reverse_right_var(left, operation_result);
                    right.adjust_human(registry, operation_result)
                }
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn reverse_left_var(&self, right: Int, operation_result: Int) -> Int {
        match self {
            Operation::Add => operation_result - right,
            Operation::Sub => operation_result + right,
            Operation::Mul => operation_result / right,
            Operation::Div => operation_result * right,
        }
    }

    fn reverse_right_var(&self, left: Int, operation_result: Int) -> Int {
        match self {
            Operation::Add => operation_result - left,
            Operation::Sub => left - operation_result,
            Operation::Mul => operation_result / left,
            Operation::Div => left / operation_result,
        }
    }

    fn new(expr: &str) -> Self {
        match expr {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => panic!("{} is not supported", expr),
        }
    }

    fn calc(&self, left: Int, right: Int) -> Int {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

pub fn solve(input: &Vec<&str>) -> Int {
    let registry = parse_registry(input);
    let root = registry.get("root");
    root.unwrap().eval(&registry)
}

fn parse_registry<'a>(input: &'a Vec<&'a str>) -> HashMap<&'a str, Expr> {
    let mut registry: HashMap<&str, Expr> = HashMap::new();
    for line in input {
        let mut name_and_expr = line.split(": ");
        let name = name_and_expr.next().unwrap();
        let expr = name_and_expr.next().unwrap();
        let expr = Expr::new(expr);
        registry.insert(name, expr);
    }
    registry
}

pub fn part2(input: &Vec<&str>) -> Int {
    let registry = parse_registry(input);

    let root = registry.get("root").unwrap();
    if let Expr::Calc(left_name, _, right_name) = root {
        let left = registry.get(left_name).unwrap();
        let right = registry.get(right_name).unwrap();
        if left.is_depend_on_human(&registry) {
            let target = right.eval(&registry);
            return left.adjust_human(&registry, target);
        } else {
            let target = left.eval(&registry);
            return right.adjust_human(&registry, target);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        assert_eq!(solve(&input), 152);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        assert_eq!(part2(&input), 301);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day21");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 85616733059734);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day21");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(part2(&input), 3560324848168);
    }
}
