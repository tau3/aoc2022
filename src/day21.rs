use std::collections::HashMap;

type Uint = u128;

enum Expr<'a> {
    Calc(&'a str, Operation, &'a str),
    Number(Uint),
}

impl<'a> Expr<'a> {
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
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
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
    fn test_with_real_data() {
        let input = util::read_real_data("day21");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 85616733059734);
    }
}
