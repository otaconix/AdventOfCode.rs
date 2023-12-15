use std::collections::HashMap;
use std::io;
use std::ops::Range;

use aoc_timing::trace::log_run;
use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone)]
struct ConditionRecord {
    statuses: Vec<Status>,
    groups: Vec<usize>,
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

fn possible_statuses(record: &ConditionRecord) -> u64 {
    fn inner<'a>(
        start: usize,
        group_lengths: &'a [usize],
        record: &ConditionRecord,
        working_groups: &mut Vec<Range<usize>>,
        original_groups: &[Range<usize>],
        memo: &mut HashMap<(usize, &'a [usize]), u64>,
    ) -> u64 {
        if let Some(result) = memo.get(&(start, group_lengths)) {
            *result
        } else if group_lengths.is_empty() {
            // No more groups to process
            if original_groups.iter().any(|og| og.start >= start) {
                // There are still some original groups after the last group we placed
                0
            } else {
                1
            }
        } else if start
            + group_lengths
                .iter()
                .fold(0, |result, group| result + group + 1)
            - 1
            > record.statuses.len()
        {
            // No more room to place remaining groups
            0
        } else {
            let current_group_length = group_lengths[0];
            let next_group_lengths = &group_lengths[1..];
            let mut sum = 0;

            let cutoff = original_groups
                .iter()
                .find(|og| og.start >= start)
                .map(|og| og.start)
                .unwrap_or(usize::MAX)
                .min(
                    record.statuses.len()
                        - (current_group_length
                            + next_group_lengths.iter().sum::<usize>()
                            + next_group_lengths.len()),
                );

            for start_index in start..=cutoff {
                if record.statuses.get(start_index + current_group_length) == Some(&Status::Damaged)
                    || record.statuses[start_index..start_index + current_group_length]
                        .iter()
                        .any(|overriden_status| overriden_status == &Status::Operational)
                {
                    continue;
                }

                working_groups.push(start_index..start_index + current_group_length);
                sum += inner(
                    start_index + current_group_length + 1,
                    next_group_lengths,
                    record,
                    working_groups,
                    original_groups,
                    memo,
                );
                working_groups.pop();
            }

            memo.insert((start, group_lengths), sum);
            sum
        }
    }

    inner(
        0,
        &record.groups,
        record,
        &mut vec![],
        &record
            .statuses
            .iter()
            .group_by(|s| *s)
            .into_iter()
            .fold(Vec::<(Status, Range<usize>)>::new(), |mut acc, (s, g)| {
                let index = acc.last().map(|(_, g)| g.end).unwrap_or(0);
                acc.push((*s, index..index + g.collect_vec().len()));
                acc
            })
            .iter()
            .filter_map(|(s, g)| {
                if s == &Status::Damaged {
                    Some(g.to_owned())
                } else {
                    None
                }
            })
            .collect_vec(),
        &mut HashMap::new(),
    )
}

fn part_1(input: &[ConditionRecord]) -> u64 {
    input.par_iter().map(possible_statuses).sum()
}

fn part_2(input: &[ConditionRecord]) -> u64 {
    input
        .iter()
        .cloned()
        .map(|record| {
            let expanded_statuses = repeat_n(record.statuses, 5)
                .interleave_shortest(repeat_n(vec![Status::Unknown], 4))
                .flatten()
                .collect_vec();
            let expanded_groups = repeat_n(record.groups, 5).flatten().collect();

            ConditionRecord {
                statuses: expanded_statuses,
                groups: expanded_groups,
            }
        })
        .enumerate()
        .par_bridge()
        .map(|(index, record)| (index, possible_statuses(&record)))
        .map(|(_, count)| count)
        .sum()
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 525152);
    }
}
