use aoc_timing::trace::log_run;
use std::{char, io};

const REPLACEMENTS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "4"),
    ("five", "5e"),
    ("six", "6"),
    ("seven", "7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn replace_spelled_digits(input: &str) -> String {
    REPLACEMENTS
        .iter()
        .fold(input.to_string(), |result, (pattern, replacement)| {
            result.replace(pattern, replacement)
        })
}

fn input_to_number<S: AsRef<str>>(input: S) -> u16 {
    let input = input.as_ref().to_string();
    let mut digit_iter = input.chars().filter(char::is_ascii_digit);

    let first_digit = digit_iter.next().unwrap();

    [first_digit, digit_iter.nth_back(0).unwrap_or(first_digit)]
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn part_1(input: &[String]) -> u16 {
    input.iter().map(input_to_number).sum()
}

fn part_2(input: &[String]) -> u16 {
    input
        .iter()
        .map(|line| replace_spelled_digits(&line.to_string()))
        .map(input_to_number)
        .sum()
}

fn main() {
    env_logger::init();
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .collect::<Vec<_>>();

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("test-input-1.txt")
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>();

        let result = part_1(&input);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("test-input-2.txt")
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>();

        let result = part_2(&input);

        assert_eq!(result, 281);
    }
}
