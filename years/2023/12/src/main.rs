use std::io;
use std::ops::Range;

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

fn possible_statuses(record: &ConditionRecord) -> u32 {
    fn inner(
        start: usize,
        group_lengths: &[usize],
        record: &ConditionRecord,
        working_groups: &mut Vec<Range<usize>>,
        original_groups: &[Range<usize>],
    ) -> u32 {
        if group_lengths.is_empty() {
            /*
            println!(
                "{}",
                (0..record.statuses.len())
                    .map(|index| {
                        if working_groups.iter().any(|w| w.contains(&index)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            );
            */
            if original_groups
                .iter()
                .any(|og| og.start > working_groups.last().unwrap().end)
            {
                0
            } else {
                1
            }
        } else if start + group_lengths[0] > record.statuses.len() {
            0
        } else {
            let current_group_length = group_lengths[0];
            let next_group_lengths = &group_lengths[1..];
            let mut sum = 0;

            let cutoff = original_groups
                .iter()
                .find_map(|original| {
                    if original.start >= start {
                        original.start.into()
                    } else {
                        None
                    }
                })
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
                );
                working_groups.pop();
            }

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
                let index = acc.last().map(|(_, g)| g.end).unwrap_or(0usize);
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
    )
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = input./*par_*/iter().map(possible_statuses).sum::<u32>();

    println!("Part 1: {part_1}");

    let part_2 = input
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
        .inspect(|r| {
            /*
            println!(
                "{}",
                r.statuses
                    .iter()
                    .map(|s| match s {
                        Status::Damaged => '#',
                        Status::Operational => '.',
                        Status::Unknown => '?',
                    })
                    .collect::<String>()
            )
                */
        })
        .enumerate()
        .par_bridge()
        .map(|(index, record)| (index, possible_statuses(&record)))
        .inspect(|(index, count)| println!("{index}: {count}"))
        .map(|(_, count)| count)
        .sum::<u32>();

    println!("Part 2: {part_2}");
}
