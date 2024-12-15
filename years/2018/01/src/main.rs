use std::collections::BTreeSet;
use std::io;
use std::ops::ControlFlow;

use aoc_timing::trace::log_run;

type Input = Vec<isize>;
type Output = isize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.parse().expect("Couldn't parse as number")
        })
        .collect()
}

fn part_1(input: &Input) -> Output {
    input.iter().sum()
}

fn part_2(input: &Input) -> Output {
    if let ControlFlow::Break(frequency) = input
        .iter()
        .cycle()
        .scan(0, |frequency, change| {
            Some({
                *frequency += change;
                *frequency
            })
        })
        .try_fold(BTreeSet::new(), |mut seen, frequency| {
            if seen.contains(&frequency) {
                ControlFlow::Break(frequency)
            } else {
                seen.insert(frequency);
                ControlFlow::Continue(seen)
            }
        })
    {
        frequency
    } else {
        panic!()
    }
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

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 2);
    }
}
