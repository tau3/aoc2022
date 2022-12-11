fn is_fully_contains((s1, e1): (u32, u32), (s2, e2): (u32, u32)) -> bool {
    (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1)
}

fn check_line(line: &str) -> bool {
    let tokens: Vec<&str> = line.split(",").collect();
    let (left, right) = (tokens[0], tokens[1]);
    let s1e1: Vec<&str> = left.split("-").collect();
    let s2e2: Vec<&str> = right.split("-").collect();
    let (s1, e1) = (s1e1[0].parse().expect("s1"), s1e1[1].parse().expect("e1"));
    let (s2, e2) = (s2e2[0].parse().expect("s2"), s2e2[1].parse().expect("e2"));

    return is_fully_contains((s1, e1), (s2, e2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_is_fully_contains() {
        assert!(!is_fully_contains((2, 4), (6, 8)));
        assert!(!is_fully_contains((2, 3), (4, 5)));
        assert!(!is_fully_contains((5, 7), (7, 9)));
        assert!(is_fully_contains((2, 8), (3, 7)));
        assert!(is_fully_contains((6, 6), (4, 6)));
        assert!(!is_fully_contains((2, 6), (4, 8)));
    }

    #[test]
    fn test_check_line() {
        assert!(check_line("6-6,4-6"));
    }
}
