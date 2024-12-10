use std::collections::HashSet;
use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;
use itertools::Either;
use itertools::Itertools;

type Input = Grid<Output>;
type Output = usize;
type Coord = (usize, usize);

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    Grid::new(
        input
            .map(|line| {
                let line = line.as_ref();

                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    )
    .unwrap()
}

fn find_trails(
    input: &Input,
    current_trails: Vec<Vec<Coord>>,
    mut trails: HashSet<Vec<Coord>>,
) -> HashSet<Vec<Coord>> {
    if current_trails.is_empty() {
        trails
    } else {
        let (next_trails, completed_trails): (Vec<Vec<_>>, Vec<_>) =
            current_trails.into_iter().partition_map(|current_trail| {
                let (column, row) = current_trail.last().unwrap();
                let current_height = *input.get(*column, *row).unwrap();

                if current_height == 9 {
                    Either::Right(current_trail)
                } else {
                    Either::Left(
                        input
                            .get_neighbors(*column, *row)
                            .into_iter()
                            .filter(|(next_column, next_row)| {
                                *input.get(*next_column, *next_row).unwrap() == current_height + 1
                            })
                            .map(|next_position| {
                                let mut next_trail = current_trail.clone();
                                next_trail.push(next_position);
                                next_trail
                            })
                            .collect(),
                    )
                }
            });

        trails.extend(completed_trails);
        find_trails(input, next_trails.into_iter().flatten().collect(), trails)
    }
}

fn part_1(input: &Input) -> Output {
    let trail_starts: Vec<_> = input
        .coordinates()
        .filter(|(column, row)| input.get(*column, *row).unwrap() == &0)
        .map(|start| vec![start])
        .collect();

    let trails = find_trails(input, trail_starts, HashSet::new());

    trails
        .into_iter()
        .into_group_map_by(|trail| trail[0])
        .values()
        .map(|trails| trails.iter().unique_by(|trail| trail.last()).count())
        .sum()
}

fn part_2(input: &Input) -> Output {
    let trail_starts: Vec<_> = input
        .coordinates()
        .filter(|(column, row)| input.get(*column, *row).unwrap() == &0)
        .map(|start| vec![start])
        .collect();

    let trails = find_trails(input, trail_starts, HashSet::new());

    trails.len()
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

        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 81);
    }
}
