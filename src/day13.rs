use std::{
    cmp::{max, Ordering},
    collections::VecDeque,
};

#[derive(Debug)]
enum Packet {
    Number(u32),
    Composite(Vec<Packet>),
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        match self {
            Packet::Number(x) => Packet::Number(*x),
            Packet::Composite(x) => {
                let mut y = Vec::new();
                for item in x {
                    y.push(item.clone());
                }
                Packet::Composite(y)
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        match (self, other) {
            (Packet::Number(x), Packet::Number(y)) => x == y,
            (x @ Packet::Number(_), y @ Packet::Composite(_)) => {
                Packet::Composite(vec![x.clone()]) == *y
            }
            (y @ Packet::Composite(_), x @ Packet::Number(_)) => {
                Packet::Composite(vec![x.clone()]) == *y
            }
            (Packet::Composite(x), Packet::Composite(y)) => {
                if x.len() != y.len() {
                    return false;
                }
                for i in 0..x.len() {
                    if x[i] != y[i] {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Packet) -> Option<Ordering> {
        match (self, rhs) {
            (Packet::Number(x), Packet::Number(y)) => Some(x.cmp(y)),
            (x @ Packet::Number(_), y @ Packet::Composite(_)) => {
                Packet::Composite(vec![x.clone()]).partial_cmp(y)
            }
            (y @ Packet::Composite(_), x @ Packet::Number(_)) => {
                y.partial_cmp(&Packet::Composite(vec![x.clone()]))
            }
            (Packet::Composite(x), Packet::Composite(y)) => {
                println!("cmp {:?} and {:?}", x, y);

                let l = max(x.len(), y.len());
                for i in 0..l {
                    let c = x[i].partial_cmp(&y[i]);
                    if c != Some(Ordering::Equal) {
                        return c;
                    }
                }
                Some(x.len().cmp(&y.len()))
            }
        }
    }
}

impl From<&str> for Packet {
    fn from(input: &str) -> Self {
        Packet::from_str(input)
    }
}

impl Packet {
    fn from_str(input: &str) -> Self {
        let input: Vec<char> = input.chars().collect();
        let mut input: VecDeque<char> = input.iter().copied().collect();
        Packet::pop_packet(&mut input)
    }

    fn from_deque(input: &mut VecDeque<char>) -> Vec<Packet> {
        // println!("from deque {:?}", input.iter().collect::<String>());
        let mut result = Vec::new();
        while !input.is_empty() {
            result.push(Self::pop_packet(input));
        }
        result
    }

    fn pop_packet(input: &mut VecDeque<char>) -> Packet {
        // println!("pop_packet {:?}", input.iter().collect::<String>());
        let token = input.pop_front().unwrap();
        if token == '[' {
            let mut i = 1;
            let mut temp = VecDeque::new();
            while !input.is_empty() {
                let c = input.pop_front().unwrap();
                if c == '[' {
                    i += 1;
                    temp.push_back(c);
                } else if c == ']' {
                    i -= 1;
                    if i == 0 {
                        break;
                    }
                    temp.push_back(c);
                } else {
                    temp.push_back(c);
                }
            }
            input.pop_front();
            // println!("deep from {:?}", temp.iter().collect::<String>());
            let res = Packet::Composite(Packet::from_deque(&mut temp));
            res
        } else if token.is_numeric() {
            input.push_front(token);
            let n = Packet::pop_number(input);
            return Packet::Number(n);
        } else {
            panic!("unexpected token '{}'", token);
        }
    }

    fn pop_number(input: &mut VecDeque<char>) -> u32 {
        // println!("pop_number {:?}", input.iter().collect::<String>());
        let mut number = String::from("");
        while !input.is_empty() {
            let c = input.pop_front().unwrap();
            if c.is_numeric() {
                number.push(c);
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_list() {
        assert_eq!(
            Packet::from_str("[1,1,3,1,1]"),
            Packet::Composite(vec![
                Packet::Number(1),
                Packet::Number(1),
                Packet::Number(3),
                Packet::Number(1),
                Packet::Number(1)
            ])
        );
    }

    #[test]
    fn test_parse_list_of_lists() {
        assert_eq!(
            Packet::from_str("[[1],[2,3,4]]"),
            Packet::Composite(vec![
                Packet::Composite(vec![Packet::Number(1)]),
                Packet::Composite(vec![
                    Packet::Number(2),
                    Packet::Number(3),
                    Packet::Number(4)
                ])
            ])
        );
    }

    #[test]
    fn test_parse_composite() {
        assert_eq!(
            Packet::from_str("[[1],4]"),
            Packet::Composite(vec![
                Packet::Composite(vec![Packet::Number(1)]),
                Packet::Number(4)
            ])
        );
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(Packet::from_str("[]"), Packet::Composite(vec![]));
    }

    #[test]
    fn test_parse_composite_empty() {
        assert_eq!(
            Packet::from_str("[[[]]]"),
            Packet::Composite(vec![Packet::Composite(vec![Packet::Composite(vec![])])])
        );
    }

    #[test]
    fn test_parse_hierarchy() {
        assert_eq!(
            Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            Packet::Composite(vec![
                Packet::Number(1),
                Packet::Composite(vec![
                    Packet::Number(2),
                    Packet::Composite(vec![
                        Packet::Number(3),
                        Packet::Composite(vec![
                            Packet::Number(4),
                            Packet::Composite(vec![
                                Packet::Number(5),
                                Packet::Number(6),
                                Packet::Number(7)
                            ])
                        ])
                    ])
                ]),
                Packet::Number(8),
                Packet::Number(9)
            ])
        );
    }

    #[test]
    fn test_cmp() {
        let left = Packet::from_str("[1,1,3,1,1]");
        let right = Packet::from_str("[1,1,5,1,1]");
        assert!(left < right);

        let left = Packet::from_str("[[1],[2,3,4]]");
        let right = Packet::from_str("[[1],4]");
        assert!(left < right);

        let left = Packet::from_str("[9]");
        let right = Packet::from_str("[[8,7,6]]");
        assert!(left > right);

        let left = Packet::from_str("[[4,4],4,4]");
        let right = Packet::from_str("[[4,4],4,4,4]");
        assert!(left < right);

        let left = Packet::from_str("[7,7,7,7]");
        let right = Packet::from_str("[7,7,7]");
        assert!(left > right);

        let left = Packet::from_str("[]");
        let right = Packet::from_str("[3]");
        assert!(left < right);

        let left = Packet::from_str("[[[]]]");
        let right = Packet::from_str("[[]]");
        assert!(left > right);

        let left = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(left > right);
    }
}
