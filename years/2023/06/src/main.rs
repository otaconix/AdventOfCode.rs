use aoc_timing::trace::log_run;
use std::{io, iter::successors};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn sqrt(n: u64) -> f64 {
    (n as f64).sqrt()
}

impl Race {
    fn naive_winners(&self) -> u64 {
        (1..self.time)
            .map(|speed| (self.time - speed) * speed)
            .skip_while(|distance| distance <= &self.distance)
            .take_while(|distance| distance > &self.distance)
            .count() as u64
    }

    /// We're basically solving a quadratic formula here
    ///
    /// This doesn't seem to work well for smaller numbers though, and I'm not sure why.
    fn winners(&self) -> u64 {
        let discriminant = (self.time * self.time) - 4 * self.distance;
        let root = sqrt(discriminant);
        let left = (root / 2.0).ceil();
        let right = (root / 2.0).floor();

        (left + right) as u64
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<Race> {
    let number_lines = input
        .map(|line| {
            let mut full_line = line.to_string();
            full_line.retain(|c| c.is_ascii_digit());
            line.to_string()
                .split_whitespace()
                .skip(1)
                .map(|number| number.parse().expect("Couldn't parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    number_lines[0]
        .iter()
        .zip(number_lines[1].iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

fn part_1(input: &[Race]) -> u64 {
    input.iter().map(Race::naive_winners).product()
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
    a * successors(Some(10u64), |n| Some(n * 10))
        .find(|n| n > &b)
        .unwrap()
        + b
}

fn part_2(input: &[Race]) -> u64 {
    let (time, distance) = input.iter().fold((0, 0), |(time, distance), race| {
        (
            concatenate_numbers(time, race.time),
            concatenate_numbers(distance, race.distance),
        )
    });

    Race { time, distance }.winners()
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

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 71503);
    }

    #[allow(dead_code)]
    // #[test]
    fn test_naive_and_quadratic_are_equal() {
        let input = parse(INPUT.lines());
        let naive_results: Vec<_> = input.iter().map(Race::naive_winners).collect();
        let better_results: Vec<_> = input.iter().map(Race::winners).collect();

        assert_eq!(naive_results, better_results);
    }
}
