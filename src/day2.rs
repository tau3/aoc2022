fn score_for_pick(pick: char) -> u32 {
    match pick {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => panic!("wrong pick: {}", pick),
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
    result
}

fn translate(choise: char) -> char {
    match choise {
        'A' | 'X' => 'R',
        'B' | 'Y' => 'P',
        'C' | 'Z' => 'S',
        _ => panic!("error input: {}", choise),
    }
}

pub fn day2(input: Vec<&str>) -> u32 {
    let mut total = 0;
    for line in input {
        let mut chars = line.chars();
        let him = chars.next().unwrap();
        let you = chars.nth(1).unwrap();
        total += score(him, you);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let actual = day2(vec!["A Y", "B X", "C Z"]);
        assert_eq!(actual, 15);
    }
}
