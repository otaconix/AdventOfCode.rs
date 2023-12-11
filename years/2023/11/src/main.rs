use std::io;

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Star,
    Space,
}

const INVALID_CELL: &str = "Invalid cell";

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Star),
            '.' => Ok(Self::Space),
            _ => Err(INVALID_CELL),
        }
    }
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Grid<Cell> {
    input
        .map(|line| {
            line.to_string()
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn sum_of_distances_between_stars<F: Clone + Fn(usize) -> usize>(
    star_coordinates: &[(usize, usize)],
    empty_row_indices: &[usize],
    empty_column_indices: &[usize],
    transform_empty_space: F,
) -> usize {
    star_coordinates
        .iter()
        .map(|(x, y)| {
            (
                x + transform_empty_space(
                    empty_column_indices
                        .iter()
                        .filter(|column_index| column_index < &x)
                        .count(),
                ),
                y + transform_empty_space(
                    empty_row_indices
                        .iter()
                        .filter(|row_index| row_index < &y)
                        .count(),
                ),
            )
        })
        .tuple_combinations::<(_, _)>()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum::<usize>()
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));
    let empty_row_indices = (0..input.height())
        .filter_map(|row_index| {
            if input.row(row_index).all(|cell| matches!(cell, Cell::Space)) {
                row_index.into()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let empty_column_indices = (0..input.width())
        .filter_map(|column_index| {
            if input
                .column(column_index)
                .all(|cell| matches!(cell, Cell::Space))
            {
                column_index.into()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let star_coordinates = input
        .coordinates()
        .filter(|(x, y)| matches!(input.get(*x, *y).unwrap(), Cell::Star))
        .collect::<Vec<_>>();

    let part_1 = sum_of_distances_between_stars(
        &star_coordinates,
        &empty_row_indices,
        &empty_column_indices,
        |count| count,
    );

    println!("Part 1: {part_1}");

    let part_2 = sum_of_distances_between_stars(
        &star_coordinates,
        &empty_row_indices,
        &empty_column_indices,
        |count| count * 999_999,
    );

    println!("Part 2: {part_2}");
}
