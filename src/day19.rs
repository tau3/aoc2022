use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

// const ORE: usize = 0;
// const CLAY: usize = 1;
// const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    resources: [i32; 4],
    robots: [i32; 4],
}

impl State {
    fn initial() -> Self {
        let resources = [0, 0, 0, 0];
        let robots = [1, 0, 0, 0];
        Self { resources, robots }
    }

    fn advance(&self, blueprint: &Blueprint) -> HashSet<State> {
        if let Some(state) = self.build_robot(blueprint, GEODE) {
            return HashSet::from([state]);
        }

        let mut result = HashSet::new();
        result.insert(self.just_harvest());
        for i in 0..self.resources.len() - 1 {
            let variant = self.build_robot(blueprint, i);
            if let Some(variant) = variant {
                result.insert(variant);
            }
        }
        result
    }

    fn just_harvest(&self) -> State {
        let mut resources = self.resources;
        for (i, val) in self.robots.iter().enumerate() {
            resources[i] += val;
        }
        State {
            resources,
            robots: self.robots,
        }
    }

    fn build_robot(&self, blueprint: &Blueprint, kind: usize) -> Option<State> {
        let prices = blueprint.prices[kind];
        let mut resources = self.resources;
        for i in 0..resources.len() {
            if i != GEODE {
                resources[i] -= prices[i];
            }
            if resources[i] < 0 {
                return None;
            }
            resources[i] += self.robots[i];
        }
        let mut robots = self.robots;
        robots[kind] += 1;
        Some(State { resources, robots })
    }
}

fn estimate(blueprint: &Blueprint) -> i32 {
    let initial = State::initial();
    let mut states = HashSet::from([initial]);
    let mut cache = HashSet::new();
    let mut result = 0;
    for t in 0..24 {
        let mut next = HashSet::new();
        for state in states.iter() {
            if !cache.insert(*state) {
                continue;
            }
            let next_states = state.advance(blueprint);
            next.extend(next_states);
        }
        let max_geode = next
            .iter()
            .map(|state| state.resources[GEODE])
            .max()
            .unwrap();
        result = result.max(max_geode);
        next.retain(|state| state.resources[GEODE] == result);
        states = next;
        // println!("size: {} {}, res: {}", t, states.len(), result);
    }
    result
}

pub fn solve(input: &[&str]) -> i32 {
    input
        .par_iter()
        .map(|line| Blueprint::new(line))
        .map(|blueprint| blueprint.index * estimate(&blueprint))
        .sum::<i32>()
}

struct Blueprint {
    index: i32,
    prices: [[i32; 3]; 4],
}

impl Blueprint {
    fn new(raw: &str) -> Self {
        let regex = Regex::new(r"[^0-9]+").unwrap();
        let only_numbers = regex.replace_all(raw, " ");
        let tokens: Vec<i32> = only_numbers
            .trim()
            .split(' ')
            .map(|token| token.parse())
            .map(|token| token.unwrap())
            .collect();
        let index = tokens[0];
        let ore_robot_ore = tokens[1];
        let clay_robot_ore = tokens[2];
        let obsidian_robot_ore = tokens[3];
        let obsidian_robot_clay = tokens[4];
        let geode_robot_ore = tokens[5];
        let geode_robot_obsidian = tokens[6];
        let prices = [
            [ore_robot_ore, 0, 0],
            [clay_robot_ore, 0, 0],
            [obsidian_robot_ore, obsidian_robot_clay, 0],
            [geode_robot_ore, 0, geode_robot_obsidian],
        ];
        Self { index, prices }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_estimate() {
        let blueprint = Blueprint {
            index: 1,
            prices: [[4, 0, 0], [2, 0, 0], [3, 14, 0], [2, 0, 7]],
        };
        assert_eq!(estimate(&blueprint), 9);
    }

    #[test]
    fn test_with_real_data() {
        let input = util::read_real_data("day19");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), 1834);
    }

    #[test]
    fn test_solve() {
        let input = [ "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.", "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."];
        assert_eq!(solve(&input), 33);
    }
}
