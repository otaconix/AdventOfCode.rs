use grid::*;
use std::collections::HashSet;
use std::io;

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
    let left = (0..x)
        .rev()
        .take_while(|x| grid.get(*x, y).unwrap().is_ascii_digit())
        .map(|x| (x, y))
        .collect::<Vec<_>>();
    let right = (x + 1..grid.width())
        .take_while(|x| grid.get(*x, y).unwrap().is_ascii_digit())
        .map(|x| (x, y));

    left.into_iter()
        .rev()
        .chain(std::iter::once((x, y)))
        .chain(right)
        .collect()
}

fn parse_number_range(grid: &Grid<char>, coord_range: &[(usize, usize)]) -> u32 {
    coord_range
        .iter()
        .map(|(x, y)| grid.get(*x, *y).unwrap())
        .collect::<String>()
        .parse::<u32>()
        .expect("Couldn't parse number")
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Grid<char> {
    input
        .map(|input| input.to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn part_1(input: &Grid<char>) -> u32 {
    let symbol_adjacent_number_coords = input
        .coordinates()
        .filter(|(x, y)| {
            let char = input.get(*x, *y).unwrap();

            char.is_ascii_digit()
                && get_coordinate_neighboring_values(input, (*x, *y))
                    .iter()
                    .any(is_symbol)
        })
        .map(|coord| get_number_range(input, coord))
        .collect::<HashSet<_>>();

    symbol_adjacent_number_coords
        .iter()
        .map(|coords| parse_number_range(input, coords))
        .sum()
}

fn part_2(grid: &Grid<char>) -> u32 {
    let potential_gears = grid
        .coordinates()
        .filter(|(x, y)| {
            let char = grid.get(*x, *y).unwrap();

            char == &'*'
        })
        .collect::<Vec<_>>();

    let gear_adjacent_number_coords = grid
        .coordinates()
        .filter(|(x, y)| {
            let char = grid.get(*x, *y).unwrap();

            char.is_ascii_digit()
                && potential_gears
                    .iter()
                    .any(|(gx, gy)| x.abs_diff(*gx) <= 1 && y.abs_diff(*gy) <= 1)
        })
        .map(|coord| get_number_range(grid, coord))
        .collect::<HashSet<_>>();

    potential_gears
        .iter()
        .filter_map(|(gx, gy)| {
            let adjacent_number_coords = gear_adjacent_number_coords
                .iter()
                .filter(|coords| {
                    coords
                        .iter()
                        .any(|(x, y)| x.abs_diff(*gx) <= 1 && y.abs_diff(*gy) <= 1)
                })
                .collect::<Vec<_>>();

            if adjacent_number_coords.len() == 2 {
                Some(
                    adjacent_number_coords
                        .iter()
                        .map(|coords| parse_number_range(grid, coords))
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let grid: Grid<char> = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = part_1(&grid);

    println!("Part 1: {part_1}");

    let part_2 = part_2(&grid);

    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_1() {
        let input = parse(&mut INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_2() {
        let input = parse(&mut INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 467835);
    }
}
