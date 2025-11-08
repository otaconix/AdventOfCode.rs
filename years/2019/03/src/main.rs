use std::{collections::HashSet, io, str::FromStr};

use aoc_timing::trace::log_run;
use rapidhash::{RapidHashMap, RapidHashSet};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err("Unknown direction: {s}".into()),
        }
    }
}

#[derive(Debug)]
struct Step(Direction, i64);

impl Step {
    fn parse(input: &str) -> Self {
        let (direction, count) = input.split_at(1);

        Step(
            Direction::from_str(direction).expect("invalid direction"),
            count.parse().expect("Invalid count"),
        )
    }

    fn points_from(&self, (x, y): (i64, i64)) -> Vec<(i64, i64)> {
        use Direction::*;

        match self.0 {
            Left => (x - self.1..=x - 1).rev().map(|x| (x, y)).collect(),
            Right => (x + 1..=x + self.1).map(|x| (x, y)).collect(),
            Up => (y - self.1..=y - 1).rev().map(|y| (x, y)).collect(),
            Down => (y + 1..=y + self.1).map(|y| (x, y)).collect(),
        }
    }
}

#[derive(Debug)]
struct Wire {
    steps: Vec<Step>,
}

impl Wire {
    fn points(&self) -> Vec<(i64, i64)> {
        self.steps
            .iter()
            .fold(((0, 0), vec![]), |(start, mut points), step| {
                let step_points = step.points_from(start);
                let last_point = step_points.last().cloned().unwrap();
                points.extend(step_points);

                (last_point, points)
            })
            .1
    }
}

type Input = Vec<Wire>;
type Output1 = i64;
type Output2 = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| Wire {
            steps: line
                .as_ref()
                .split(',')
                .map(|step| Step::parse(step))
                .collect(),
        })
        .collect()
}

fn part_1(input: &Input) -> Output1 {
    let mut points_a = input[0].points().into_iter().collect::<RapidHashSet<_>>();
    let points_b = input[1].points().into_iter().collect::<RapidHashSet<_>>();

    points_a.retain(|point| points_b.contains(point));

    points_a
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn part_2(input: &Input) -> Output2 {
    let mut points_a = input[0].points().into_iter().enumerate().fold(
        RapidHashMap::default(),
        |mut map, (steps, point)| {
            map.entry(point).or_insert(steps);

            map
        },
    );
    let points_b = input[1].points().into_iter().enumerate().fold(
        RapidHashMap::default(),
        |mut map, (steps, point)| {
            map.entry(point).or_insert(steps);

            map
        },
    );

    points_a.retain(|k, _| points_b.contains_key(k));

    points_a
        .into_iter()
        .map(|(point, steps)| steps + points_b[&point])
        .min()
        .unwrap()
        + 2
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

        assert_eq!(result, 159);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 610);
    }
}
