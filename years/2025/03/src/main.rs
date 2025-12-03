use std::cmp::Reverse;
use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

type Input = Vec<Vec<u64>>;
type Output1 = u64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect()
}

fn sort_batteries_by_index(bank: &[u64]) -> Vec<(usize, u64)> {
    bank.iter()
        .copied()
        .enumerate()
        .sorted_by_key(|(index, joltage)| (Reverse(*joltage), *index))
        .collect_vec()
}

fn part_1(input: &Input) -> Output1 {
    input
        .iter()
        .map(|bank| {
            let sorted_batteries = sort_batteries_by_index(bank);
            let first = sorted_batteries
                .iter()
                .find(|(index, _)| *index != bank.len() - 1)
                .unwrap();
            let second = sorted_batteries
                .iter()
                .find_map(|(index, joltage)| {
                    if *index > first.0 {
                        Some(joltage)
                    } else {
                        None
                    }
                })
                .unwrap();
            let first = first.1;

            first * 10 + second
        })
        .sum()
}

fn most_joltage(bank: &[u64], desired_length: usize) -> u64 {
    let sorted_bank = sort_batteries_by_index(bank);

    fn inner(
        sorted_bank: &[(usize, u64)],
        desired_length: usize,
        number_thus_far: u64,
        digits: usize,
        current_index: usize,
    ) -> Option<u64> {
        if digits == desired_length {
            Some(number_thus_far)
        } else if sorted_bank.len() - current_index < desired_length - digits {
            None
        } else {
            sorted_bank
                .iter()
                .filter(|(index, _)| digits == 0 || *index > current_index)
                .flat_map(|(index, joltage)| {
                    inner(
                        sorted_bank,
                        desired_length,
                        number_thus_far * 10 + joltage,
                        digits + 1,
                        *index,
                    )
                })
                .next()
        }
    }

    inner(&sorted_bank, desired_length, 0, 0, 0).unwrap()
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

        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 3121910778619);
    }
}

