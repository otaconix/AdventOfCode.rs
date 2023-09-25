use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::io;

use coord::Coordinate2D;
use log::info;

const MOVE_LEFT: Coordinate2D = Coordinate2D::new(-1, 0);
const MOVE_RIGHT: Coordinate2D = Coordinate2D::new(1, 0);
const MOVE_DOWN: Coordinate2D = Coordinate2D::new(0, -1);

#[derive(Debug, PartialEq, Eq, Clone)]
struct RockCoords(Coordinate2D);

impl RockCoords {
    fn translate(&self, translation: &Coordinate2D) -> Self {
        Self(self.0.translate(translation))
    }
}

impl Ord for RockCoords {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .y
            .cmp(&other.0.y)
            .reverse()
            .then_with(|| self.0.x.cmp(&other.0.x))
    }
}

impl PartialOrd for RockCoords {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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

#[derive(Debug)]
struct Rock {
    shape: BTreeSet<RockCoords>,
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
                .clone()
                .into_iter()
                .map(|coord| coord.translate(translation))
                .collect(),
            min_x: self.min_x + translation.x,
            max_x: self.max_x + translation.x,
        }
    }

    fn move_by_jet(&self, jet: Jet) -> Self {
        self.translate(&jet.as_translation())
    }
}

struct Well {
    width: u8,
    jets_stream: Box<dyn Iterator<Item = Jet>>,
    settled_rocks: BTreeSet<RockCoords>,
}

impl Well {
    fn new(width: u8, jets: &Vec<Jet>) -> Self {
        Self {
            width,
            jets_stream: Box::new(jets.to_owned().into_iter().cycle()),
            settled_rocks: BTreeSet::new(),
        }
    }

    fn drop_rock(&mut self, rock: &Rock) {
        let initial_translation = Coordinate2D::new(
            2,
            self.settled_rocks.first().map(|c| c.0.y).unwrap_or(-1) + 4,
        );

        let mut rock = rock.clone().translate(&initial_translation);

        loop {
            rock = self
                .jets_stream
                .next()
                .map(|jet| rock.move_by_jet(jet))
                .filter(|r| {
                    r.min_x >= 0
                        && r.max_x < self.width.into()
                        && r.shape.is_disjoint(&self.settled_rocks)
                })
                .unwrap_or(rock);

            let rock_down = rock.translate(&MOVE_DOWN);

            if rock_down.shape.last().unwrap().0.y < 0
                || !rock_down.shape.is_disjoint(&self.settled_rocks)
            {
                break;
            } else {
                rock = rock_down;
            }
        }

        self.settled_rocks.append(&mut rock.shape);
    }
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.settled_rocks.is_empty() {
            writeln!(f, "Empty")?;
        } else {
            for y in (0..=self.settled_rocks.first().unwrap().0.y).rev() {
                for x in 0..self.width as i64 {
                    let coord = RockCoords(Coordinate2D::new(x, y));

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

    let mut well = Well::new(7, &input);
    rocks
        .iter()
        .cycle()
        .take(2022)
        .for_each(|r| well.drop_rock(r));

    info!("Well:\n{well}");
    println!("Part 1: {}", well.settled_rocks.first().unwrap().0.y + 1);

    /*
    let mut well = Well::new(7, &input);
    rocks
        .iter()
        .cycle()
        .take(1_000_000_000_000)
        .for_each(|r| well.drop_rock(r));

    println!("Part 2: {}", well.settled_rocks.first().unwrap().0.y + 1);
    */
}
