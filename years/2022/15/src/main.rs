use std::{collections::HashSet, io, ops::Range, str::FromStr};

use coord::Coordinate2D;

#[derive(Debug)]
struct Sensor {
    own_coord: Coordinate2D,
    closest_beacon_coord: Coordinate2D,
}

impl Sensor {
    fn distance_to_beacon(&self) -> u64 {
        self.own_coord
            .manhattan_distance(&self.closest_beacon_coord)
    }

    fn impossible_range_on_row(&self, row: i64) -> Range<i64> {
        let y_distance_from_row = self.own_coord.y.abs_diff(row);
        let distance_to_beacon = self.distance_to_beacon();

        if distance_to_beacon >= y_distance_from_row {
            let diff = (distance_to_beacon - y_distance_from_row) as i64;

            (self.own_coord.x - diff)..(self.own_coord.x + diff + 1)
        } else {
            0..0
        }
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pom::char_class::*;
        use pom::parser::*;

        let number = || {
            (sym(b'-').opt() + is_a(digit).repeat(1..)).map(|(sign, digits)| {
                sign.map_or(1, |_| -1)
                    * digits
                        .iter()
                        .skip_while(|c| *c == &b'0')
                        .fold(0i64, |result, digit| result * 10 + (digit - b'0') as i64)
            })
        };
        let coordinate = || {
            ((seq(b"x=") * number()) + (seq(b", y=") * number()))
                .map(|(x, y)| Coordinate2D::new(x, y))
        };
        let sensor = ((seq(b"Sensor at ") * coordinate())
            + (seq(b": closest beacon is at ") * coordinate()))
        .map(|(own, beacon)| Sensor {
            own_coord: own,
            closest_beacon_coord: beacon,
        });

        sensor.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

fn ranges_overlap(left: &Range<i64>, right: &Range<i64>) -> bool {
    left.contains(&right.start) || left.contains(&right.end)
}

fn remove_overlaps(mut ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    ranges.sort_by_key(|range| range.start);
    ranges.iter().fold(vec![], |mut acc, range| {
        if !acc
            .last()
            .map(|last| ranges_overlap(last, range))
            .unwrap_or(false)
        {
            acc.push(range.clone());
        } else {
            let last = acc.last_mut().unwrap();
            last.end = last.end.max(range.end);
        }

        acc
    })
}

fn main() {
    let sensors = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();
    let unique_beacons: HashSet<Coordinate2D> = sensors
        .iter()
        .map(|sensor| sensor.closest_beacon_coord)
        .collect();
    let part_1_row = 2_000_000;

    let part_1: usize = remove_overlaps(
        sensors
            .iter()
            .map(|sensor| sensor.impossible_range_on_row(part_1_row))
            .filter(|range| !range.is_empty())
            .collect(),
    )
    .iter()
    .map(|range| range.clone().count())
    .sum::<usize>()
        - unique_beacons
            .iter()
            .filter(|beacon| beacon.y == part_1_row)
            .count();

    println!("Part 1: {part_1:#?}");

    let part_2 = (0..=4_000_000)
        .map(|row| {
            let impossible_in_row = sensors
                .iter()
                .map(|sensor| {
                    let mut range = sensor.impossible_range_on_row(row);
                    range.start = range.start.clamp(0, 4_000_000);
                    range.end = range.end.clamp(0, 4_000_000);

                    range
                })
                .filter(|range| !range.is_empty())
                .collect::<Vec<_>>();

            remove_overlaps(impossible_in_row)
        })
        .enumerate()
        .find(|(_, impossible_in_row)| impossible_in_row.len() > 1)
        .map(|(y, x)| x[0].end * 4_000_000 + y as i64)
        .unwrap();

    println!("Part 2: {part_2:?}");
}
