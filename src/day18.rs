use std::collections::{HashSet, VecDeque};

type Voxel = (i32, i32, i32);

fn adjacent((x, y, z): &Voxel) -> Vec<Voxel> {
    vec![
        (*x - 1, *y, *z),
        (*x + 1, *y, *z),
        (*x, *y + 1, *z),
        (*x, *y - 1, *z),
        (*x, *y, *z + 1),
        (*x, *y, *z - 1),
    ]
}

fn border(cubes: &HashSet<Voxel>) -> Voxel {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for (xc, yc, zc) in cubes {
        if *xc > x {
            x = *xc;
        }
        if *yc > y {
            y = *yc;
        }
        if *zc > z {
            z = *zc;
        }
    }
    (x + 1, y + 1, z + 1)
}

pub fn part2(lava: &HashSet<Voxel>) -> u32 {
    let border = border(lava);
    let mut shell = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    shell.insert((0, 0, 0));
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let adjacents = adjacent(&current);
        for adjacent in adjacents {
            if is_inside_border(&border, &adjacent)
                && !lava.contains(&adjacent)
                && shell.insert(adjacent)
            {
                queue.push_back(adjacent);
            }
        }
    }
    lava
        .iter()
        .flat_map(adjacent)
        .filter(|voxel| is_inside_border(&border, voxel))
        .filter(|voxel| shell.contains(voxel))
        .count() as u32
}

fn is_inside_border((xm, ym, zm): &Voxel, (x, y, z): &Voxel) -> bool {
    *x >= 0 && *x <= *xm && *y >= 0 && *y <= *ym && *z >= 0 && *z <= *zm
}

pub fn solve(lava: &HashSet<Voxel>) -> u32 {
    let mut result = 0;
    for voxel in lava {
        let adjacents = adjacent(voxel);
        for adjacent in adjacents {
            if !lava.contains(&adjacent) {
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
    fn test_part2() {
        let input = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ];
        assert_eq!(part2(&parse(input)), 58);
    }

    #[test]
    fn test_part2_with_real_data() {
        let input = util::read_real_data("day18");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(part2(&parse(input)), 2044);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day18");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&parse(input)), 3448);
    }

    fn parse(input: Vec<&str>) -> HashSet<Voxel> {
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
