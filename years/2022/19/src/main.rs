mod solution;
use aoc_timing::trace::log_run;

use solution::Blueprint;
use std::io;

use crate::solution::Factory;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Vec<Blueprint> {
    input
        .map(|line| line.as_ref().parse().expect("Failed to parse blueprint"))
        .collect()
}

fn part_1(input: &[Blueprint]) -> u32 {
    input
        .iter()
        .map(|blueprint| blueprint.run_simulation(Factory::initial(24)))
        .zip(1..)
        .map(|(i, max_geodes)| i * max_geodes)
        .sum()
}

fn part_2(input: &[Blueprint]) -> u32 {
    input
        .iter()
        .take(3)
        .map(|blueprint| blueprint.run_simulation(Factory::initial(32)))
        .product()
}

fn main() {
    env_logger::init();

    let blueprints: Vec<Blueprint> = log_run("Parsing", || {
        parse(io::stdin().lines().map(|result| result.expect("I/O error")))
    });

    let part_1: u32 = log_run("Part 1", || part_1(&blueprints));
    println!("Part 1: {part_1}");

    let part_2: u32 = log_run("Part 2", || part_2(&blueprints));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 33);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 3472);
    }
}
