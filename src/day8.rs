fn solve(input: &Vec<&str>) -> u32 {
    let forest: Vec<Vec<u32>> = input.iter().map(|line| parse_line(line)).collect();

    let height = forest.len();
    let width = forest[0].len();
    let mut result = 0;
    for row in 0..height {
        for col in 0..width {
            if row == 0 || col == 0 || row == width - 1 || col == height - 1 {
                result += 1;
                continue;
            }
            result += check((row, col), &forest)
        }
    }
    result
}

fn check((row, col): (usize, usize), forest: &Vec<Vec<u32>>) -> u32 {
    let height = forest.len();
    let width = forest[0].len();

    let horizontal = &forest[row];
    let vertical: Vec<u32> = forest.iter().map(|line| line[col]).collect();

    let slices = [
        &horizontal[0..col],
        &horizontal[col + 1..width],
        &vertical[0..row],
        &vertical[row + 1..height],
    ];
    let tree = forest[row][col];
    for slice in slices {
        if slice.iter().all(|t| t < &tree) {
            return 1;
        }
    }
    return 0;
}

fn parse_line(line: &str) -> Vec<u32> {
    line.chars().map(|c| c as u32 - 48).collect()
}

fn part2(input: &Vec<&str>) -> u32 {
    let forest: Vec<Vec<u32>> = input.iter().map(|line| parse_line(line)).collect();

    let height = forest.len();
    let width = forest[0].len();
    let mut result = 0;
    for row in 0..height {
        for col in 0..width {
            let mut score = score((row, col), &forest);
            if score > result {
                score = result;
            }
        }
    }
    result
}

fn score((row, col): (usize, usize), forest: &Vec<Vec<u32>>) -> u32 {
    let height = forest.len();
    let width = forest[0].len();

    let horizontal = &forest[row];
    let vertical: Vec<u32> = forest.iter().map(|line| line[col]).collect();

    0
}

fn score_line(line: &Vec<u32>) -> u32 {
    let mut result = 1;
    for i in 1..line.len() - 1 {
        result += 1;
        if line[i] >= line[i - 1] {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_score_line() {
        assert_eq!(score_line(&vec![3]), 1);
        assert_eq!(score_line(&vec![5, 2]), 1);
        assert_eq!(score_line(&vec![1, 2]), 2);
        assert_eq!(score_line(&vec![3, 5, 3]), 2);
    }

    #[test]
    fn test_solve() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        assert_eq!(solve(&input), 21);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day8");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 1832);
    }

    #[test]
    fn test_part2() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        assert_eq!(part2(&input), 8);
    }

    // #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day8");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 1832);
    }
}
