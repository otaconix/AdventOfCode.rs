use itertools::Itertools;
use std::cmp::Reverse;

type Input = Vec<Vec<(usize, u64)>>;
type Output1 = u64;
type Output2 = Output1;

pub fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
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

pub fn part_1(input: &Input) -> Output1 {
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

pub fn part_2(input: &Input) -> Output2 {
    input.iter().map(|bank| most_joltage(bank, 12)).sum()
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
