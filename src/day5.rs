use std::collections::VecDeque;

pub fn solve(input: Vec<&str>) -> String {
    let mut input = input.into();
    let (initial, moves) = parse(&mut input);
    let mut stacks = parse_initial(&initial);

    for move_ in moves {
        let move_ = parse_move(&move_);
        apply_move(&mut stacks, move_);
    }

    headers(stacks)
}

fn headers(stacks: Vec<VecDeque<char>>) -> String {
    let mut result = String::new();
    for mut stack in stacks {
        let item = stack.pop_front().unwrap();
        result.push(item);
    }
    result
}

fn parse<'a>(input: &'a mut VecDeque<&str>) -> (Vec<&'a str>, &'a VecDeque<&'a str>) {
    let mut initial = Vec::new();

    while let Some(item) = input.pop_front() {
        if !item.is_empty() {
            initial.push(item);
        } else {
            break;
        }
    }

    (initial, input)
}

fn parse_initial(initial: &Vec<&str>) -> Vec<VecDeque<char>> {
    let initial_len = initial.len();
    let max_count = count(&initial[initial_len - 1]);
    let mut result = vec![VecDeque::new(); max_count];
    for line in initial[0..initial.len() - 1].iter() {
        let chars: Vec<char> = line.chars().collect();
        let count = count(&line);
        for i in 0..count {
            let pos = 4 * i + 1;
            if chars[pos] != ' ' {
                result[i].push_back(chars[pos]);
            }
        }
    }

    result.try_into().unwrap()
}

fn count(line: &str) -> usize {
    (line.len() + 1) / 4
}

fn parse_move(move_: &str) -> (u32, u32, u32) {
    let mut tokens = move_.split(" ");
    let amount = tokens.nth(1).unwrap();
    let from = tokens.nth(1).unwrap();
    let to = tokens.nth(1).unwrap();
    (
        amount.parse().unwrap(),
        from.parse().unwrap(),
        to.parse().unwrap(),
    )
}

fn apply_move(stacks: &mut Vec<VecDeque<char>>, (amount, from, to): (u32, u32, u32)) {
    for _ in 0..amount {
        pop_once(stacks, (from, to));
    }
}

fn pop_once(stacks: &mut Vec<VecDeque<char>>, (from, to): (u32, u32)) {
    let item = stacks[(from - 1) as usize].pop_front().unwrap();
    stacks[(to - 1) as usize].push_front(item);
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(solve(input), "CMZ");
    }
}
