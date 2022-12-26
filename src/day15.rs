use std::collections::HashSet;

type Point = (i32, i32);

fn manhattan_distance((x1, y1): &Point, (x2, y2): &Point) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

fn area((col, row): &Point, radius: usize, target_row: usize) -> Vec<Point> {
    let radius_signed = radius as i32;
    let mut smallest = 0;
    if radius_signed <= *row {
        smallest = row - radius_signed;
    }
    let biggest = *row as usize + radius;
    if !(target_row <= biggest && target_row >= smallest as usize) {
        return vec![];
    }

    let mut result = Vec::new();
    let radius_signed = radius_signed - (row - target_row as i32).abs();
    for i in -radius_signed..=radius_signed {
        let candidate = (i + col, target_row as i32);
        result.push(candidate);
    }
    result
}

fn solve(input: Vec<(Point, Point)>, row: usize) -> usize {
    let mut taken = HashSet::new();
    for (sensor, beacon) in input.iter() {
        taken.insert(sensor);
        taken.insert(beacon);
    }
    let mut result = HashSet::new();
    for (sensor, beacon) in input.iter() {
        let radius = manhattan_distance(sensor, beacon);
        let area = area(sensor, radius, row);
        for point in area {
            if !taken.contains(&point) {
                result.insert(point);
            }
        }
    }
    result.len()
}

fn parse(input: Vec<&str>) -> Vec<(Point, Point)> {
    let mut result = Vec::new();
    for line in input {
        let mut tokens = line.split(' ');
        let sensor = (
            parse_value(tokens.nth(2).unwrap()),
            parse_value(tokens.next().unwrap()),
        );
        let beacon = (
            parse_value(tokens.nth(4).unwrap()),
            parse_value(tokens.next().unwrap()),
        );
        result.push((sensor, beacon));
    }
    result
}

fn parse_value(input: &str) -> i32 {
    input[2..].replace([',', ':'], "").parse().unwrap()
}

pub fn part1(input: Vec<&str>, row: usize) -> usize {
    let input = parse(input);
    solve(input, row)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part1_with_real_data() {
        let input = util::read_real_data("day15");
        let input = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(part1(input, 2000000), 4919281);
    }

    #[test]
    fn test_part1() {
        let input = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];

        assert_eq!(part1(input, 10), 26);
    }
}
