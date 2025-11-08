use std::{io, ops::RangeInclusive};

use aoc_timing::trace::log_run;

type Input = RangeInclusive<u64>;
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(mut input: I) -> Input {
    let line = input.next().unwrap().as_ref().to_string();
    let (from, to) = line.split_once('-').unwrap();

    from.parse().unwrap()..=to.parse().unwrap()
}

fn digits(mut n: u64) -> Vec<u8> {
    if n == 0 {
        vec![0]
    } else {
        let mut result = vec![];

        while n > 0 {
            result.push((n % 10) as u8);
            n /= 10;
        }

        result.reverse();

        result
    }
}

fn is_valid_password(input: u64) -> bool {
    let digits = digits(input);

    digits.windows(2).all(|pair| pair[0] <= pair[1])
        && digits.windows(2).any(|pair| pair[0] == pair[1])
}

fn is_valid_password2(input: u64) -> bool {
    let digits = digits(input);

    digits.windows(2).all(|pair| pair[0] <= pair[1])
        && digits.iter().enumerate().any(|(index, n)| match index {
            0 => *n == digits[index + 1] && *n != digits[index + 2],
            4 => *n == digits[index + 1] && *n != digits[index - 1],
            5 => false,
            _ => digits[index - 1] != *n && *n == digits[index + 1] && *n != digits[index + 2],
        })
}

fn part_1(input: &Input) -> Output1 {
    input
        .clone()
        .filter(|password| is_valid_password(*password))
        .count()
}

fn part_2(input: &Input) -> Output2 {
    input
        .clone()
        .filter(|password| is_valid_password2(*password))
        .count()
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
