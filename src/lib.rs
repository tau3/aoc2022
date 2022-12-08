use anyhow::{Context, Result};

pub fn day1(input: &dyn AsRef<[&str]>) -> Result<(usize, u32)> {
    let mut calories_per_elf = Vec::new();
    let mut current = 0;
    for line in input.as_ref() {
        if line.is_empty() {
            calories_per_elf.push(current);
            current = 0;
            continue;
        }
        let calories: u32 = line.parse()?;
        current += calories;
    }
    calories_per_elf.push(current);

    return calories_per_elf
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .context("empty input")
        .map(|(index, calories)| (index + 1, calories.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let actual = day1(&[
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);

        assert_eq!(actual.unwrap(), (4, 24000));
    }
}
