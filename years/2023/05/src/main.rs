use aoc_timing::trace::log_run;
use std::{io, ops::Range};

#[derive(Debug)]
enum ParsingState {
    Start,
    Seeds(Vec<u64>),
    MapTitle(Vec<u64>, Vec<Map>, String, Vec<MapRange>),
    Map(Vec<u64>, Vec<Map>),
}

#[derive(Debug)]
struct MapRange {
    source: Range<u64>,
    dest: Range<u64>,
}

impl MapRange {
    fn map(&self, input: &u64) -> Option<u64> {
        if self.source.contains(input) {
            Some(input - self.source.start + self.dest.start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    title: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn new(title: String, mut ranges: Vec<MapRange>) -> Self {
        ranges.sort_by_key(|range| range.source.start);

        Map { title, ranges }
    }

    fn map(&self, input: &u64) -> u64 {
        self.ranges
            .iter()
            .map(|range| range.map(input))
            .find(|mapped| mapped.is_some())
            .unwrap_or(Some(*input))
            .unwrap()
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn parse_numbers_line(line: &str, skip: usize) -> Vec<u64> {
    line.split(' ')
        .skip(skip)
        .map(|number_string| number_string.parse().expect("Couldn't parse number"))
        .collect()
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Input {
    let parsing_state =
        input
            .map(|line| line.to_string())
            .fold(ParsingState::Start, |state, line| match state {
                ParsingState::Start => ParsingState::Seeds(parse_numbers_line(&line, 1)),
                ParsingState::Seeds(seeds) => {
                    if line.is_empty() {
                        ParsingState::Seeds(seeds)
                    } else {
                        ParsingState::MapTitle(
                            seeds,
                            vec![],
                            line.split(' ').next().expect("No title found").to_string(),
                            vec![],
                        )
                    }
                }
                ParsingState::MapTitle(seeds, mut maps, title, mut ranges) => {
                    if line.is_empty() {
                        maps.push(Map::new(title, ranges));
                        ParsingState::Map(seeds, maps)
                    } else {
                        let raw_range = parse_numbers_line(&line, 0);
                        ranges.push(MapRange {
                            source: raw_range[1]..raw_range[1] + raw_range[2],
                            dest: raw_range[0]..raw_range[0] + raw_range[2],
                        });
                        ParsingState::MapTitle(seeds, maps, title, ranges)
                    }
                }
                ParsingState::Map(seeds, maps) => ParsingState::MapTitle(
                    seeds,
                    maps,
                    line.split(' ').next().expect("No title found").to_string(),
                    vec![],
                ),
            });

    match parsing_state {
        ParsingState::MapTitle(seeds, mut maps, title, ranges) => {
            maps.push(Map::new(title, ranges));
            Input { seeds, maps }
        }
        _ => panic!("Unexpected parsing state: {parsing_state:?}"),
    }
}

fn part_1(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| input.maps.iter().fold(*seed, |input, map| map.map(&input)))
        .min()
        .expect("No mapped seeds?")
}

fn part_2(input: &Input) -> u64 {
    (0..input.seeds.len())
        .step_by(2)
        .flat_map(|seed_index| {
            input.seeds[seed_index]..input.seeds[seed_index] + input.seeds[seed_index + 1]
        })
        .map(|seed| input.maps.iter().fold(seed, |input, map| map.map(&input)))
        .min()
        .expect("No mapped seeds?")
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 46);
    }
}
