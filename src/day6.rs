use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve(input: &str, window_size: usize) -> usize {
    let mut result: usize = window_size;
    let mut window: VecDeque<char> = input.chars().take(window_size).collect();
    while !is_marker(&window, window_size) {
        window.pop_front();
        window.push_back(input.chars().nth(result).unwrap());
        result += 1;
    }
    result
}

fn is_marker(window: &VecDeque<char>, window_size: usize) -> bool {
    let distinct: HashSet<&char> = window.iter().collect();
    distinct.len() == window_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    const MARKER_SIZE: usize = 4;
    const MESSAGE_SIZE: usize = 14;

    #[test]
    fn test_solve() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", MARKER_SIZE), 7);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", MARKER_SIZE), 5);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", MARKER_SIZE), 6);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", MARKER_SIZE), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", MARKER_SIZE), 11);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day6");
        let input = &input[0];

        assert_eq!(solve(&input, MARKER_SIZE), 1598);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", MESSAGE_SIZE), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", MESSAGE_SIZE), 23);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", MESSAGE_SIZE), 23);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", MESSAGE_SIZE), 29);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", MESSAGE_SIZE), 26);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day6");
        let input = &input[0];

        assert_eq!(solve(&input, MESSAGE_SIZE), 2414);
    }
}
