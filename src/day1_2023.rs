pub fn day1<S: AsRef<str>>(input: &[S]) -> u32 {
    input
        .iter()
        .map(|line| line.as_ref())
        .map(find_calibration_value)
        .sum()
}

fn find_calibration_value(line: &str) -> u32 {
    let mut result = 0;
    for char in line.chars() {
        if char.is_numeric() {
            result = char.to_digit(10).unwrap() * 10;
            break;
        }
    }
    for char in line.chars().rev() {
        if char.is_numeric() {
            result += char.to_digit(10).unwrap();
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::util::read_real_data;

    use super::*;

    #[test]
    fn test_day1() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let actual = day1(&input);
        assert_eq!(142, actual);
    }

    #[test]
    fn test_with_real_data() {
        let input = read_real_data("day1_2023");
        let actual = day1(&input);
        assert_eq!(123, actual);
    }

    #[test]
    fn test_find_calibration_value() {
        assert_eq!(12, find_calibration_value("1abc2"));
    }
}
