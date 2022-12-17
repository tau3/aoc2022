pub fn day1<S: AsRef<str>>(input: &Vec<S>) -> (usize, u32) {
    let calories_per_elf = calories_per_elf(input);

    let result = calories_per_elf
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .map(|(index, calories)| (index + 1, calories.to_owned()))
        .expect("day1: unexpected empty input");

    result
}

fn calories_per_elf<S: AsRef<str>>(input: &Vec<S>) -> Vec<u32> {
    let mut calories_per_elf = Vec::new();
    let mut current = 0;
    for line in input {
        let line = line.as_ref();
        if line.is_empty() {
            calories_per_elf.push(current);
            current = 0;
            continue;
        }
        let calories: u32 = line
            .parse()
            .unwrap_or_else(|_| panic!("failed to parse {}", line));
        current += calories;
    }
    calories_per_elf.push(current);
    calories_per_elf
}

pub fn day1_part2<S: AsRef<str>>(input: &Vec<S>) -> u32 {
    let mut calories_per_elf = calories_per_elf(input);
    calories_per_elf.sort_by(|x, y| y.cmp(x));
    calories_per_elf.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_real_data;

    #[test]
    fn test_day1() {
        let actual = day1(&vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);

        assert_eq!(actual, (4, 24000));
    }

    #[test]
    fn test_with_real_data() {
        let (index, value) = day1(&read_real_data("day1"));
        assert_eq!((index, value), (137, 75622));
    }

    #[test]
    fn test_part2() {
        let actual = day1_part2(&vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);
        assert_eq!(actual, 45000);
    }

    #[test]
    fn test_part2_with_real_data() {
        let expected = 213159;
        let actual = day1_part2(&read_real_data("day1"));
        assert_eq!(actual, expected, "expected {}, got {}", expected, actual);
    }
}
