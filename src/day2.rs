#[derive(PartialEq)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

impl Pick {
    pub fn score_for_pick(&self) -> u32 {
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

fn score(him: char, you: char) -> u32 {
    let him = Pick::from_his_pick(him);
    let you = Pick::from_your_pick(you);
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

fn translate(choice: char) -> char {
    match choice {
        'A' | 'X' => 'R',
        'B' | 'Y' => 'P',
        'C' | 'Z' => 'S',
        _ => panic!("error input: {}", choice),
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

pub fn part2(mut him: char, outcome: char) -> u32 {
    him = translate(him);
    let you = translate_outcome(him, outcome);
    score(him, you)
}

fn translate_outcome(_him: char, _outcome: char) -> char {
    //x -lose, y - dray
    // match (him, outcome) {
    // 	('A', 'X') => {''}
    // 	('A', 'Y') => {''}
    // 	('A', 'Z') => {}
    // 	('B', 'X') => {}
    // 	('B', 'Y') => {}
    // 	('B', 'Z') => {}
    // 	('C', 'X') => {}
    // 	('C', 'Y') => {}
    // 	('C', 'Z') => {}
    // 	_ => panic!("Foo")
    // }
    'A'
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_day2() {
        let actual = day2(vec!["A Y", "B X", "C Z"]);
        assert_eq!(actual, 15);
    }

    #[test]
    fn test_with_real_data() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("resources/test/day2");

        let contents = fs::read_to_string(dir).unwrap();
        let lines: Vec<&str> = contents.lines().collect();

        let actual = day2(lines);
        assert_eq!(actual, 9177, "actual is {}", actual);
    }
}
