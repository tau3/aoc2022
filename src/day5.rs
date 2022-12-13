use std::collections::VecDeque;

pub fn solve<S: AsRef<str>>(input: Vec<S>, crane: &Crane) -> String {
    let mut input: VecDeque<S> = input.into();
    let (initial, moves) = parse(&mut input);
    let mut stacks = parse_initial(&initial);

    for move_ in moves {
        let move_ = parse_move(&move_);
        crane(&mut stacks, move_);
    }

    headers(stacks)
}

fn headers(stacks: Vec<VecDeque<char>>) -> String {
    let mut result = String::new();
    for mut stack in stacks {
        let maybe_item = stack.pop_front();
        if let Some(item) = maybe_item {
            result.push(item);
        }
    }
    result
}

fn parse<S>(input: &mut VecDeque<S>) -> (Vec<String>, &VecDeque<S>)
where
    S: AsRef<str>,
{
    let mut initial = Vec::new();

    while let Some(item) = input.pop_front() {
        if !item.as_ref().is_empty() {
            initial.push(item.as_ref().to_owned());
        } else {
            break;
        }
    }

    (initial, input)
}

fn parse_initial<S: AsRef<str>>(initial: &Vec<S>) -> Vec<VecDeque<char>> {
    let initial_len = initial.len();
    let max_count = count(&initial[initial_len - 1]);
    let mut result = vec![VecDeque::new(); max_count];
    for line in initial[0..initial.len() - 1].iter() {
        let line = line.as_ref();
        let chars: Vec<char> = line.chars().collect();
        let count = count(line);
        for (i, item) in result.iter_mut().enumerate().take(count) {
            let pos = 4 * i + 1;
            if chars[pos] != ' ' {
                item.push_back(chars[pos]);
            }
        }
    }

    result
}

fn count<S: AsRef<str>>(line: S) -> usize {
    (line.as_ref().len() + 1) / 4
}

fn parse_move(move_: &dyn AsRef<str>) -> (u32, u32, u32) {
    let mut tokens = move_.as_ref().split(' ');
    let amount = tokens.nth(1).unwrap();
    let from = tokens.nth(1).unwrap();
    let to = tokens.nth(1).unwrap();
    (
        amount.parse().unwrap(),
        from.parse().unwrap(),
        to.parse().unwrap(),
    )
}

type Crane = dyn Fn(&mut [VecDeque<char>], (u32, u32, u32));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn apply_move(stacks: &mut [VecDeque<char>], (amount, from, to): (u32, u32, u32)) {
        for _ in 0..amount {
            pop_once(stacks, (from, to));
        }
    }

    fn pop_once(stacks: &mut [VecDeque<char>], (from, to): (u32, u32)) {
        let item = stacks[(from - 1) as usize].pop_front().unwrap();
        stacks[(to - 1) as usize].push_front(item);
    }

    fn new_crane(stacks: &mut [VecDeque<char>], (amount, from, to): (u32, u32, u32)) {
        let source = stacks.get_mut((from - 1) as usize).unwrap();
        let slice: Vec<char> = source
            .drain(0..(amount) as usize)
            .into_iter()
            .rev()
            .collect();
        let target = stacks.get_mut((to - 1) as usize).unwrap();
        slice.iter().for_each(|i| target.push_front(*i));
    }

    #[test]
    fn test_parse_initial() {
        let initial = vec!["    [D]", "[N] [C]", "[Z] [M] [P]", " 1   2   3 "];
        let actual = parse_initial(&initial);
        assert_eq!(actual.len(), 3);

        let second: &VecDeque<char> = &actual[1];
        let expected: VecDeque<char> = vec!['D', 'C', 'M'].into();
        assert_eq!(second, &expected);
    }

    #[test]
    fn test_solve() {
        let input = vec![
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        assert_eq!(solve(input, &apply_move), "CMZ");
    }

    #[test]
    fn test_new_crane() {
        let mut stacks: Vec<VecDeque<char>> =
            vec![['D', 'N', 'Z'].into(), ['C', 'M'].into(), ['P'].into()];
        new_crane(&mut stacks, (3, 1, 3));
        let expected: Vec<VecDeque<char>> =
            vec![[].into(), ['C', 'M'].into(), ['D', 'N', 'Z', 'P'].into()];
        assert_eq!(stacks, expected);
    }

    #[test]
    fn test_solve_with_new_crane() {
        let input = vec![
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        assert_eq!(solve(input, &new_crane), "MCD");
    }

    #[test]
    fn test_new_crane_with_real_data() {
        let input = util::read_real_data("day5");
        assert_eq!(solve(input, &new_crane), "WJVRLSJJT");
    }

    #[test]
    fn test_real_data() {
        let input = util::read_real_data("day5");
        assert_eq!(solve(input, &apply_move), "DHBJQJCCW");
    }
}
