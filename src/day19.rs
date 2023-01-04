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
            resources[i] -= prices[i];
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

pub fn estimate(blueprint: &Blueprint) -> i32 {
    let initial = State::initial();
    let mut states = vec![HashSet::from([initial])];
    let mut result = 0;
    for t in 0..=24 {
        let mut next = HashSet::new();
        for state in states[t].iter() {
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
        // next.retain(|state| state.resources[GEODE] == max_geode);
        states.push(next);
        println!("size: {} {}", t, states.last().unwrap().len());
    }
    // let mut result = 0;
    // for state in states.last().unwrap() {
        // result = result.max(state.resources[GEODE]);
    // }
    result
}

pub struct Blueprint {
    prices: [[i32; 4]; 4],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate() {
        let blueprint = Blueprint {
            prices: [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
        };
        assert_eq!(estimate(&blueprint), 12);
    }
}
