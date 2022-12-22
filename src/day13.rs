use std::collections::VecDeque;

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

// impl Eq for Packet{}

impl Packet {
    fn from_str(input: &str) -> Self {
        let input: Vec<char> = input.chars().collect();
        let mut input: VecDeque<char> = input.iter().copied().collect();
        Packet::pop_packet(&mut input)
    }

    fn from_deque(input: &mut VecDeque<char>) -> Vec<Packet> {
        let mut result = Vec::new();
        while !input.is_empty() {
            result.push(Self::pop_packet(input));
        }
        result
    }

    fn pop_packet(input: &mut VecDeque<char>) -> Packet {
        println!("parse {:?}", input);
        let token = input.pop_front().unwrap();
        if token == '[' {
            let mut temp = VecDeque::new();
            while !input.is_empty() {
                let c = input.pop_back().unwrap();
                if c != ']' {
                    temp.push_back(c);
                } else {
                    break;
                }
            }
            let res = Packet::Composite(Packet::from_deque(input));
            input.extend(temp);
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
        println!("pop number {:?}", input);
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
}
