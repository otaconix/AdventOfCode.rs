use std::{io, str::FromStr};

use coord::Coordinate2D;

#[derive(Debug)]
struct Sensor {
    own_coord: Coordinate2D,
    closest_beacon_coord: Coordinate2D,
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

fn main() {
    let sensors = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    println!("Sensors: {sensors:#?}");
}
