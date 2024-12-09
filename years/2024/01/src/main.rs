use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

#[derive(Debug)]
struct LocationIdLists {
    left: Vec<u32>,
    right: Vec<u32>,
}

type Input = LocationIdLists;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (left, right) = input
        .map(|line| {
            let line = line.as_ref();

            let (left, right) = line.split_once("   ").expect("Couldn't split line");

            (
                left.parse::<u32>().expect("Couldn't parse left number"),
                right.parse::<u32>().expect("Couldn't parse right number"),
            )
        })
        .unzip();

    LocationIdLists { left, right }
}

fn part_1(input: &Input) -> u32 {
    let (mut left, mut right) = (input.left.clone(), input.right.clone());

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part_2(input: &Input) -> u32 {
    let right_counts = input.right.iter().counts();

    input
        .left
        .iter()
        .map(|left| left * *right_counts.get(left).unwrap_or(&0) as u32)
        .sum()
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

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 31);
    }
}
