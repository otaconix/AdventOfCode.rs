use std::io;

use aoc_timing::trace::log_run;
use geo::Area;
use geo::BooleanOps;
use geo::HasDimensions;
use geo::Intersects;
use geo::MultiPolygon;
use geo::Polygon;
use geo::Rect;
use itertools::Itertools;

struct Claim {
    id: usize,
    rect: Polygon,
}

/// Adds the extension method `none` to an iterator.
///
/// Example:
/// ```rust
/// let arr = [1, 3, 5];
/// assert_eq!(
///     !arr.iter().any(|n| n % 2 == 0),
///     arr.iter().none(|n| n % 2 == 0)
/// );
/// ```
#[allow(dead_code)]
trait NoneIterator: Iterator {
    fn none<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        !self.any(f)
    }
}

impl<T> NoneIterator for T where T: Iterator + Sized {}

type Input = Vec<Claim>;
type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            let (id, rest) = line.split_once(" @ ").unwrap();
            let id = id[1..].parse().unwrap();
            let (offset, size) = rest.split_once(": ").unwrap();
            let (x, y) = offset.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let (width, height) = size.split_once('x').unwrap();
            let width = width.parse::<f64>().unwrap();
            let height = height.parse::<f64>().unwrap();

            Claim {
                id,
                rect: Rect::new((x, y), (x + width, y + height)).to_polygon(),
            }
        })
        .collect()
}

fn part_1(input: &Input) -> Output {
    input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.rect.intersection(&b.rect))
        .filter(|intersection| !intersection.is_empty())
        .fold(MultiPolygon::new(vec![]), |result, intersection| {
            result.union(&intersection)
        })
        .unsigned_area() as usize
}

fn part_2(input: &Input) -> Output {
    input
        .iter()
        .find(|claim| {
            input
                .iter()
                .filter(|other| claim.id != other.id)
                .none(|other| claim.rect.intersects(&other.rect))
        })
        .map(|claim| claim.id)
        .unwrap()
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

        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 3);
    }
}
