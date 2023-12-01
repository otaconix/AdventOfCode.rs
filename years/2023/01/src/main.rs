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

fn input_to_number<S: ToString>(input: S) -> u16 {
    let input = input.to_string();
    let mut digit_iter = input.chars().filter(char::is_ascii_digit);

    let first_digit = digit_iter.next().unwrap();

    [first_digit, digit_iter.nth_back(0).unwrap_or(first_digit)]
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .collect::<Vec<_>>();

    let part_1 = input.iter().map(input_to_number).sum::<u16>();

    println!("Part 1: {part_1}");

    let part_2 = input
        .iter()
        .map(|line| replace_spelled_digits(&line.to_string()))
        .map(input_to_number)
        .sum::<u16>();

    println!("Part 2: {part_2}");
}
