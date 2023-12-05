use std::io;

#[derive(Debug)]
enum ParsingState {
    Start,
    Seeds(Vec<u32>),
    MapTitle(Vec<u32>, Vec<Map>, String, Vec<MapRange>),
    Map(Vec<u32>, Vec<Map>),
}

#[derive(Debug)]
struct MapRange {
    source: u32,
    dest: u32,
    length: u32,
}

#[derive(Debug)]
struct Map {
    title: String,
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

fn main() {
    let parsing_state = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .fold(ParsingState::Start, |state, line| match state {
            ParsingState::Start => ParsingState::Seeds(
                line.split(' ')
                    .skip(1)
                    .map(str::parse)
                    .map(|result| result.expect("Couldn't parse seed number"))
                    .collect(),
            ),
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
                    let raw_range = line
                        .split(' ')
                        .map(str::parse)
                        .map(|result| result.expect("Couldn't parse number"))
                        .collect::<Vec<_>>();
                    ranges.push(MapRange {
                        source: raw_range[0],
                        dest: raw_range[1],
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

    println!("Input: {input:#?}");
}
