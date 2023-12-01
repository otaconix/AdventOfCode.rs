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

fn main() {
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .collect::<Vec<_>>();

    let part_1 = input
        .iter()
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>()
        })
        .map(|digits| {
            [digits[0], digits[digits.len() - 1]]
                .iter()
                .collect::<String>()
        })
        .map(|num_str| num_str.parse::<u16>().unwrap())
        .sum::<u16>();

    println!("Part 1: {part_1}");

    let part_2 = input
        .iter()
        .map(|line| replace_spelled_digits(&line.to_string()))
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>()
        })
        .map(|digits| {
            [digits[0], digits[digits.len() - 1]]
                .iter()
                .collect::<String>()
        })
        .map(|num_str| num_str.parse::<u16>().unwrap())
        .sum::<u16>();

    println!("Part 2: {part_2}");
}
