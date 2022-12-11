fn fully_contains((s1, e1): (u32, u32), (s2, e2): (u32, u32)) -> bool {
    (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1)
}

fn has_overlap((s1, e1): (u32, u32), (s2, e2): (u32, u32)) -> bool {
    (s1 < s2 && e1 <s2) || (e2 < s1 && e2 < e1)
}


fn check_line(line: &str) -> bool {
    let tokens: Vec<&str> = line.split(',').collect();
    let (left, right) = (tokens[0], tokens[1]);
    let s1e1: Vec<&str> = left.split('-').collect();
    let s2e2: Vec<&str> = right.split('-').collect();
    let (s1, e1) = (s1e1[0].parse().expect("s1"), s1e1[1].parse().expect("e1"));
    let (s2, e2) = (s2e2[0].parse().expect("s2"), s2e2[1].parse().expect("e2"));

    fully_contains((s1, e1), (s2, e2))
}

pub fn solve<S: AsRef<str>>(input: &[S]) -> u32 {
    input
        .iter()
        .map(|line| line.as_ref())
        .map(check_line)
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}


pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    input
        .iter()
        .map(|line| line.as_ref())
        .map(check_line)
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_fully_contains() {
        assert!(!fully_contains((2, 4), (6, 8)));
        assert!(!fully_contains((2, 3), (4, 5)));
        assert!(!fully_contains((5, 7), (7, 9)));
        assert!(fully_contains((2, 8), (3, 7)));
        assert!(fully_contains((6, 6), (4, 6)));
        assert!(!fully_contains((2, 6), (4, 8)));
    }

    #[test]
    fn test_check_line() {
        assert!(check_line("6-6,4-6"));
    }

    #[test]
    fn test_solve() {
        let input = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        assert_eq!(solve(&input), 2);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day4");
        assert_eq!(solve(&input), 576);
    }
}
