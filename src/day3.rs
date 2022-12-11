use std::collections::HashSet;

pub fn solve(input: &[&str]) -> usize {
    input
        .iter()
        .map(|x| find_item(x))
        .map(|x| item_to_priority(x))
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
        .expect(&format!("no intersection in {}", line));
    result.clone()
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn item_to_priority(item: char) -> usize {
    1 + ALPHABET
        .find(item)
        .expect(&format!("can't find char {}", item))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let input = [
	    "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(solve(&input), 157);
    }
}
