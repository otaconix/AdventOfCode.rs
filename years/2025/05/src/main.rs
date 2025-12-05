use std::io;
use std::ops::RangeInclusive;

use aoc_timing::trace::log_run;
use itertools::Itertools;

type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Ranges(Vec<RangeInclusive<usize>>),
        Ids(Vec<RangeInclusive<usize>>, Vec<usize>),
    }

    use State::*;

    let end_state = input.fold(Ranges(vec![]), |state, line| match state {
        Ranges(mut ranges) => {
            let line = line.as_ref();
            if line.is_empty() {
                Ids(ranges, vec![])
            } else {
                let (start, end) = line.split_once('-').unwrap();
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();

                ranges.push(RangeInclusive::new(start, end));

                Ranges(ranges)
            }
        }
        Ids(ranges, mut ids) => {
            let line = line.as_ref();

            ids.push(line.parse().unwrap());

            Ids(ranges, ids)
        }
    });

    match end_state {
        Ranges(_) => panic!("Parsing failed (only got ranges?)"),
        Ids(ranges, ids) => (ranges, ids),
    }
}

fn part_1((ranges, ids): &Input) -> Output1 {
    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part_2((ranges, _): &Input) -> Output2 {
    ranges
        .iter()
        .sorted_unstable_by_key(|range| range.start())
        .fold((0, 0), |(total, current_min), range| {
            let start = range.start().max(&current_min);

            if start > range.end() {
                (total, current_min)
            } else {
                (total + range.end() - start + 1, range.end() + 1)
            }
        })
        .0
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

        assert_eq!(result, 14);
    }
}
