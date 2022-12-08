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

pub fn day2(input: Vec<&str>) -> u32 {
    let mut total = 0;
    for line in input {
        let him = line.chars().nth(0).unwrap();
        let you = line.chars().nth(2).unwrap();
        total += score(him, you);
    }
    return total;
}

fn score_for_pick(pick: char) -> u32 {
    match pick {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => panic!("wrong pick") // TODO fmt + result
    }
}

fn score(mut him: char, mut you: char) -> u32 {
    him = translate(him);
    you = translate(you);

    let mut result = score_for_pick(you);

    if him == you {
        return 3 + result;
    }

    match you {
        'R' => {
            if him == 'S' {
                result += 6;
            }
        }
        'P' => {
            if him == 'R' {
                result += 6;
            }
        }
        'S' => {
            if him == 'P' {
                result += 6;
            }
        }
        _ => {
            panic!("error input!"); // TODO format + result
        }
    }
    println!("score={}", result);
    result
}

fn translate(choise: char) -> char {
    match choise {
        'A' | 'X' => 'R',
        'B' | 'Y' => 'P',
        'C' | 'Z' => 'S',
        _ => panic!("error input!"), // TODO fomrat + result
    }
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

    #[test]
    fn test_day2() {
        let actual = day2(vec!["A Y", "B X", "C Z"]);
        assert_eq!(actual, 15);
    }
}
