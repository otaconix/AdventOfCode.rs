use std::collections::BTreeSet;
use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

type Input = Vec<String>;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input.map(|line| line.as_ref().to_string()).collect()
}

fn part_1(input: &Input) -> usize {
    let (twos, threes) = input
        .iter()
        .map(|id| {
            id.chars()
                .counts()
                .values()
                .unique()
                .copied()
                .collect::<BTreeSet<_>>()
        })
        .fold((0, 0), |(twos, threes), counts| {
            (
                twos + usize::from(counts.contains(&2)),
                threes + usize::from(counts.contains(&3)),
            )
        });

    twos * threes
}

fn part_2(input: &Input) -> String {
    input
        .iter()
        .tuple_combinations::<(_, _)>()
        .find(|(id_a, id_b)| {
            id_a.chars()
                .zip(id_b.chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
        })
        .map(|(id_a, id_b)| {
            id_a.chars()
                .zip(id_b.chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect()
        })
        .unwrap()
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
    const INPUT2: &str = include_str!("test-input2");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT2.lines());
        let result = part_2(&input);

        assert_eq!(result, "fgij".to_string());
    }
}
