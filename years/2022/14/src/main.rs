use coord::Coordinate2D;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

struct RockStructure {
    coords: Vec<Coordinate2D>,
}

impl FromStr for RockStructure {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(" -> ")
            .map(|coord| {
                coord
                    .split_once(',')
                    .ok_or_else(|| "No comma in coordinates?".to_string())
                    .and_then(|(x, y)| {
                        x.parse::<i64>()
                            .and_then(|x| y.parse::<i64>().map(|y| Coordinate2D::new(x, y)))
                            .map_err(|e| e.to_string())
                    })
            })
            .collect::<Result<Vec<Coordinate2D>, String>>()
            .map(|coords| RockStructure { coords })
    }
}

struct RockCoordinates<'a> {
    structure: &'a RockStructure,
    index: usize,
    prev: Option<Coordinate2D>,
}

impl RockStructure {
    fn to_coordinates(&self) -> RockCoordinates {
        RockCoordinates {
            structure: self,
            index: 0,
            prev: None,
        }
    }
}

trait ToStep {
    fn to_step(&self) -> i8;
}

impl ToStep for Ordering {
    fn to_step(&self) -> i8 {
        match *self {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }
}

impl Iterator for RockCoordinates<'_> {
    type Item = Coordinate2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.structure.coords.len() {
            None
        } else {
            match self.prev {
                Some(prev) if prev == self.structure.coords[self.index] => {
                    self.prev = self.structure.coords.get(self.index).copied();
                    self.index += 1;

                    if self.index >= self.structure.coords.len() {
                        self.prev = None
                    }
                }
                Some(prev) => {
                    let next = self.structure.coords[self.index];
                    self.prev = Some(Coordinate2D::new(
                        prev.x + prev.x.cmp(&next.x).to_step() as i64,
                        prev.y + prev.y.cmp(&next.y).to_step() as i64,
                    ));
                }
                None => {
                    self.prev = self.structure.coords.get(self.index).copied();
                    self.index += 1;
                }
            };

            self.prev
        }
    }
}

trait PuzzleCoordinate {
    fn down(&self) -> Self;
    fn down_left(&self) -> Self;
    fn down_right(&self) -> Self;
}

impl PuzzleCoordinate for Coordinate2D {
    fn down(&self) -> Self {
        Coordinate2D::new(self.x, self.y + 1)
    }

    fn down_left(&self) -> Self {
        Coordinate2D::new(self.x - 1, self.y + 1)
    }

    fn down_right(&self) -> Self {
        Coordinate2D::new(self.x + 1, self.y + 1)
    }
}

fn main() {
    let rock_structures: Vec<_> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            line.parse::<RockStructure>()
                .expect("Couldn't parse rock structure")
        })
        .collect();
    let rock_coords = rock_structures
        .iter()
        .flat_map(|structure| structure.to_coordinates())
        .collect::<HashSet<_>>();
    let lowest_rock_y = rock_coords
        .iter()
        .map(|rock| rock.y)
        .max()
        .expect("No rocks?");
    let start_coordinate = Coordinate2D::new(500, 0);

    let mut stage = rock_coords.clone();
    let part_1 = (0..)
        .find(|_| {
            let mut sand = start_coordinate;

            while sand.y < lowest_rock_y {
                if let Some(next) = [sand.down(), sand.down_left(), sand.down_right()]
                    .iter()
                    .find(|next| !stage.contains(next))
                {
                    sand = *next;
                } else {
                    stage.insert(sand);
                    return false;
                }
            }

            true
        })
        .unwrap();

    println!("Part 1: {part_1}");

    let floor_y = lowest_rock_y + 2;
    let mut stage = rock_coords;
    let part_2 = (1..)
        .find(|_| {
            let mut sand = start_coordinate;

            while sand.y + 1 < floor_y {
                if let Some(next) = [sand.down(), sand.down_left(), sand.down_right()]
                    .iter()
                    .find(|next| !stage.contains(next))
                {
                    sand = *next;
                } else {
                    stage.insert(sand);
                    return sand == start_coordinate;
                }
            }

            stage.insert(sand);
            false
        })
        .unwrap();

    println!("Part 2: {part_2}");
}
