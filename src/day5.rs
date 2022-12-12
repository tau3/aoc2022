use std::collections::VecDeque;

pub fn solve(input: Vec<&str>) -> String {
    let input = input.into();
    let (initial, moves) = parse(&input);
    let mut stacks = parse_initial(&initial);

    for move_ in moves {
        let move_ = parse_move(&move_);
        apply_move(&mut stacks, move_);
    }

    headers(stacks)
}

fn headers(stacks: [VecDeque<char>; POSITIONS.len()]) -> String {
    let mut result = String::new();
    for mut stack in stacks {
        let item = stack.pop_front().unwrap();
        result.push(item);
    }
    result
}

fn parse<'a>(input: &'a VecDeque<&str>) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut initial = Vec::new();
   
    while let Some(item) = input.pop_front() {
        if !item.is_empty() {
            initial.push(item);
        } else {
            break;
        }
    }

    (initial, (*input).into())
}

const POSITIONS: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];

fn parse_initial(initial: &Vec<&str>) -> [VecDeque<char>; POSITIONS.len()] {
    let mut result = vec![VecDeque::new(); POSITIONS.len()];
    for line in initial {
        for i in POSITIONS {
            let chars: Vec<char> = line.chars().collect();
            if chars[i] != ' ' {
                result[i].push_front(chars[i]);
            }
        }
    }

    result.try_into().unwrap()
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

fn apply_move(
    stacks: &mut [VecDeque<char>; POSITIONS.len()],
    (amount, from, to): (u32, u32, u32),
) {
    let mut source = &mut stacks[(from - 1) as usize];
    let mut target = &mut stacks[(to - 1) as usize];
    for _ in 0..amount {
        let item = source.pop_front().unwrap();
        target.push_front(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
