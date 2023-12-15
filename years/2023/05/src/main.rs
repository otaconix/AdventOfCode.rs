use aoc_timing::trace::log_run;
use std::io;
use std::ops::Range;

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

    fn map_range(&self, input: &Range<u64>) -> MapRangeResult {
        MapRangeResult {
            before: if input.start < self.source.start {
                Some(input.start..input.end.min(self.source.start))
            } else {
                None
            },
            mapped: if input.start < self.source.end && input.end > self.source.start {
                Some(
                    self.map(&input.start.max(self.source.start)).unwrap()
                        ..self.map(&(input.end.min(self.source.end) - 1)).unwrap() + 1,
                )
            } else {
                None
            },
            after: if input.end > self.source.end {
                Some(input.start.max(self.source.end)..input.end)
            } else {
                None
            },
        }
    }
}

#[derive(Debug)]
struct Map {
    title: String,
    ranges: Vec<MapRange>,
}

struct MapRangeResult {
    before: Option<Range<u64>>,
    mapped: Option<Range<u64>>,
    after: Option<Range<u64>>,
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

    /// returns (mapped, unmapped)
    fn map_range(&self, input: &Range<u64>) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        self.ranges.iter().fold(
            (vec![], vec![input.to_owned()]),
            |(mut mapped, unmapped), map_range| {
                let (mut new_mapped, new_unmapped) = unmapped.into_iter().fold(
                    (vec![], vec![]),
                    |(mut mapped, mut unmapped), range| {
                        let MapRangeResult {
                            before,
                            mapped: new_mapped,
                            after,
                        } = map_range.map_range(&range);
                        if let Some(new_mapped) = new_mapped {
                            mapped.push(new_mapped);
                        }
                        if let Some(before) = before {
                            unmapped.push(before);
                        }
                        if let Some(after) = after {
                            unmapped.push(after);
                        }

                        (mapped, unmapped)
                    },
                );

                mapped.append(&mut new_mapped);

                (mapped, new_unmapped)
            },
        )
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
    let mut seed_ranges = (0..input.seeds.len())
        .step_by(2)
        .map(|seed_index| {
            input.seeds[seed_index]..input.seeds[seed_index] + input.seeds[seed_index + 1]
        })
        .collect::<Vec<_>>();
    seed_ranges.sort_unstable_by_key(|range| range.start);

    input
        .maps
        .iter()
        .fold(seed_ranges, |seed_ranges, map| {
            let (mapped, unmapped) = seed_ranges.into_iter().fold(
                (vec![], vec![]),
                |(mut mapped, unmapped), seed_range| {
                    let (mut new_mapped, new_unmapped) = unmapped
                        .into_iter()
                        .chain(Some(seed_range))
                        .map(|unmapped| map.map_range(&unmapped))
                        .fold((vec![], vec![]), |(mut m1, mut u1), (mut m2, mut u2)| {
                            m1.append(&mut m2);
                            u1.append(&mut u2);

                            (m1, u1)
                        });
                    mapped.append(&mut new_mapped);

                    (mapped, new_unmapped)
                },
            );

            let mut result = mapped.into_iter().chain(unmapped).collect::<Vec<_>>();
            result.sort_unstable_by_key(|range| range.start);
            result
        })
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
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

    impl From<MapRangeResult> for (Option<Range<u64>>, Option<Range<u64>>, Option<Range<u64>>) {
        fn from(val: MapRangeResult) -> Self {
            (val.before, val.mapped, val.after)
        }
    }

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

    #[test]
    fn test_map_range() {
        let map_range = MapRange {
            source: 1..4,
            dest: 5..8,
        };

        let before_mapped_after = map_range.map_range(&(0..6));
        assert_eq!(
            (Some(0..1), Some(5..8), Some(4..6)),
            before_mapped_after.into()
        );

        let before_mapped = map_range.map_range(&(0..2));
        assert_eq!((Some(0..1), Some(5..6), None), before_mapped.into());

        let mapped_after = map_range.map_range(&(1..5));
        assert_eq!((None, Some(5..8), Some(4..5)), mapped_after.into());

        let mapped = map_range.map_range(&(3..4));
        assert_eq!((None, Some(7..8), None), mapped.into());

        let before = map_range.map_range(&(0..1));
        assert_eq!((Some(0..1), None, None), before.into());

        let after = map_range.map_range(&(4..7));
        assert_eq!((None, None, Some(4..7)), after.into());
    }
}
