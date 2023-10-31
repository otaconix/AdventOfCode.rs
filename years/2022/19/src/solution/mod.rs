mod parser;

use aoc_macros::EnumVariants;
use aoc_utils::EnumVariants;
use log::debug;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::{Add, Index, IndexMut, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, EnumVariants)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Resources([u32; 4]);

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0].saturating_sub(rhs.0[0]),
            self.0[1].saturating_sub(rhs.0[1]),
            self.0[2].saturating_sub(rhs.0[2]),
            self.0[3].saturating_sub(rhs.0[3]),
        ])
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl Add<&Resource> for Resources {
    type Output = Self;

    fn add(self, rhs: &Resource) -> Self::Output {
        let mut result = self;
        result[rhs] += 1;

        result
    }
}

impl Mul<u32> for Resources {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}

impl Index<&Resource> for Resources {
    type Output = u32;

    fn index(&self, index: &Resource) -> &Self::Output {
        match *index {
            Resource::Ore => &self.0[0],
            Resource::Clay => &self.0[1],
            Resource::Obsidian => &self.0[2],
            Resource::Geode => &self.0[3],
        }
    }
}

impl IndexMut<&Resource> for Resources {
    fn index_mut(&mut self, index: &Resource) -> &mut Self::Output {
        match *index {
            Resource::Ore => &mut self.0[0],
            Resource::Clay => &mut self.0[1],
            Resource::Obsidian => &mut self.0[2],
            Resource::Geode => &mut self.0[3],
        }
    }
}

#[derive(Debug, Default)]
pub struct Blueprint {
    id: u32,
    costs: HashMap<Resource, Resources>,
    max: Resources,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Factory {
    minutes_left: u32,
    resources: Resources,
    robots: Resources,
}

impl Ord for Factory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resources[&Resource::Geode].cmp(&other.resources[&Resource::Geode])
    }
}

impl PartialOrd for Factory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Factory {
    pub fn initial(minutes_left: u32) -> Self {
        Self {
            minutes_left,
            robots: Resources([1, 0, 0, 0]),
            ..Self::default()
        }
    }

    fn buy_robot(&self, robot_resource: &Resource, cost: &Resources) -> Option<Self> {
        let cost_difference = *cost - self.resources;
        let minutes_necessary = Resource::variants()
            .iter()
            .filter(|resource| cost_difference[resource] > 0)
            .map(|resource| {
                if self.robots[resource] > 0 {
                    cost_difference[resource].div_ceil(self.robots[resource])
                } else {
                    self.minutes_left
                }
            })
            .max()
            .unwrap_or(0)
            + 1;

        if minutes_necessary > self.minutes_left {
            None
        } else {
            Some(Self {
                minutes_left: self.minutes_left - minutes_necessary,
                resources: self.resources + (self.robots * minutes_necessary) - *cost,
                robots: self.robots + robot_resource,
            })
        }
    }

    fn run_to_completion(&self) -> Self {
        Self {
            minutes_left: 0,
            robots: self.robots,
            resources: self.resources + (self.robots * self.minutes_left),
        }
    }

    fn needs_to_produce_more_robots(
        &self,
        robot_resource: &Resource,
        blueprint: &Blueprint,
    ) -> bool {
        robot_resource == &Resource::Geode
            || self.robots[robot_resource] < blueprint.max[robot_resource]
    }

    // Try to buy every possible robot (wait for as long as is necessary to actually be
    // able to), and also return a state where we didn't buy a robot.
    fn next_possible_states(&self, blueprint: &Blueprint) -> Vec<Factory> {
        Resource::variants()
            .iter()
            .filter(|resource| self.needs_to_produce_more_robots(resource, blueprint))
            .flat_map(|resource| {
                self.buy_robot(resource, &blueprint.costs[resource])
                    .into_iter()
            })
            .chain(Some(self.run_to_completion()))
            .collect()
    }

    fn can_theoretically_produce_more_geodes(&self, geodes: u32) -> bool {
        self.resources[&Resource::Geode]
            + (1..=self.minutes_left)
                .map(|minute| minute + self.robots[&Resource::Geode])
                .sum::<u32>()
            > geodes
    }
}

impl Blueprint {
    fn new(id: u32, costs: HashMap<Resource, Resources>) -> Self {
        let max_by_resource = Resources(
            Resource::variants()
                .map(|resource| costs.values().map(|cost| cost[&resource]).max().unwrap()),
        );

        Blueprint {
            id,
            costs,
            max: max_by_resource,
        }
    }

    pub fn run_simulation(&self, initial_factory: Factory) -> u32 {
        debug!("Starting simulation for blueprint {}", self.id);
        let mut states_queue = BinaryHeap::from([initial_factory]);
        let mut seen_states = HashSet::new();
        let mut max_geodes = 0;

        while let Some(factory) = states_queue.pop() {
            if seen_states.contains(&factory) {
                continue;
            }

            seen_states.insert(factory);

            if factory.minutes_left == 0 {
                max_geodes = max_geodes.max(factory.resources[&Resource::Geode]);
            } else if factory.can_theoretically_produce_more_geodes(max_geodes) {
                states_queue.extend(factory.next_possible_states(self));
            }
        }
        debug!(
            "Ended simulation for blueprint {}, max geodes: {}",
            self.id, max_geodes
        );
        debug!("Seen states: {}", seen_states.len());

        max_geodes
    }
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::blueprint_parser()
            .parse_str(s)
            .map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        env_logger::init();

        let factory = Factory::initial(24);

        let blueprint_1 = Blueprint::new(
            1,
            HashMap::from([
                (Resource::Ore, Resources([4, 0, 0, 0])),
                (Resource::Clay, Resources([2, 0, 0, 0])),
                (Resource::Obsidian, Resources([3, 14, 0, 0])),
                (Resource::Geode, Resources([2, 0, 7, 0])),
            ]),
        );

        let blueprint_2 = Blueprint::new(
            2,
            HashMap::from([
                (Resource::Ore, Resources([2, 0, 0, 0])),
                (Resource::Clay, Resources([3, 0, 0, 0])),
                (Resource::Obsidian, Resources([3, 8, 0, 0])),
                (Resource::Geode, Resources([3, 0, 12, 0])),
            ]),
        );

        assert_eq!(9, blueprint_1.run_simulation(factory));
        assert_eq!(24, blueprint_2.run_simulation(factory));
    }

    #[test]
    fn compare_factories() {
        let factory_no_geodes = Factory::default();
        let factory_one_geode = Factory {
            resources: Resources([0, 0, 0, 1]),
            ..Default::default()
        };

        assert!(factory_one_geode > factory_no_geodes);
    }
}
