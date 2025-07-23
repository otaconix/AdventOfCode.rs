use aoc_timing::trace::log_run;
use coord::Coordinate2D;
use std::collections::HashSet;
use std::io;
use std::num::NonZeroUsize;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rope {
    knots: Vec<Coordinate2D>,
}

impl Rope {
    fn new(length: NonZeroUsize) -> Rope {
        Rope {
            knots: std::iter::repeat_n(Coordinate2D::new(0, 0), length.into())
                .collect(),
        }
    }

    fn tail(&self) -> &Coordinate2D {
        self.knots.last().unwrap()
    }

    fn move_in_direction(&mut self, direction: Direction) {
        let head = self.knots.first_mut().unwrap();

        match direction {
            Direction::Up => head.y += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
        };

        for knot_index in 1..self.knots.len() {
            let head = self.knots[knot_index - 1];
            let tail = self.knots.get_mut(knot_index).unwrap();

            let vertical_delta = head.y - tail.y;
            let horizontal_delta = head.x - tail.x;

            match (vertical_delta.abs(), horizontal_delta.abs()) {
                (2, 2) | (1, 2) | (2, 1) => {
                    tail.x += horizontal_delta.signum();
                    tail.y += vertical_delta.signum();
                }
                (0, 2) => tail.x += horizontal_delta.signum(),
                (2, 0) => tail.y += vertical_delta.signum(),
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    distance: i64,
    direction: Direction,
}

struct RepeatedDirection {
    direction: Direction,
    count: i64,
}

impl Iterator for RepeatedDirection {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= 0 {
            None
        } else {
            self.count -= 1;
            Some(self.direction)
        }
    }
}

impl Move {
    fn as_repeated_direction(&self) -> RepeatedDirection {
        RepeatedDirection {
            direction: self.direction,
            count: self.distance,
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_once(' ').ok_or("Invalid move: {s}")?;
        let distance: i64 = distance
            .parse()
            .map_err(|e| format!("Couldn't parse count {distance}: {e}"))?;

        match direction {
            "U" => Ok(Move {
                distance,
                direction: Direction::Up,
            }),
            "D" => Ok(Move {
                distance,
                direction: Direction::Down,
            }),
            "L" => Ok(Move {
                distance,
                direction: Direction::Left,
            }),
            "R" => Ok(Move {
                distance,
                direction: Direction::Right,
            }),
            _ => Err(format!("Unknown move direction: {direction}")),
        }
    }
}

fn main() {
    env_logger::init();

    let moves: Vec<Move> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse().unwrap())
        .collect();

    let part_1: HashSet<_> = log_run("Part 1", || {
        moves
            .iter()
            .flat_map(|mov| mov.as_repeated_direction())
            .scan(Rope::new(2.try_into().unwrap()), |rope, direction| {
                rope.move_in_direction(direction);
                Some(*rope.tail())
            })
            .collect()
    });

    println!("Part 1: {}", part_1.len());

    let part_2: HashSet<_> = log_run("Part 2", || {
        moves
            .iter()
            .flat_map(|mov| mov.as_repeated_direction())
            .scan(
                Rope::new(NonZeroUsize::new(10).unwrap()),
                |rope, direction| {
                    rope.move_in_direction(direction);
                    Some(*rope.tail())
                },
            )
            .collect()
    });

    println!("Part 2: {}", part_2.len());
}
