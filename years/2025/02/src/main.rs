use std::io;
use std::ops::RangeInclusive;

use aoc_timing::trace::log_run;
use itertools::Itertools;

type Input = Vec<RangeInclusive<u64>>;
type Output1 = u64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap();
    line.as_ref()
        .split(',')
        .map(|range| {
            let (from, to) = range.split_once('-').unwrap();

            RangeInclusive::new(from.parse().unwrap(), to.parse().unwrap())
        })
        .collect()
}

fn digit_count(n: &u64) -> u32 {
    if *n == 0 { 1 } else { n.ilog10() + 1 }
}

/// The idea: for each range, take the first half of the digits of the lower bound, then iterate
/// from that number forward, and on every iteration, duplicate the digits and check if the result
/// is within the range.
/// Only consider those results that are in range!
fn part_1(input: &Input) -> Output1 {
    input
        .iter()
        .flat_map(|range| {
            let start_count = digit_count(range.start());
            let end_count = digit_count(range.end());

            (range.start() / 10u64.pow(start_count.div_ceil(2))
                ..=range.end() / 10u64.pow(end_count / 2))
                .map(|n| n + n * 10u64.pow(digit_count(&n)))
                .skip_while(|n| !range.contains(n))
                .take_while(|n| range.contains(n))
                .collect::<Vec<_>>()
                .into_iter()
        })
        .sum()
}

fn is_repeating(n: &str) -> bool {
    let s = n.as_bytes();

    (1..=n.len() / 2).any(|digits| {
        if n.len().is_multiple_of(digits) {
            (0..n.len())
                .step_by(digits)
                .map(|x| &s[x..x + digits])
                .all_equal()
        } else {
            false
        }
    })
}

/// This is a little more difficult. We can do it the easy way and simply loop over all numbers in
/// range, which will take a while.
fn part_2(input: &Input) -> Output2 {
    input
        .iter()
        .flat_map(|range| {
            range.clone().filter(|n| {
                let r = n.to_string();

                is_repeating(&r)
            })
        })
        .sum()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input));
        println!("Part 2: {part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 4174379265);
    }
}
