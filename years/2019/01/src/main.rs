use std::{borrow::Borrow, io};

use aoc_timing::trace::log_run;

type Input = Vec<u64>;
type Output1 = u64;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.parse().expect("Couldn't parse module mass")
        })
        .collect()
}

fn required_fuel<U: Borrow<u64>>(mass: U) -> u64 {
    (mass.borrow() / 3).saturating_sub(2)
}

fn required_fuel_including_fuel<U: Borrow<u64>>(mass: U) -> u64 {
    if mass.borrow() == &0 {
        0
    } else {
        let fuel = required_fuel(mass);
        fuel + required_fuel_including_fuel(&fuel)
    }
}

fn part_1(input: &Input) -> Output1 {
    input.iter().map(required_fuel).sum()
}

fn part_2(input: &Input) -> Output2 {
    input.iter().map(required_fuel_including_fuel).sum()
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

        assert_eq!(result, 34241);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 51316);
    }
}
