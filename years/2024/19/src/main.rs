use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use aoc_timing::trace::log_run;

struct Input {
    towels: HashSet<String>,
    designs: Vec<String>,
}
type Output1 = usize;
type Output2 = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    enum State {
        Towels(HashSet<String>),
        Designs(HashSet<String>, Vec<String>),
    }
    let end_state = input.fold(State::Towels(HashSet::new()), |state, line| {
        let line = line.as_ref();

        match state {
            State::Towels(mut towels) => {
                if line.is_empty() {
                    State::Designs(towels, vec![])
                } else {
                    towels.extend(line.split(", ").map(|towel| towel.to_string()));
                    State::Towels(towels)
                }
            }
            State::Designs(towels, mut designs) => {
                designs.push(line.to_string());
                State::Designs(towels, designs)
            }
        }
    });

    if let State::Designs(towels, designs) = end_state {
        Input { towels, designs }
    } else {
        panic!("Didn't reach the designs part of the input?");
    }
}

fn design_combinations<'a>(
    design: &'a str,
    towels: &HashSet<String>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(result) = cache.get(design) {
        *result
    } else if design.is_empty() {
        1
    } else {
        let result = towels
            .iter()
            .filter(|t| design.starts_with(*t))
            .map(|t| design_combinations(&design[t.len()..], towels, cache))
            .sum();
        cache.insert(design, result);

        result
    }
}

fn part_1(input: &Input) -> Output1 {
    let mut cache = Default::default();

    input
        .designs
        .iter()
        .filter(|design| design_combinations(design, &input.towels, &mut cache) > 0)
        .count()
}

fn part_2(input: &Input) -> Output2 {
    let mut cache = Default::default();

    input
        .designs
        .iter()
        .map(|design| design_combinations(design, &input.towels, &mut cache))
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

        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 16);
    }
}
