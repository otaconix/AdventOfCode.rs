use std::collections::{HashMap, HashSet};
use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;
use log::debug;

#[derive(Debug)]
enum Splitter {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
enum Mirror {
    Backslash,
    Slash,
}

#[derive(Debug)]
enum Cell {
    Empty,
    Splitter(Splitter),
    Mirror(Mirror),
}

impl Cell {
    fn next_direction(&self, direction: Direction) -> (Direction, Option<Direction>) {
        use Direction::*;
        use Mirror::*;
        use Splitter::*;

        match self {
            Cell::Empty => (direction, None),
            Cell::Splitter(splitter) => match (splitter, direction) {
                (Horizontal, Left | Right) => (direction, None),
                (Horizontal, _) => (Left, Some(Right)),
                (Vertical, Up | Down) => (direction, None),
                (Vertical, _) => (Down, Some(Up)),
            },
            Cell::Mirror(mirror) => match (mirror, direction) {
                (Slash, Left) => (Down, None),
                (Slash, Right) => (Up, None),
                (Slash, Down) => (Left, None),
                (Slash, Up) => (Right, None),
                (Backslash, Left) => (Up, None),
                (Backslash, Right) => (Down, None),
                (Backslash, Down) => (Right, None),
                (Backslash, Up) => (Left, None),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Input = Grid<Cell>;

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            line.to_string()
                .chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '-' => Cell::Splitter(Splitter::Horizontal),
                    '|' => Cell::Splitter(Splitter::Vertical),
                    '/' => Cell::Mirror(Mirror::Slash),
                    '\\' => Cell::Mirror(Mirror::Backslash),
                    _ => panic!("Unknown cell {c}"),
                })
                .collect()
        })
        .collect()
}

type DirectedPosition = (Direction, (usize, usize));

fn advance_beam(
    (direction, (column, row)): DirectedPosition,
    grid: &Grid<Cell>,
) -> Option<(DirectedPosition, Option<DirectedPosition>)> {
    if let Some((column, row)) = match direction {
        Direction::Up => {
            if row == 0 {
                None
            } else {
                Some((column, row - 1))
            }
        }
        Direction::Down => {
            if row + 1 >= grid.height() {
                None
            } else {
                Some((column, row + 1))
            }
        }
        Direction::Left => {
            if column == 0 {
                None
            } else {
                Some((column - 1, row))
            }
        }
        Direction::Right => {
            if column + 1 >= grid.width() {
                None
            } else {
                Some((column + 1, row))
            }
        }
    } {
        let (next_direction, new_beam_direction) =
            grid.get(column, row).unwrap().next_direction(direction);

        Some((
            (next_direction, (column, row)),
            new_beam_direction.map(|new_beam_direction| (new_beam_direction, (column, row))),
        ))
    } else {
        None
    }
}

fn log_grid(
    input: &Input,
    energized_tiles: &HashSet<&(usize, usize)>,
    seen_states: &HashSet<DirectedPosition>,
) {
    if log::log_enabled!(log::Level::Debug) {
        let cell_directions = seen_states.iter().fold(
            HashMap::<(usize, usize), Vec<Direction>>::new(),
            |mut map, (direction, position)| {
                if let Some(directions) = map.get_mut(position) {
                    directions.push(*direction);
                } else {
                    map.insert(*position, vec![*direction]);
                }

                map
            },
        );

        debug!(
            "Direction:\n{}",
            (0..input.height())
                .map(|row| (0..input.width())
                    .map(|column| {
                        let cell = input.get(column, row).unwrap();

                        match cell {
                            Cell::Mirror(Mirror::Slash) => '/',
                            Cell::Mirror(Mirror::Backslash) => '\\',
                            Cell::Splitter(Splitter::Horizontal) => '-',
                            Cell::Splitter(Splitter::Vertical) => '|',
                            Cell::Empty => {
                                if let Some(directions) = cell_directions.get(&(column, row)) {
                                    let directions_count = directions.len();

                                    if directions_count == 1 {
                                        match directions[0] {
                                            Direction::Up => '^',
                                            Direction::Down => 'v',
                                            Direction::Left => '<',
                                            Direction::Right => '>',
                                        }
                                    } else {
                                        (b'0' + directions_count as u8) as char
                                    }
                                } else {
                                    '.'
                                }
                            }
                        }
                    })
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        );
        debug!(
            "Energized:\n{}",
            (0..input.height())
                .map(|row| {
                    (0..input.width())
                        .map(|column| {
                            if energized_tiles.contains(&(column, row)) {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

fn determine_energized_cells_count(input: &Input, start: DirectedPosition) -> usize {
    let (start_direction, _) = input
        .get(start.1 .0, start.1 .1)
        .unwrap()
        .next_direction(start.0);
    let mut beams = vec![(start_direction, start.1)];
    let mut seen_states = HashSet::new();

    while let Some(mut beam) = beams.pop() {
        loop {
            if seen_states.contains(&beam) {
                break;
            }

            seen_states.insert(beam);

            if let Some((next_beam, new_beam)) = advance_beam(beam, input) {
                beam = next_beam;

                if let Some(new_beam) = new_beam {
                    beams.push(new_beam);
                }
            } else {
                break;
            }
        }
    }

    let energized_tiles = seen_states
        .iter()
        .map(|(_, position)| position)
        .collect::<HashSet<_>>();

    log_grid(input, &energized_tiles, &seen_states);

    energized_tiles.len()
}

fn part_1(input: &Input) -> usize {
    determine_energized_cells_count(input, (Direction::Right, (0, 0)))
}

fn part_2(input: &Input) -> usize {
    (0..input.width())
        .flat_map(|column| {
            [
                (Direction::Down, (column, 0)),
                (Direction::Up, (column, input.height() - 1)),
            ]
        })
        .chain((0..input.height()).flat_map(|row| {
            [
                (Direction::Right, (0, row)),
                (Direction::Left, (input.width() - 1, row)),
            ]
        }))
        .map(|start| determine_energized_cells_count(input, start))
        .max()
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

        let part_2 = log_run("Part 1", || part_2(&input));
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

        assert_eq!(result, 46);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 51);
    }
}
