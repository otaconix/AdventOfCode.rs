use std::{collections::HashMap, hash::BuildHasher, io};

use aoc_timing::trace::log_run;
use dijkstra::dijkstra;
use rapidhash::RapidHashMap;

type Input = RapidHashMap<String, String>;
type Output1 = usize;
type Output2 = Output1;

static CENTER_OF_MASS: &str = "COM";
static YOU: &str = "YOU";
static SANTA: &str = "SAN";

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            let (center, orbiting) = line.split_once(')').expect("Not an actual direct orbit?");

            (orbiting.to_string(), center.to_string())
        })
        .collect()
}

fn path_len_to_com<S: BuildHasher>(
    map: &HashMap<String, String, S>,
    planet: &str,
    current_length: usize,
) -> usize {
    if planet == CENTER_OF_MASS {
        current_length
    } else {
        path_len_to_com(map, &map[planet], current_length + 1)
    }
}

fn part_1(input: &Input) -> Output1 {
    input
        .keys()
        .map(|planet| path_len_to_com(input, planet, 0))
        .sum()
}

fn part_2(input: &Input) -> Output2 {
    let goal = input.get(SANTA).expect("Santa isn't orbiting any planet?");
    dijkstra(
        YOU,
        |potential_end| potential_end == goal,
        |node| {
            let mut neighbors: Vec<_> = input
                .iter()
                .filter(|(orbiting, center)| center == node)
                .map(|(orbiting, center)| ((*orbiting).as_str(), 1))
                .collect();

            if let Some(next_planet) = input.get(*node) {
                neighbors.push((next_planet.as_str(), 1));
            }

            neighbors.into_iter()
        },
    )
    .unwrap()
    .into_iter()
    .find(|(node, _)| node == goal)
    .unwrap()
    .1 - 1
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

        assert_eq!(result, 42);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT2.lines());
        let result = part_2(&input);

        assert_eq!(result, 4);
    }
}
