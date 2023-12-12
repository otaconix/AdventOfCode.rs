use std::{collections::HashMap, io};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
struct ConditionRecord {
    statuses: Vec<Status>,
    groups: Vec<u8>,
}

struct StatusPermutations {
    state: u32,
    unknowns: usize,
}

impl StatusPermutations {
    fn new(unknowns: usize) -> Self {
        if unknowns > 32 {
            panic!("Max amount of unknowns is 32 (new was called with {unknowns})");
        }

        StatusPermutations { state: 0, unknowns }
    }
}

impl Iterator for StatusPermutations {
    type Item = Vec<Status>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 2u32.pow(self.unknowns as u32) {
            None
        } else {
            let result = (0..self.unknowns)
                .map(|bit| {
                    if self.state & 1 << bit == 0 {
                        Status::Damaged
                    } else {
                        Status::Operational
                    }
                })
                .collect();

            self.state += 1;

            Some(result)
        }
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<ConditionRecord> {
    input
        .map(|line| {
            let line = line.to_string();
            let (statuses, groups) = line.split_once(' ').expect("No space in line");
            let statuses = statuses
                .chars()
                .map(|c| match c {
                    '#' => Status::Damaged,
                    '.' => Status::Operational,
                    '?' => Status::Unknown,
                    _ => panic!("Unknown spring status: {c}"),
                })
                .collect();
            let groups = groups.split(',').map(|num| num.parse().unwrap()).collect();

            ConditionRecord { statuses, groups }
        })
        .collect()
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = input
        .iter()
        .map(|report| {
            let unknowns = report
                .statuses
                .iter()
                .enumerate()
                .filter(|(_, status)| matches!(status, Status::Unknown))
                .map(|(index, _)| index)
                .enumerate()
                .collect::<HashMap<_, _>>();

            StatusPermutations::new(unknowns.len())
                .map(|permutation| {
                    let mut report = report.clone();

                    for (replacement_index, original_index) in &unknowns {
                        report.statuses[*original_index] = permutation[*replacement_index];
                    }

                    report
                })
                .filter(|report| {
                    report
                        .statuses
                        .iter()
                        .group_by(|status| *status)
                        .into_iter()
                        .filter_map(|(status, group)| {
                            if matches!(status, Status::Damaged) {
                                Some(group.count() as u8)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        == report.groups
                })
                .count()
        })
        .sum::<usize>();

    println!("Part 1: {part_1}");
}
