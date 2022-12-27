use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc};
use std::{collections::HashSet, thread};

type Point = (i32, i32);

struct Area {
    sensor: Point,
    radius: usize,
}

impl Area {
    fn from(sensor: Point, beacon: Point) -> Self {
        let radius = manhattan_distance(&sensor, &beacon);
        Self { sensor, radius }
    }

    fn contains(&self, point: &Point) -> bool {
        manhattan_distance(&self.sensor, point) <= self.radius
    }
}

pub fn part2(input: Vec<&str>, border: usize) -> usize {
    let input = parse(input);
    solve_part2(input, border)
}

fn solve_part2(input: Vec<(Point, Point)>, border: usize) -> usize {
    let areas: Vec<Area> = input
        .iter()
        .map(move |(sensor, beacon)| Area::from(*sensor, *beacon))
        .collect();
    let areas = Arc::new(areas);
    let region_size = border / 4 - 1;
    let (sender, receiver) = mpsc::channel();
    for x in 0..region_size {
        for y in 0..region_size {
            let areas = Arc::clone(&areas);
            let sender = sender.clone();
            thread::spawn(move || search_in_region(region_size, areas, x, y, sender));
        }
    }
    for _ in 0..16 {
        if let Some(result) = receiver.recv().unwrap() {
            return result;
        }
    }
    unreachable!("fail");
}

fn search_in_region(
    region_size: usize,
    areas: Arc<Vec<Area>>,
    x: usize,
    y: usize,
    sender: Sender<Option<usize>>,
) {
    for col in 0..region_size {
        for row in 0..region_size {
            let global_col = col + x * region_size;
            let global_row = row + y * region_size;
            if !is_covered(&areas, &(global_col as i32, global_row as i32)) {
                sender
                    .send(Some(global_col * 4000000 + global_row))
                    .unwrap();
                return;
            }
        }
    }
    if sender.send(None).is_err() {
        // whatever
    }
}

fn is_covered(areas: &[Area], point: &Point) -> bool {
    for area in areas.iter() {
        if area.contains(point) {
            return true;
        }
    }
    false
}

fn manhattan_distance((x1, y1): &Point, (x2, y2): &Point) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

fn covered_area_on_row((col, row): &Point, radius: usize, target_row: usize) -> Vec<Point> {
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
    let mut result = Vec::new();
    for (sensor, beacon) in input.iter() {
        let radius = manhattan_distance(sensor, beacon);
        let area = covered_area_on_row(sensor, radius, row);
        for point in area {
            if !taken.contains(&point) {
                result.push(point);
            }
        }
    }
    let result: HashSet<_> = result.iter().collect();
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

    // #[test]
    // fn test_part2_with_real_data() {
    //     let input = util::read_real_data("day15");
    //     let input = input.iter().map(|line| line.as_str()).collect();
    //     assert_eq!(part2(input, 4000000), 123);
    // }

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

    #[test]
    fn test_part2() {
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

        assert_eq!(part2(input, 20), 56000011);
    }
}
