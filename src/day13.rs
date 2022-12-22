use std::{
    cmp::{min, Ordering},
    collections::VecDeque,
};

#[derive(Debug)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        match self {
            Packet::Number(x) => Packet::Number(*x),
            Packet::List(list) => {
                let mut result = Vec::new();
                for item in list {
                    result.push(item.clone());
                }
                Packet::List(result)
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        match (self, other) {
            (Packet::Number(x), Packet::Number(y)) => x == y,
            (x @ Packet::Number(_), y @ Packet::List(_)) => Packet::List(vec![x.clone()]) == *y,
            (y @ Packet::List(_), x @ Packet::Number(_)) => Packet::List(vec![x.clone()]) == *y,
            (Packet::List(x), Packet::List(y)) => {
                if x.len() != y.len() {
                    return false;
                }
                for i in 0..x.len() {
                    if x[i] != y[i] {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Packet) -> Option<Ordering> {
        match (self, rhs) {
            (Packet::Number(x), Packet::Number(y)) => Some(x.cmp(y)),
            (x @ Packet::Number(_), y @ Packet::List(_)) => {
                Packet::List(vec![x.clone()]).partial_cmp(y)
            }
            (y @ Packet::List(_), x @ Packet::Number(_)) => {
                y.partial_cmp(&Packet::List(vec![x.clone()]))
            }
            (Packet::List(x), Packet::List(y)) => {
                let min_length = min(x.len(), y.len());
                for i in 0..min_length {
                    let comparison = x[i].partial_cmp(&y[i]);
                    if comparison != Some(Ordering::Equal) {
                        return comparison;
                    }
                }
                Some(x.len().cmp(&y.len()))
            }
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, rhs: &Packet) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

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
        let token = input.pop_front().unwrap();
        if token == '[' {
            Packet::pop_list(input)
        } else if token.is_numeric() {
            input.push_front(token);
            let n = Packet::pop_number(input);
            Packet::Number(n)
        } else {
            panic!("unexpected token '{}'", token);
        }
    }

    fn pop_list(input: &mut VecDeque<char>) -> Packet {
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
        Packet::List(Packet::from_deque(&mut temp))
    }

    fn pop_number(input: &mut VecDeque<char>) -> u32 {
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

pub fn solve(input: &Vec<&str>) -> usize {
    let mut result = 0;

    let block_count = (input.len() + 1) / 3;
    for i in 0..block_count {
        let left = Packet::from_str(input[i * 3]);
        let right = Packet::from_str(input[i * 3 + 1]);
        if left < right {
            result += i + 1;
        }
    }
    result
}

pub fn part2(input: &[&str]) -> usize {
    let mut packets: Vec<Packet> = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::from_str(line))
        .collect();

    let two = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    packets.push(six.clone());
    packets.push(two.clone());

    packets.sort();

    let six_pos = 1 + packets.iter().position(|x| *x == six).unwrap();
    let two_pos = 1 + packets.iter().position(|x| *x == two).unwrap();
    six_pos * two_pos
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_parse_simple_list() {
        assert_eq!(
            Packet::from_str("[1,1,3,1,1]"),
            Packet::List(vec![
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
            Packet::List(vec![
                Packet::List(vec![Packet::Number(1)]),
                Packet::List(vec![
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
            Packet::List(vec![
                Packet::List(vec![Packet::Number(1)]),
                Packet::Number(4)
            ])
        );
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(Packet::from_str("[]"), Packet::List(vec![]));
    }

    #[test]
    fn test_parse_composite_empty() {
        assert_eq!(
            Packet::from_str("[[[]]]"),
            Packet::List(vec![Packet::List(vec![Packet::List(vec![])])])
        );
    }

    #[test]
    fn test_parse_hierarchy() {
        assert_eq!(
            Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            Packet::List(vec![
                Packet::Number(1),
                Packet::List(vec![
                    Packet::Number(2),
                    Packet::List(vec![
                        Packet::Number(3),
                        Packet::List(vec![
                            Packet::Number(4),
                            Packet::List(vec![
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

    #[test]
    fn test_solve() {
        let input = vec![
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ];

        assert_eq!(solve(&input), 13);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day13");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 5808);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ];

        assert_eq!(part2(&input), 140);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day13");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(part2(&input), 22713);
    }
}
