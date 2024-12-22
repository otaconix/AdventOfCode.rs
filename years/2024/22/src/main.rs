use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;
use rapidhash::RapidHashMap;
use rapidhash::RapidHashSet;
use rayon::prelude::*;

type Input = Vec<isize>;
type Output1 = isize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.parse().unwrap()
        })
        .collect()
}

const PRUNER: isize = 16777216;

struct SecretIterator {
    secret: isize,
}

impl From<isize> for SecretIterator {
    fn from(secret: isize) -> Self {
        SecretIterator { secret }
    }
}

impl Iterator for SecretIterator {
    type Item = isize;

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

fn sequence_earnings(
    sequence: &[isize; 4],
    delta_sequences: &[RapidHashMap<[isize; 4], isize>],
) -> isize {
    delta_sequences
        .iter()
        .map(|delta_sequence| delta_sequence.get(sequence).unwrap_or(&0))
        .sum()
}

fn part_2(input: &Input) -> Output2 {
    let secret_sequences = input
        .iter()
        .copied()
        .map(|secret| {
            SecretIterator::from(secret)
                .take(2001)
                .map(|n| n % 10)
                .tuple_windows()
                .map(|(a, b)| (a - b, b))
                .tuple_windows::<(_, _, _, _)>()
                .map(|window| {
                    (
                        Into::<[_; 4]>::into(window).map(|(delta, _bananas)| delta),
                        window.3 .1,
                    )
                })
                .fold(RapidHashMap::default(), |mut map, (window, bananas)| {
                    map.entry(window).or_insert(bananas);
                    map
                })
        })
        .collect_vec();

    let combined_sequences = secret_sequences
        .iter()
        .flat_map(|secret_sequence| secret_sequence.keys())
        .collect::<RapidHashSet<_>>();

    combined_sequences
        .into_par_iter()
        .map(|sequence| sequence_earnings(sequence, &secret_sequences))
        .max()
        .unwrap()
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
