#[derive(PartialEq, Copy, Clone)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_guide(pick: char) -> Self {
        match pick {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("from_guide: wrong guide {}", pick),
        }
    }
}

impl Pick {
    fn score_for_pick(&self) -> u32 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }

    fn from_his_pick(pick: char) -> Self {
        match pick {
            'A' => Pick::Rock,
            'B' => Pick::Paper,
            'C' => Pick::Scissors,
            _ => panic!("from_his_pack: wrong pick: {}", pick),
        }
    }

    fn from_your_pick(pick: char) -> Self {
        match pick {
            'X' => Pick::Rock,
            'Y' => Pick::Paper,
            'Z' => Pick::Scissors,
            _ => panic!("from_your_pick: wrong picK: {}", pick),
        }
    }
}

fn score(him: Pick, you: Pick) -> u32 {
    let result = you.score_for_pick();
    if him == you {
        return 3 + result;
    }

    result + score_for_round(him, you)
}

fn score_for_round(him: Pick, you: Pick) -> u32 {
    match (you, him) {
        (Pick::Rock, Pick::Scissors)
        | (Pick::Paper, Pick::Rock)
        | (Pick::Scissors, Pick::Paper) => 6,
        _ => 0,
    }
}

pub fn day2<S: AsRef<str>>(input: &Vec<S>) -> u32 {
    let mut total = 0;
    for line in input {
        let line = line.as_ref();
        let mut chars = line.chars();
        let him = chars.next().unwrap();
        let him = Pick::from_his_pick(him);
        let you = chars.nth(1).unwrap();
        let you = Pick::from_your_pick(you);
        total += score(him, you);
    }
    total
}

pub fn part2<S: AsRef<str>>(input: &Vec<S>) -> u32 {
    let mut total = 0;
    for line in input {
        let line = line.as_ref();
        let mut chars = line.chars();
        let him = chars.next().unwrap();
        let him = Pick::from_his_pick(him);
        let outcome = chars.nth(1).unwrap();
        let outcome = Outcome::from_guide(outcome);
        total += score_part2(him, outcome);
    }
    total
}

fn score_part2(him: Pick, outcome: Outcome) -> u32 {
    let you = translate_outcome(&him, &outcome);
    score(him, you)
}

fn translate_outcome(him: &Pick, outcome: &Outcome) -> Pick {
    match (him, outcome) {
        (Pick::Paper, Outcome::Lose) => Pick::Rock,
        (Pick::Paper, Outcome::Win) => Pick::Scissors,
        (Pick::Rock, Outcome::Lose) => Pick::Scissors,
        (Pick::Rock, Outcome::Win) => Pick::Paper,
        (Pick::Scissors, Outcome::Lose) => Pick::Paper,
        (Pick::Scissors, Outcome::Win) => Pick::Rock,
        (other, Outcome::Draw) => *other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_real_data;

    #[test]
    fn test_day2() {
        let actual = day2(&vec!["A Y", "B X", "C Z"]);
        assert_eq!(actual, 15);
    }

    #[test]
    fn test_with_real_data() {
        let actual = day2(&read_real_data("day2"));
        assert_eq!(actual, 9177, "actual is {}", actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&vec!["A Y", "B X", "C Z"]);
        assert_eq!(actual, 12);
    }

    #[test]
    fn test_part2_with_real_data() {
        let actual = part2(&read_real_data("day2"));
        assert_eq!(actual, 12111, "actual is {}", actual);
    }
}
