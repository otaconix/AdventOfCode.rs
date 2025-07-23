use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use rapidhash::fast::{HashMapExt, RapidHashMap};

type Input = Vec<usize>;
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.parse().unwrap()
        })
        .collect()
}

const PRUNER: usize = 16777216;

struct SecretIterator {
    secret: usize,
}

impl From<usize> for SecretIterator {
    fn from(secret: usize) -> Self {
        SecretIterator { secret }
    }
}

impl Iterator for SecretIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.secret;

        self.secret = (self.secret ^ (self.secret * 64)) % PRUNER;
        self.secret = (self.secret ^ (self.secret >> 5)) % PRUNER;
        self.secret = (self.secret ^ (self.secret * 2048)) % PRUNER;

        Some(result)
    }
}

fn part_1(input: &Input) -> Output1 {
    input
        .iter()
        .copied()
        .map(|secret| SecretIterator::from(secret).take(2001).last().unwrap())
        .sum()
}

/// Pack a diff sequence into a single number.
///
/// Since each diff is in the range `-9..=9`, which has 19 values, we know there are at most 19
/// raised to the 4th power unique diffs. 19 happens to fit in 5 bits (2^5 = 32), so the four diffs
/// fit in 20 bits.
///
/// NB: the diffs as they come in here fall in the range `0..=18` so they can all be unsigned.
fn diff_sequence_key(diff: &(usize, usize, usize, usize)) -> usize {
    ((diff.0) << 15) | ((diff.1) << 10) | ((diff.2) << 5) | diff.3
}

/// The idea is to:
///
/// 1. Loop over each secret, generate both a list of the amount of bananas to be had at each step,
///    and a list of windows of 4 diffs.
/// 2. Keep a map of diff to the sum of bananas it results in
/// 3. Loop over each secret's diffs, and add the amount of bananas it results in for that list in
///    the map
/// 4. Loop over the entries in the map, and find the max value
fn part_2(input: &Input) -> Output2 {
    let (prices, diffs) = input.iter().fold(
        (vec![], vec![]),
        |(mut prices, mut diffs): (Vec<Vec<usize>>, Vec<RapidHashMap<usize, usize>>), secret| {
            let secrets = SecretIterator::from(*secret).take(2001).collect_vec();
            prices.push(secrets.iter().map(|price| price % 10).collect());
            diffs.push(
                secrets
                    .iter()
                    .map(|price| price % 10)
                    .tuple_windows()
                    .map(|(a, b)| b + 9 - a)
                    .tuple_windows::<(_, _, _, _)>()
                    .enumerate()
                    .map(|(diff_index, diff_window)| (diff_sequence_key(&diff_window), diff_index))
                    .fold(
                        RapidHashMap::with_capacity(2000),
                        |mut diffs, (diff_key, diff_index)| {
                            diffs.entry(diff_key).or_insert(diff_index);

                            diffs
                        },
                    ),
            );

            (prices, diffs)
        },
    );

    let mut sums: RapidHashMap<usize, usize> = RapidHashMap::with_capacity(10_000);

    for (index, diffs) in diffs.iter().enumerate() {
        for (diff, diff_index) in diffs {
            *sums.entry(*diff).or_default() += prices[index][diff_index + 4];
        }
    }

    *sums.values().max().unwrap()
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
    const INPUT2: &str = include_str!("test-input2");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT2.lines());
        let result = part_2(&input);

        assert_eq!(result, 23);
    }
}
