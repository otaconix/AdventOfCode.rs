use aoc_timing::trace::log_run;
use std::io;
use std::ops::RangeInclusive;

struct SectionIdRange {
    start: u32,
    end: u32,
}

impl std::str::FromStr for SectionIdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once('-') {
            Result::Ok(SectionIdRange {
                start: start
                    .parse()
                    .unwrap_or_else(|_| panic!("Couldn't parse range start: {}", start)),
                end: end
                    .parse()
                    .unwrap_or_else(|_| panic!("Couldn't parse range end: {}", end)),
            })
        } else {
            Result::Err(format!("Couldn't parse SectionIdRange: {}", s))
        }
    }
}

impl From<SectionIdRange> for RangeInclusive<u32> {
    fn from(range: SectionIdRange) -> Self {
        range.start..=range.end
    }
}

fn main() {
    env_logger::init();

    let paired_ranges: Vec<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            let (range_a, range_b) = line
                .split_once(',')
                .unwrap_or_else(|| panic!("Couldn't split input line into two: {}", line));

            let range_a: RangeInclusive<_> = range_a.parse::<SectionIdRange>().unwrap().into();
            let range_b: RangeInclusive<_> = range_b.parse::<SectionIdRange>().unwrap().into();

            (range_a, range_b)
        })
        .collect();

    let part_1 = log_run("Part 1", || {
        paired_ranges
            .iter()
            .filter(|(range_a, range_b)| {
                range_a.to_owned().all(|a| range_b.contains(&a))
                    || range_b.to_owned().all(|b| range_a.contains(&b))
            })
            .count()
    });

    println!("Part 1: {}", part_1);

    let part_2 = log_run("Part 2", || {
        paired_ranges
            .iter()
            .filter(|(range_a, range_b)| range_a.to_owned().any(|a| range_b.contains(&a)))
            .count()
    });

    println!("Part 2: {}", part_2);
}
