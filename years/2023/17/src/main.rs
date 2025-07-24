use std::iter::once;
use std::{io, iter::successors};

use aoc_timing::trace::log_run;
use dijkstra::dijkstra;
use direction::Direction;
use grid::Grid;
use itertools::Itertools;

type Input = Grid<u32>;
type Coord = (usize, usize);

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            line.as_ref()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect()
}

#[allow(unused)] // Only used for debugging
fn print_grid_with_path(input: &Input, path: &[Coord]) {
    println!(
        "{}",
        (0..input.height())
            .map(|row| (0..input.width())
                .map(|column| {
                    if let Some(path_position) =
                        path.iter().position(|(x, y)| x == &column && y == &row)
                    {
                        if path_position == 0 {
                            char::from_u32(input.get(column, row).unwrap() + u32::from(b'0')).unwrap()
                        } else {
                            char::from(
                                Direction::determine(
                                    &path[path_position - 1],
                                    &path[path_position],
                                )
                                .unwrap(),
                            )
                        }
                    } else {
                        char::from_u32(input.get(column, row).unwrap() + u32::from(b'0')).unwrap()
                    }
                })
                .collect::<String>())
            .join("\n")
    );
}

fn part_1(input: &Input) -> u32 {
    let end = (input.width() - 1, input.height() - 1);
    let path = dijkstra(
        ((0, 0), Direction::Down, 0),
        |(coord, _, _)| coord == &end,
        |(coord, direction, count)| {
            [
                direction
                    .advance(coord, 1)
                    .map(|coord| (coord, *direction, count + 1)),
                direction
                    .turn_left()
                    .advance(coord, 1)
                    .map(|coord| (coord, direction.turn_left(), 1)),
                direction
                    .turn_right()
                    .advance(coord, 1)
                    .map(|coord| (coord, direction.turn_right(), 1)),
            ]
            .into_iter()
            .flatten()
            .filter(|((next_column, next_row), _, next_count)| {
                input.is_valid_coord(*next_column, *next_row) && next_count <= &3
            })
            .map(|node @ ((column, row), _, _)| (node, *input.get(column, row).unwrap()))
        },
    )
    .unwrap()
    .into_iter()
    .map(|((coord, _, _), _)| coord)
    .collect_vec();

    // print_grid_with_path(input, &path);

    path.into_iter()
        .skip(1)
        .map(|(column, row)| *input.get(column, row).unwrap())
        .sum()
}

fn intermediate_coords(from: &Coord, to: &Coord) -> Vec<Coord> {
    if let Some(direction) = Direction::determine(from, to) {
        let distance = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
        successors(Some(*from), |coord| direction.advance(coord, 1))
            .take(distance + 1)
            .skip(1)
            .collect()
    } else {
        vec![]
    }
}

fn part_2(input: &Input) -> u32 {
    let end = (input.width() - 1, input.height() - 1);
    let path = once((0, 0))
        .chain(
            dijkstra(
                ((0, 0), Direction::Down, 0),
                |(coord, _, _)| coord == &end,
                |(coord, direction, count)| {
                    [
                        direction
                            .advance_with_intermediate_coords(coord, 1)
                            .map(|coords| (coords, *direction, count + 1)),
                        direction
                            .turn_left()
                            .advance_with_intermediate_coords(coord, 4)
                            .map(|coords| (coords, direction.turn_left(), 4)),
                        direction
                            .turn_right()
                            .advance_with_intermediate_coords(coord, 4)
                            .map(|coords| (coords, direction.turn_right(), 4)),
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|(coords, _, next_count)| {
                        input.is_valid_coord(coords.last().unwrap().0, coords.last().unwrap().1)
                            && next_count <= &10
                    })
                    .map(|(coords, direction, count)| {
                        (
                            (*coords.last().unwrap(), direction, count),
                            coords
                                .into_iter()
                                .map(|(column, row)| *input.get(column, row).unwrap())
                                .sum::<u32>(),
                        )
                    })
                },
            )
            .unwrap()
            .into_iter()
            .map(|((coord, _, _), _)| coord)
            .tuple_windows()
            .flat_map(|(from, to)| intermediate_coords(&from, &to)),
        )
        .collect_vec();

    // print_grid_with_path(input, &path);

    path.into_iter()
        .skip(1)
        .map(|(column, row)| *input.get(column, row).unwrap())
        .sum()
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

        assert_eq!(result, 102);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 94);
    }

    #[test]
    fn test_part_2_small_map() {
        let input = parse(include_str!("test-input-ultra-crucible-small-map").lines());
        let result = part_2(&input);

        assert_eq!(result, 71);
    }
}
