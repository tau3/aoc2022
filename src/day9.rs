use std::collections::HashSet;

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head == tail {
        return tail;
    }

    if is_adjacent(head, tail) {
        return tail;
    }

    let (xt, yt) = tail;
    let (xh, yh) = head;

    if xt == xh {
        return (xt, yt + (yh - yt) / (yh - yt).abs());
    }

    if yt == yh {
        return (xt + (xh - xt) / (xh - xt).abs(), yt);
    }

    (
        xt + (xh - xt) / (xh - xt).abs(),
        yt + (yh - yt) / (yh - yt).abs(),
    )
}

fn is_adjacent(head: (i32, i32), tail: (i32, i32)) -> bool {
    let (x, y) = head;
    let around = [
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
    ];

    around.contains(&tail)
}

fn parse_head_moves(input: &Vec<&str>) -> String {
    let mut result = String::from("");
    for line in input {
        let mut tokens = line.split(' ');
        let direction = tokens.next().unwrap().chars().next().unwrap();
        let count = tokens.next().unwrap().parse().unwrap();
        for _ in 0..count {
            result.push(direction);
        }
    }
    result
}

pub fn solve(input: &Vec<&str>) -> usize {
    let moves = parse_head_moves(input);
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut result = HashSet::new();
    for move_ in moves.chars() {
        head = move_head(head, move_);
        tail = move_tail(head, tail);
        result.insert(tail);
    }
    result.len()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let moves = parse_head_moves(input);
    let mut rope = [(0, 0); 10];
    let mut result = HashSet::new();
    for move_ in moves.chars() {
        rope[0] = move_head(rope[0], move_);
        for i in 1..rope.len() {
            rope[i] = move_tail(rope[i - 1], rope[i]);
        }
        result.insert(rope[9]);
    }
    result.len()
}

fn move_head((x, y): (i32, i32), move_: char) -> (i32, i32) {
    match move_ {
        'R' => (x + 1, y),
        'U' => (x, y + 1),
        'L' => (x - 1, y),
        'D' => (x, y - 1),
        _ => panic!("invalid move: {}", move_),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_move_tail() {
        assert_eq!(move_tail((5, 5), (5, 5)), (5, 5));
        assert_eq!(move_tail((1, 0), (0, 0)), (0, 0));
        assert_eq!(move_tail((0, 5), (0, 2)), (0, 3));
        assert_eq!(move_tail((0, 0), (3, 0)), (2, 0));
        assert_eq!(move_tail((2, 3), (1, 1)), (2, 2));
        assert_eq!(move_tail((3, 2), (1, 1)), (2, 2));
    }

    #[test]
    fn test_parse_head_moves() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        assert_eq!(parse_head_moves(&input), "RRRRUUUULLLDRRRRDLLLLLRR");
    }

    #[test]
    fn test_solve() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        assert_eq!(solve(&input), 13);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day9");
        let input = input.iter().map(|line| line.as_str()).collect();

        assert_eq!(solve(&input), 6486);
    }

    #[test]
    fn test_part2() {
        let input = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        assert_eq!(part2(&input), 36);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day9");
        let input = input.iter().map(|line| line.as_str()).collect();

        assert_eq!(part2(&input), 2678);
    }
}
