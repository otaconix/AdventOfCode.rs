use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::io;

use coord::Coordinate2D;
use log::info;

const MOVE_LEFT: Coordinate2D = Coordinate2D::new(-1, 0);
const MOVE_RIGHT: Coordinate2D = Coordinate2D::new(1, 0);
const MOVE_DOWN: Coordinate2D = Coordinate2D::new(0, -1);
const PART1_ROCK_COUNT: usize = 2022;
#[allow(dead_code)] // TODO: remove when part 2 is done
const PART2_ROCK_COUNT: usize = 1_000_000_000_000;
const WELL_WIDTH: i64 = 7;

/// Coordinates of a rock
///
/// By applying the _newtype_ pattern here, we can
/// implement Ord for the underlying [Coordinate2D].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RockCoords(Coordinate2D);

impl RockCoords {
    fn new(x: i64, y: i64) -> Self {
        RockCoords(Coordinate2D::new(x, y))
    }

    fn translate(&self, translation: &Coordinate2D) -> Self {
        Self(self.0.translate(translation))
    }
}

/// Orders the `RockCoords` by:
///   1. `y`, descending (so highest first)
///   2. then `x`, ascending
impl Ord for RockCoords {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.y.cmp(&other.0.y).reverse() {
            Ordering::Equal => self.0.x.cmp(&other.0.x),
            ord => ord,
        }
    }
}

impl PartialOrd for RockCoords {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn as_translation(&self) -> Coordinate2D {
        match self {
            Jet::Left => MOVE_LEFT,
            Jet::Right => MOVE_RIGHT,
        }
    }
}

impl TryFrom<char> for Jet {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Jet::Left),
            '>' => Ok(Jet::Right),
            _ => Err(format!("Invalid jet character: {value}")),
        }
    }
}

/// A rock that we will be dropping down the well
///
/// By keeping track of the top left- and rightmost corners, we can
/// quickly check if the rock would go out of bounds.
#[derive(Debug)]
struct Rock {
    shape: Vec<RockCoords>,
    min_x: i64,
    max_x: i64,
}

impl Rock {
    fn new(shape: Vec<Coordinate2D>) -> Self {
        Self {
            min_x: shape.iter().min_by_key(|c| c.x).unwrap().x,
            max_x: shape.iter().max_by_key(|c| c.x).unwrap().x,
            shape: shape.into_iter().map(RockCoords).collect(),
        }
    }

    fn translate(&self, translation: &Coordinate2D) -> Self {
        Self {
            shape: self
                .shape
                .iter()
                .copied()
                .map(|coord| coord.translate(translation))
                .collect(),
            min_x: self.min_x + translation.x,
            max_x: self.max_x + translation.x,
        }
    }
}

/// The well we will drop rocks into.
///
/// By keeping tracks of the coordinates of each part of
/// the settled rocks in a [BTreeSet], we can efficiently
/// do a collision check, since there's no need to check for
/// collisions past each part of a rock.
struct Well {
    settled_rocks: BTreeSet<RockCoords>,
}

impl Well {
    fn new() -> Self {
        Self {
            settled_rocks: BTreeSet::new(),
        }
    }

    fn drop_rock<'a, T>(&mut self, rock: &Rock, jets: &mut T)
    where
        T: Iterator<Item = &'a Jet>,
    {
        // No need to check for collisions with settled rocks for the first three
        // movevements, so we just do the horizontal movements first
        let initial_translation = Coordinate2D::new(
            2,
            self.settled_rocks.first().map(|c| c.0.y).unwrap_or(-1) + 1,
        );

        let mut rock = rock.translate(&initial_translation);

        for _ in 0..3 {
            let jet_translation = jets.next().unwrap().as_translation();
            let rock_sideways = rock.translate(&jet_translation);
            if self.rock_is_in_bounds(&rock_sideways) {
                rock = rock_sideways;
            }
        }

        loop {
            let jet_translation = jets.next().unwrap().as_translation();
            let rock_sideways = rock.translate(&jet_translation);
            if self.rock_is_in_bounds(&rock_sideways)
                && !self.rock_collides_with_settled_rocks(&rock_sideways)
            {
                rock = rock_sideways;
            }

            let rock_down = rock.translate(&MOVE_DOWN);
            if rock_down.shape.last().unwrap().0.y < 0
                || rock_down
                    .shape
                    .iter()
                    .any(|coord| self.settled_rocks.contains(coord))
            {
                break;
            } else {
                rock = rock_down;
            }
        }

        self.settled_rocks.extend(rock.shape);
    }

    fn rock_is_in_bounds(&self, rock: &Rock) -> bool {
        rock.min_x >= 0 && rock.max_x < WELL_WIDTH
    }

    fn rock_collides_with_settled_rocks(&self, rock: &Rock) -> bool {
        rock.shape.iter().any(|c| self.settled_rocks.contains(c))
    }
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.settled_rocks.is_empty() {
            writeln!(f, "Empty")?;
        } else {
            for y in (0..=self.settled_rocks.first().unwrap().0.y).rev() {
                for x in 0..WELL_WIDTH {
                    let coord = RockCoords::new(x, y);

                    if self.settled_rocks.contains(&coord) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn main() {
    env_logger::init();

    let rocks: Vec<Rock> = vec![
        // -
        Rock::new(vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(1, 0),
            Coordinate2D::new(2, 0),
            Coordinate2D::new(3, 0),
        ]),
        // +
        Rock::new(vec![
            Coordinate2D::new(1, 0),
            Coordinate2D::new(0, 1),
            Coordinate2D::new(1, 1),
            Coordinate2D::new(2, 1),
            Coordinate2D::new(1, 2),
        ]),
        // ⅃
        Rock::new(vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(1, 0),
            Coordinate2D::new(2, 0),
            Coordinate2D::new(2, 1),
            Coordinate2D::new(2, 2),
        ]),
        // |
        Rock::new(vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(0, 1),
            Coordinate2D::new(0, 2),
            Coordinate2D::new(0, 3),
        ]),
        // _
        Rock::new(vec![
            Coordinate2D::new(0, 0),
            Coordinate2D::new(1, 0),
            Coordinate2D::new(0, 1),
            Coordinate2D::new(1, 1),
        ]),
    ];

    let input: Vec<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .flat_map(|line| line.chars().map(Jet::try_from).collect::<Vec<_>>())
        .collect::<Result<Vec<_>, String>>()
        .expect("Invalid input");

    let mut well = Well::new();
    let mut jets = input.iter().cycle();
    rocks
        .iter()
        .cycle()
        .take(PART1_ROCK_COUNT)
        .for_each(|r| well.drop_rock(r, &mut jets));

    info!("Well:\n{well}");
    println!("Part 1: {}", well.settled_rocks.first().unwrap().0.y + 1);

    /*
    let mut jets = input.iter().cycle();
    let mut well = Well::new();
    rocks
        .iter()
        .cycle()
        .take(PART2_ROCK_COUNT)
        .for_each(|r| well.drop_rock(r, &mut jets));

    println!("Part 2: {}", well.settled_rocks.first().unwrap().0.y + 1);
    */
}
