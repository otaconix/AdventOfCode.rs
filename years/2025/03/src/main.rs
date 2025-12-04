use std::cmp::Reverse;
use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

type Input = Vec<Vec<(usize, u64)>>;
type Output1 = u64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();
            let mut result = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .enumerate()
                .collect_vec();
            result.sort_unstable_by_key(|(index, joltage)| (Reverse(*joltage), *index));
            result
        })
        .collect()
}

/// Sort the bank of batteries by joltage (descending), then by index within the bank (ascending).
fn sort_battery_bank(bank: &[u64]) -> Vec<(usize, u64)> {
    let mut result = bank.iter().copied().enumerate().collect_vec();

    result.sort_unstable_by_key(|(index, joltage)| (Reverse(*joltage), *index));

    result
}

fn part_1(input: &Input) -> Output1 {
    input.iter().map(|bank| most_joltage(bank, 2)).sum()
}

fn most_joltage(sorted_bank: &[(usize, u64)], desired_length: usize) -> u64 {
    fn inner(
        sorted_bank: &[(usize, u64)],
        desired_length: usize,
        number_thus_far: u64,
        digits: usize,
        current_index: Option<usize>,
    ) -> Option<u64> {
        if digits == desired_length {
            Some(number_thus_far)
        } else if current_index
            .map(|i| i + desired_length - digits > sorted_bank.len())
            .unwrap_or(false)
        {
            None
        } else {
            sorted_bank
                .iter()
                .filter(|(index, _)| current_index.map(|i| i < *index).unwrap_or(true))
                .flat_map(|(index, joltage)| {
                    inner(
                        sorted_bank,
                        desired_length,
                        number_thus_far * 10 + joltage,
                        digits + 1,
                        Some(*index),
                    )
                })
                .next()
        }
    }

    inner(sorted_bank, desired_length, 0, 0, None).unwrap()
}

fn part_2(input: &Input) -> Output2 {
    input.iter().map(|bank| most_joltage(bank, 12)).sum()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        // let part_1 = part_1(&input);
        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");

        // let part_2 = part_2(&input);
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

        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 3121910778619);
    }
}
