use std::hash::Hash;
use std::io;
use std::iter::successors;

use aoc_timing::trace::log_run;
use grid::Grid;
use itertools::Itertools;

type ICoords = (isize, isize);
type Input = Grid<char>;
type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    Grid::new(
        input
            .map(|line| {
                let line = line.as_ref();

                line.chars().collect()
            })
            .collect(),
    )
    .expect("Couldn't create grid")
}

fn antinode_coords(antenna_a: &ICoords, antenna_b: &ICoords, map: &Input) -> Option<ICoords> {
    let dx = antenna_b.0 - antenna_a.0;
    let dy = antenna_b.1 - antenna_a.1;

    let (x, y) = (antenna_b.0 + dx, antenna_b.1 + dy);

    if !(0..map.width() as isize).contains(&x) || !(0..map.height() as isize).contains(&y) {
        None
    } else {
        Some((x, y))
    }
}

fn antinode_line(antenna_a: &ICoords, antenna_b: &ICoords, map: &Input) -> Vec<ICoords> {
    let dx = antenna_b.0 - antenna_a.0;
    let dy = antenna_b.1 - antenna_a.1;

    successors(Some(*antenna_b), |(x, y)| {
        let x = x + dx;
        let y = y + dy;

        if !(0..map.width() as isize).contains(&x) || !(0..map.height() as isize).contains(&y) {
            None
        } else {
            Some((x, y))
        }
    })
    .collect()
}

fn solution<K: Clone + Eq + Hash, T: IntoIterator<Item = K>>(
    input: &Input,
    f: fn(&ICoords, &ICoords, &Input) -> T,
) -> usize {
    input
        .coordinates()
        .map(|(column, row)| {
            (
                input.get(column, row).unwrap(),
                (column as isize, row as isize),
            )
        })
        .filter(|(key, _)| **key != '.')
        .into_group_map()
        .values()
        .flat_map(|coord_list| {
            coord_list
                .iter()
                .permutations(2)
                .flat_map(|coords| f(coords[0], coords[1], input))
        })
        .unique()
        .count()
}

fn part_1(input: &Input) -> Output {
    solution(input, antinode_coords)
}

fn part_2(input: &Input) -> Output {
    solution(input, antinode_line)
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

        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 34);
    }
}
