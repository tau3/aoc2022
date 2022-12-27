use std::collections::HashSet;

type Cube = (i32, i32, i32);

fn adjacent((x, y, z): &Cube) -> Vec<Cube> {
    vec![
        (*x - 1, *y, *z),
        (*x + 1, *y, *z),
        (*x, *y + 1, *z),
        (*x, *y - 1, *z),
        (*x, *y, *z + 1),
        (*x, *y, *z - 1),
    ]
}

pub fn solve(cubes: &HashSet<Cube>) -> u32 {
    let mut result = 0;
    for cube in cubes {
        let adjacents = adjacent(cube);
        for adjacent in adjacents {
            if !cubes.contains(&adjacent) {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ];
        assert_eq!(solve(&parse(input)), 64);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day18");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&parse(input)), 3448);
    }

    fn parse(input: Vec<&str>) -> HashSet<Cube> {
        let mut result = HashSet::new();
        for line in input {
            let mut tokens = line.split(',');
            let x = tokens.next().unwrap().parse().unwrap();
            let y = tokens.next().unwrap().parse().unwrap();
            let z = tokens.next().unwrap().parse().unwrap();
            result.insert((x, y, z));
        }
        result
    }
}
