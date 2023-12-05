use std::io;

#[derive(Debug)]
enum ParsingState {
    Start,
    Seeds(Vec<u64>),
    MapTitle(Vec<u64>, Vec<Map>, String, Vec<MapRange>),
    Map(Vec<u64>, Vec<Map>),
}

#[derive(Debug)]
struct MapRange {
    source: u64,
    dest: u64,
    length: u64,
}

impl MapRange {
    fn map(&self, input: &u64) -> Option<u64> {
        let range = self.source..self.source + self.length;

        if range.contains(input) {
            Some(self.dest + input - self.source)
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

fn main() {
    let parsing_state = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
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
                    maps.push(Map { title, ranges });
                    ParsingState::Map(seeds, maps)
                } else {
                    let raw_range = parse_numbers_line(&line, 0);
                    ranges.push(MapRange {
                        source: raw_range[1],
                        dest: raw_range[0],
                        length: raw_range[2],
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

    let input = match parsing_state {
        ParsingState::MapTitle(seeds, mut maps, title, ranges) => {
            maps.push(Map { title, ranges });
            Input { seeds, maps }
        }
        _ => panic!("Unexpected parsing state: {parsing_state:?}"),
    };

    let part_1 = input
        .seeds
        .iter()
        .map(|seed| input.maps.iter().fold(*seed, |input, map| map.map(&input)))
        .min()
        .expect("No mapped seeds?");

    println!("Part 1: {part_1}");
}
