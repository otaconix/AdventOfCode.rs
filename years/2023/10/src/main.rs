use std::{cmp::Ordering, io};

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Horizontal,
    Ground,
    NorthEast,
    Vertical,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    const INVALID_TILE: &'static str = "Invalid tile";
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile::*;

        match value {
            '-' => Ok(Horizontal),
            '.' => Ok(Ground),
            'L' => Ok(NorthEast),
            '|' => Ok(Vertical),
            'J' => Ok(NorthWest),
            'F' => Ok(SouthEast),
            '7' => Ok(SouthWest),
            'S' => Ok(Start),
            _ => Err(Tile::INVALID_TILE),
        }
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Grid<Tile> {
    Grid::new(
        input
            .map(|line| {
                line.to_string()
                    .chars()
                    .map(|c| c.try_into().unwrap())
                    .collect()
            })
            .collect(),
    )
    .unwrap()
}

fn get_loop(input: &Grid<Tile>) -> Vec<(usize, usize)> {
    let start_coords = input
        .coordinates()
        .find(|(x, y)| input.get(*x, *y).unwrap() == &Tile::Start)
        .expect("Start not found");

    use Direction::*;
    use Tile::*;

    fn next(
        input: &Grid<Tile>,
        (x, y): (usize, usize),
        direction: &Direction,
    ) -> Option<((usize, usize), Direction)> {
        match (input.get(x, y).unwrap(), direction) {
            (Horizontal, East) => ((x + 1, y), East).into(),
            (Horizontal, West) => ((x - 1, y), West).into(),
            (Ground, East) => ((x + 1, y), East).into(),
            (Ground, West) => ((x - 1, y), West).into(),
            (Ground, North) => ((x, y - 1), North).into(),
            (Ground, South) => ((x, y + 1), South).into(),
            (NorthEast, South) => ((x + 1, y), East).into(),
            (NorthEast, West) => ((x, y - 1), North).into(),
            (Vertical, North) => ((x, y - 1), North).into(),
            (Vertical, South) => ((x, y + 1), South).into(),
            (NorthWest, South) => ((x - 1, y), West).into(),
            (NorthWest, East) => ((x, y - 1), North).into(),
            (SouthEast, North) => ((x + 1, y), East).into(),
            (SouthEast, West) => ((x, y + 1), South).into(),
            (SouthWest, North) => ((x - 1, y), West).into(),
            (SouthWest, East) => ((x, y + 1), South).into(),
            (Start, _) => input
                .get_neighbors(x, y)
                .into_iter()
                .map(|(nx, ny)| {
                    (
                        (nx, ny),
                        match (x.cmp(&nx), y.cmp(&ny)) {
                            (_, Ordering::Greater) => North,
                            (_, Ordering::Less) => South,
                            (Ordering::Greater, _) => West,
                            (Ordering::Less, _) => East,
                            _ => panic!(),
                        },
                    )
                })
                .find(|(neighbor, direction)| next(input, *neighbor, direction).is_some()),
            (_, _) => None,
        }
    }

    std::iter::successors(Some((start_coords, North)), |((x, y), direction)| {
        next(input, (*x, *y), direction)
    })
    .map(|step| step.0)
    .enumerate()
    .take_while(|(index, coords)| index == &0 || coords != &start_coords)
    .map(|(_, coords)| coords)
    .collect()
}

/// Shoelace formula
///
/// While it seems to work now, it's not entirely clear to me why...
/// I'm subtracting the loop length from the sum part of the shoelace formula, then adding one when
/// I'm done. I'm probably overlooking something silly, but it works, so meh.
fn cells_within_loop(grid_loop: &[(usize, usize)]) -> i32 {
    (grid_loop
        .iter()
        .cycle()
        .take(grid_loop.len() + 1)
        .map(|(x, y)| (*x as i32, *y as i32))
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| (x1 + x2) * (y2 - y1))
        .sum::<i32>()
        .abs()
        - grid_loop.len() as i32)
        / 2
        + 1
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let grid_loop = get_loop(&input);
    let part_1 = grid_loop.len() / 2;
    println!("Part 1: {part_1}");

    let part_2 = cells_within_loop(&grid_loop);
    println!("Part 2: {part_2}");
}
