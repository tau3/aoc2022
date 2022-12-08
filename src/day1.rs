use anyhow::{Context, Result};

pub fn day1(input: &Vec<&str>) -> Result<(usize, u32)> {
    let calories_per_elf = calories_per_elf(input)?;

    let result = calories_per_elf
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .context("empty input")
        .map(|(index, calories)| (index + 1, calories.to_owned()))?;

    Ok(result)
}

fn calories_per_elf(input: &Vec<&str>) -> Result<Vec<u32>> {
    let mut calories_per_elf = Vec::new();
    let mut current = 0;
    for line in input {
        if line.is_empty() {
            calories_per_elf.push(current);
            current = 0;
            continue;
        }
        let calories: u32 = line.parse()?;
        current += calories;
    }
    calories_per_elf.push(current);
    Ok(calories_per_elf)
}

pub fn day1_part2(input: &Vec<&str>) -> Result<u32> {
    let mut calories_per_elf = calories_per_elf(input)?;
    calories_per_elf.sort_by(|x, y| y.cmp(x));
    Ok(calories_per_elf.iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_day1() {
        let actual = day1(&vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);

        assert_eq!(actual.unwrap(), (4, 24000));
    }

    #[test]
    fn test_with_real_data() -> Result<()> {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("resources/test/day1");

        let contents = fs::read_to_string(dir)?;
        let lines: Vec<&str> = contents.lines().collect();

        let (index, value) = day1(&lines)?;
        println!("index={}, value={}", index, value);

        Ok(())
    }

    #[test]
    fn test_part2() {
        let actual = day1_part2(&vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);
        assert_eq!(actual.unwrap(), 45000);
    }

    #[test]
    fn test_part2_with_real_data() -> Result<()> {
        let lines = read_file()?;

        let (index, value) = day1(&lines)?;
        println!("index={}, value={}", index, value);

        Ok(())
    }

    fn read_file() -> Result<Vec<&str>> {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("resources/test/day1");

        let contents = fs::read_to_string(dir)?;
        let result: Vec<&str> = contents.lines().collect();
        Ok(result)
    }
}
