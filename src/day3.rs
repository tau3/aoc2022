use std::collections::HashSet;

pub fn solve<S: AsRef<str>>(input: &[S]) -> usize {
    input
        .iter()
        .map(|items| items.as_ref())
        .map(find_item)
        .map(item_to_priority)
        .sum()
}

fn find_item(line: &str) -> char {
    let size = line.len();
    let middle = size / 2;
    let left = &line[0..middle];
    let left: HashSet<char> = left.chars().collect();

    let right = &line[middle..size];
    let right: HashSet<char> = right.chars().collect();

    let mut intersection = left.intersection(&right);
    let result = intersection
        .next()
        .unwrap_or_else(|| panic!("no intersection in {}", line));
    *result
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn item_to_priority(item: char) -> usize {
    1 + ALPHABET
        .find(item)
        .unwrap_or_else(|| panic!("can't find char {}", item))
}

pub fn solve_part2<S: AsRef<str>>(input: &Vec<S>) -> usize {
    let mut result = 0;
    for i in 0..input.len() / 3 {
        let badge = badge(&[
            input[3 * i].as_ref(),
            input[3 * i + 1].as_ref(),
            input[3 * i + 2].as_ref(),
        ]);
        let score = item_to_priority(badge);
        result += score;
    }
    result
}

fn badge(group: &[&str; 3]) -> char {
    let mut result: HashSet<char> = group[0].chars().collect();
    for item in group.iter().skip(1) {
        let distint: HashSet<char> = item.chars().collect();
        let intersection: HashSet<&char> = result.intersection(&distint).collect();
        result = intersection.iter().copied().copied().collect();
    }

    assert!(result.len() == 1, "too long intersection: {:?}", result);
    result
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("expect to have badge in group {:?}", group))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_find_item() {
        let actual = find_item("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(actual, 'p');
    }

    #[test]
    fn test_priority() {
        assert_eq!(item_to_priority('v'), 22);
        assert_eq!(item_to_priority('P'), 42);
    }

    #[test]
    fn test_solve() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(solve(&input), 157);
    }

    #[test]
    fn test_solve_with_real_data() {
        let input = util::read_real_data("day3");
        let actual = solve(&input);
        assert_eq!(actual, 7811);
    }

    #[test]
    fn test_badge() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        assert_eq!(badge(&input), 'r');
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day3");
        let actual = solve_part2(&input);
        assert_eq!(actual, 2639);
    }
}
