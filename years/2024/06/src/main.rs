use std::collections::HashSet;
use std::io;
use std::iter::successors;

use aoc_timing::trace::log_run;
use grid::Grid;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn next_position(&self, position: Coordinates, lab: &Lab) -> Option<Coordinates> {
        match *self {
            Direction::Up if position.1 > 0 => Some((position.0, position.1 - 1)),
            Direction::Left if position.0 > 0 => Some((position.0 - 1, position.1)),
            Direction::Down if position.1 < lab.height() - 1 => Some((position.0, position.1 + 1)),
            Direction::Right if position.0 < lab.width() - 1 => Some((position.0 + 1, position.1)),
            _ => None,
        }
    }

    fn turn(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LabCell {
    Empty,
    Obstruction,
}

type Coordinates = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardPosition {
    coordinates: Coordinates,
    direction: Direction,
}

type Lab = Grid<LabCell>;

impl GuardPosition {
    fn next(&self, lab: &Lab) -> Option<Self> {
        successors(Some(self.direction), |dir| Some(dir.turn()))
            .take(4)
            .map(|dir| {
                dir.next_position(self.coordinates, lab)
                    .map(|pos| (dir, pos))
            })
            .find(|maybe_next| match maybe_next {
                None => true, // Guard got out of the lab
                Some((_, pos)) if *lab.get(pos.0, pos.1).unwrap() != LabCell::Obstruction => true,
                _ => false,
            })
            .flatten()
            .map(|(dir, pos)| Self {
                coordinates: pos,
                direction: dir,
            })
    }
}

#[derive(Debug)]
struct Input {
    lab: Lab,
    guard_start_position: GuardPosition,
    guard_positions: HashSet<Coordinates>,
}

fn all_guard_positions(
    guard_start_position: GuardPosition,
    lab: &Lab,
) -> impl Iterator<Item = GuardPosition> + use<'_> {
    successors(Some(guard_start_position), |guard_pos| guard_pos.next(lab))
}

type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let mut start_position = GuardPosition {
        coordinates: (0, 0),
        direction: Direction::Up,
    };

    let grid = Grid::new(
        input
            .enumerate()
            .map(|(row, line)| {
                let line = line.as_ref();

                line.chars()
                    .enumerate()
                    .map(|(column, c)| match c {
                        '.' => LabCell::Empty,
                        '#' => LabCell::Obstruction,
                        '^' => {
                            start_position = GuardPosition {
                                coordinates: (column, row),
                                direction: Direction::Up,
                            };
                            LabCell::Empty
                        }
                        _ => panic!("Unsupported character found in input: {c}"),
                    })
                    .collect()
            })
            .collect(),
    )
    .expect("Invalid grid");

    Input {
        guard_start_position: start_position,
        guard_positions: successors(Some(start_position), |guard_pos| guard_pos.next(&grid))
            .map(|pos| pos.coordinates)
            .collect::<HashSet<_>>(),
        lab: grid,
    }
}

fn part_1(input: &Input) -> Output {
    input.guard_positions.len()
}

/// Uses the [Tortoise & Hare algorithm]
///
/// [Tortoise & Hare algorithm]: https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare
fn part_2(input: &Input) -> Output {
    input
        .guard_positions
        .par_iter()
        .copied()
        .filter(|coord| coord != &input.guard_start_position.coordinates)
        .map(|(col, row)| {
            let mut lab = input.lab.clone();

            lab.update(col, row, LabCell::Obstruction);

            lab
        })
        .filter(|lab| {
            all_guard_positions(input.guard_start_position, lab)
                .zip(
                    all_guard_positions(input.guard_start_position, lab)
                        .skip(1)
                        .step_by(2),
                )
                .any(|(tortoise, hare)| tortoise == hare)
        })
        .count()
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

        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 6);
    }
}
