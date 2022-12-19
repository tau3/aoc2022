// TODO memo

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let (start, end) = find_start(&grid);
        Self {
            grid,
            width,
            height,
            start,
            end,
        }
    }

    fn shortest_path(&self, target: (usize, usize)) -> u32 {
        if target == self.start {
            return 0;
        }
        let adjacent = self.adjacent(target);
        println!("point {:?}, adjacent {:?}", target, adjacent);
        1 + adjacent
            .iter()
            .filter(|(col, row)| self.can_jump((*col, *row), target))
            .map(|(col, row)| self.shortest_path((*col, *row)))
            .min()
            .unwrap()
    }

    fn at(&self, (col, row): (usize, usize)) -> char {
        self.grid[row][col]
    }

    fn can_jump(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let from = self.at(from);
        let to = self.at(to);

        add1_char(from) >= to
    }

    fn adjacent(&self, (col, row): (usize, usize)) -> Vec<(usize, usize)> {
        let (col, row) = (col as i32, row as i32);
        let result = vec![
            (col - 1, row + 1),
            (col, row + 1),
            (col + 1, row + 1),
            (col + 1, row),
            (col + 1, row - 1),
            (col, row - 1),
            (col - 1, row - 1),
            (col - 1, row),
        ];

        result
            .iter()
            .filter(|(col, row)| {
                *col >= 0 && *row >= 0 && *col < self.width as i32 && *row < self.height as i32
            })
            .map(|(col, row)| (*col as usize, *row as usize))
            .collect()
    }
}

fn add1_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap()
}

pub fn solve(input: Vec<Vec<char>>) -> u32 {
    let grid = Grid::new(input);
    grid.shortest_path(grid.end)
}

fn find_start(input: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == 'S' {
                start = (col, row);
            } else if input[row][col] == 'E' {
                end = (col, row);
            }
        }
    }
    println!("start {:?}, end {:?}", start, end);

    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

        let input = input.iter().map(|line| line.chars().collect()).collect();
        assert_eq!(solve(input), 31);
    }
}
