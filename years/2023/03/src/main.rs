use grid::*;
use std::{collections::HashSet, io};

fn is_symbol(char: &char) -> bool {
    match char {
        &'.' => false,
        _ if char.is_ascii_digit() => false,
        _ => true,
    }
}

fn get_coordinate_neighboring_values(grid: &Grid<char>, (x, y): (usize, usize)) -> Vec<char> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(|(dx, dy)| (x.checked_add_signed(dx), y.checked_add_signed(dy)))
    .filter_map(|(x, y)| match (x, y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    })
    .filter_map(|(x, y)| grid.get(x, y))
    .cloned()
    .collect()
}

fn get_number_range(grid: &Grid<char>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut left = (0..x)
        .rev()
        .take_while(|x| grid.get(*x, y).unwrap().is_ascii_digit())
        .map(|x| (x, y))
        .collect::<Vec<_>>();
    left.reverse();
    let right = (x + 1..grid.width())
        .take_while(|x| grid.get(*x, y).unwrap().is_ascii_digit())
        .map(|x| (x, y));

    left.into_iter()
        .chain(std::iter::once((x, y)))
        .chain(right)
        .collect()
}

fn main() {
    let grid: Grid<char> = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let symbol_adjacent_number_coords = grid
        .coordinates()
        .filter(|(x, y)| {
            let char = grid.get(*x, *y).unwrap();

            char.is_ascii_digit()
                && get_coordinate_neighboring_values(&grid, (*x, *y))
                    .iter()
                    .any(is_symbol)
        })
        .map(|coord| get_number_range(&grid, coord))
        .collect::<HashSet<_>>();

    let part_1 = symbol_adjacent_number_coords
        .iter()
        .map(|coords| {
            coords
                .iter()
                .map(|(x, y)| grid.get(*x, *y).unwrap())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();

    println!("Part 1: {part_1}");
}
