use std::collections::HashMap;

type Uint = u128;

enum Expr<'a> {
    Calc(&'a str, Operation, &'a str),
    Number(Uint),
}

impl<'a> Expr<'a> {
    fn decompose(&self) -> (&str, &Operation, &str) {
        match self {
            Expr::Number(_) => panic!("simple number!"),
            Expr::Calc(left, oper, right) => (left, &oper, right),
        }
    }
    fn eval(&self, registry: &HashMap<&str, Expr>) -> Uint {
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
            Expr::Calc(left, _, right) => {
                if right == &"humn" || left == &"humn" {
                    return true;
                }
                let left = registry.get(left).unwrap();
                let right = registry.get(right).unwrap();
                return left.is_depend_on_human(registry) || right.is_depend_on_human(registry);
            }
        }
    }

    fn adjust_human(&self, registry: &HashMap<&str, Expr>, operation_result: Uint) -> Uint {
        let (left, oper, right) = self.decompose();
        let left = registry.get(left).unwrap();
        let right = registry.get(right).unwrap();
        if left.is_depend_on_human(registry) {
            let right = right.eval(registry);
            let operation_result = oper.reverse_left_var(right, operation_result);
            return left.adjust_human(registry, operation_result);
        } else {
            let left = left.eval(registry);
            let operation_result = oper.reverse_right_var(left, operation_result);
            return right.adjust_human(registry, operation_result);
        }
    }
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn reverse_left_var(&self, right: Uint, operation_result: Uint) -> Uint {
        match self {
            Operation::Add => operation_result - right,
            Operation::Sub => operation_result + right,
            Operation::Mul => operation_result / right,
            Operation::Div => operation_result * right,
        }
    }

    fn reverse_right_var(&self, left: Uint, operation_result: Uint) -> Uint {
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

    fn calc(&self, left: Uint, right: Uint) -> Uint {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

pub fn solve(input: &Vec<&str>) -> Uint {
    let mut registry: HashMap<&str, Expr> = HashMap::new();
    for line in input {
        let mut name_and_expr = line.split(": ");
        let name = name_and_expr.next().unwrap();
        let expr = name_and_expr.next().unwrap();
        let expr = Expr::new(expr);
        registry.insert(name, expr);
    }

    let root = registry.get("root");
    root.unwrap().eval(&registry)
}

pub fn part2(input: &Vec<&str>) -> Uint {
    let mut registry: HashMap<&str, Expr> = HashMap::new();
    for line in input {
        let mut name_and_expr = line.split(": ");
        let name = name_and_expr.next().unwrap();
        let expr = name_and_expr.next().unwrap();
        let expr = Expr::new(expr);
        registry.insert(name, expr);
    }

    let root = registry.get("root").unwrap();
    let (left, _, right) = root.decompose();
    let left = registry.get(left).unwrap();
    let right = registry.get(right).unwrap();
    if left.is_depend_on_human(&registry) {
        let target = right.eval(&registry);
        return left.adjust_human(&registry, target);
    } else {
        let target = left.eval(&registry);
        return right.adjust_human(&registry, target);
    }
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
}
