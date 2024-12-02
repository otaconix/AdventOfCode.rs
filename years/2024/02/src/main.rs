use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

type Input = Vec<Vec<i32>>;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.split_whitespace()
                .map(|num| num.parse().expect("Couldn't parse number"))
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<_> = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    diffs
        .iter()
        .all(|diff| *diff >= -3 && *diff <= 3 && *diff != 0)
        && diffs
            .iter()
            .map(|diff| diff.signum())
            .tuple_windows()
            .all(|(sign_a, sign_b)| sign_a == sign_b)
}

fn part_1(input: &Input) -> usize {
    input.iter().filter(|report| is_safe(report)).count()
}

fn is_safe_with_removal(report: &[i32]) -> bool {
    if is_safe(report) {
        true
    } else {
        for i in 0..report.len() {
            let mut after_removal = report[0..i].to_vec();
            after_removal.append(&mut report[i + 1..].to_vec());

            if is_safe(&after_removal) {
                return true;
            }
        }

        false
    }
}

fn part_2(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| is_safe_with_removal(report))
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 4);
    }
}
