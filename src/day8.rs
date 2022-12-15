pub fn solve(input: &[&str]) -> u32 {
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
    0
}

fn parse_line(line: &str) -> Vec<u32> {
    line.chars().map(|c| c as u32 - 48).collect()
}

pub fn part2(input: &[&str]) -> u32 {
    let forest: Vec<Vec<u32>> = input.iter().map(|line| parse_line(line)).collect();

    let height = forest.len();
    let width = forest[0].len();
    let mut result = 0;
    for row in 0..height {
        for col in 0..width {
            let score = score((row, col), &forest);
            if score > result {
                result = score;
            }
        }
    }
    result
}

fn score((row, col): (usize, usize), forest: &Vec<Vec<u32>>) -> u32 {
    let height = forest.len();
    let width = forest[0].len();

    if row == 0 || col == 0 || row == width - 1 || col == height - 1 {
        return 0;
    }

    let horizontal = &forest[row];
    let vertical: Vec<u32> = forest.iter().map(|line| line[col]).collect();

    let up = vertical[0..row].iter().rev().copied().collect();
    let down = vertical[row + 1..height].to_vec();
    let left = horizontal[0..col].iter().rev().copied().collect();
    let right = horizontal[col + 1..width].to_vec();

    let tree = forest[row][col];

    let score_up = score_line(&up, tree);
    let score_down = score_line(&down, tree);
    let score_left = score_line(&left, tree);
    let score_right = score_line(&right, tree);

    score_down * score_up * score_left * score_right
}

fn score_line(line: &Vec<u32>, tree: u32) -> u32 {
    if line.len() == 1 {
        return 1;
    }

    let mut result = 1;
    for i in 0..line.len() - 1 {
        if line[i] <= line[i + 1] && line[i] < tree {
            result += 1;
        } else {
            return result;
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
        assert_eq!(score_line(&vec![3], 5), 1);
        assert_eq!(score_line(&vec![3, 3], 5), 2);
        assert_eq!(score_line(&vec![5, 2], 5), 1);
        assert_eq!(score_line(&vec![1, 2], 5), 2);
        assert_eq!(score_line(&vec![3, 5, 3], 5), 2);

        assert_eq!(score_line(&vec![3, 5, 6], 3), 1);
    }

    #[test]
    fn test_solve() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        assert_eq!(solve(&input), 21);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day8");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 1832);
    }

    #[test]
    fn test_part2() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day8");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(part2(&input), 192);
    }
}
