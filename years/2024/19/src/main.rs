use std::io;

use aoc_timing::trace::log_run;
use fxhash::FxHashMap;
use fxhash::FxHashSet;

type Input = Vec<usize>;
type Output1 = usize;
type Output2 = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Towels(FxHashSet<String>),
        Designs(FxHashSet<String>, Vec<String>),
    }
    let end_state = input.fold(State::Towels(Default::default()), |state, line| {
        let line = line.as_ref();

        match state {
            State::Towels(mut towels) => {
                if line.is_empty() {
                    State::Designs(towels, vec![])
                } else {
                    towels.extend(line.split(", ").map(std::string::ToString::to_string));
                    State::Towels(towels)
                }
            }
            State::Designs(towels, mut designs) => {
                designs.push(line.to_string());
                State::Designs(towels, designs)
            }
        }
    });

    match end_state { State::Designs(towels, designs) => {
        let mut cache = Default::default();
        let towels: FxHashMap<_, Vec<_>> =
            towels
                .into_iter()
                .fold(Default::default(), |mut map, towel| {
                    let ts = map.entry(towel.as_bytes()[0]).or_default();
                    ts.push(towel);

                    map
                });

        designs
            .iter()
            .map(|design| design_combinations(design, &towels, &mut cache))
            .collect()
    } _ => {
        panic!("Didn't reach the designs part of the input?");
    }}
}

fn design_combinations<'a>(
    design: &'a str,
    towels: &FxHashMap<u8, Vec<String>>,
    cache: &mut FxHashMap<&'a str, usize>,
) -> usize {
    if let Some(result) = cache.get(design) {
        *result
    } else if design.is_empty() {
        1
    } else {
        let result = towels
            .get(&design.as_bytes()[0])
            .map_or(0, |ts| {
                ts.iter()
                    .filter(|t| design.starts_with(*t))
                    .map(|t| design_combinations(&design[t.len()..], towels, cache))
                    .sum()
            });
        cache.insert(design, result);

        result
    }
}

fn part_1(input: &Input) -> Output1 {
    input
        .iter()
        .filter(|combinations| **combinations > 0)
        .count()
}

fn part_2(input: &Input) -> Output2 {
    input.iter().sum()
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

        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 16);
    }
}
