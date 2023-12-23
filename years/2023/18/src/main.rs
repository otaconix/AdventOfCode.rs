use std::io;

use aoc_timing::trace::log_run;
use itertools::Itertools;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct DigInstruction {
    direction: Direction,
    distance: i64,
    color: String,
}

type Input = Vec<DigInstruction>;

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.to_string();
            let split = line.split(' ').collect::<Vec<_>>();
            let (direction, distance, color) = (split[0], split[1], split[2]);

            let direction = match direction {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Invalid direction {direction}"),
            };

            let length: i64 = distance.parse().expect("Invalid length");

            let color = color[2..].chars().take(6).collect();

            DigInstruction {
                direction,
                distance: length,
                color,
            }
        })
        .collect::<Input>()
}

fn dug_out_circumference(polygon: &[(i64, i64)]) -> i64 {
    polygon
        .iter()
        .copied()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) as i64 + y1.abs_diff(y2) as i64)
        .sum()
}

/// Shoelace formula
///
/// We want the area *including* the circumference.
fn dug_out_area(polygon: &[(i64, i64)]) -> i64 {
    (polygon
        .iter()
        .copied()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| (x1 + x2) * (y2 - y1))
        .sum::<i64>()
        .abs()
        + dug_out_circumference(polygon))
        / 2
        + 1
}

fn make_polygon(input: &Input) -> Vec<(i64, i64)> {
    input
        .iter()
        .fold(vec![(0i64, 0i64)], |mut result, instruction| {
            let (last_x, last_y) = result.last().copied().unwrap();
            let delta = instruction.distance;

            result.push(match instruction.direction {
                Direction::Left => (last_x - delta, last_y),
                Direction::Right => (last_x + delta, last_y),
                Direction::Down => (last_x, last_y - delta),
                Direction::Up => (last_x, last_y + delta),
            });

            result
        })
}

fn part_1(input: &Input) -> i64 {
    dug_out_area(&make_polygon(input))
}

fn part_2(input: &Input) -> i64 {
    let input = input
        .iter()
        .map(|instruction| {
            let (distance, direction) = &instruction.color.split_at(5);
            let distance = i64::from_str_radix(distance, 16).unwrap();
            let direction = match *direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Unknown encoded direction: {direction}"),
            };

            DigInstruction {
                direction,
                distance,
                color: "".to_string(),
            }
        })
        .collect::<Vec<_>>();

    dug_out_area(&make_polygon(&input))
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

        assert_eq!(result, 62);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 952408144115);
    }
}
