use std::io;

use aoc_timing::trace::log_run;
use aoc_utils::PartitionEnumerated;
use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Stuff {
    Ash,
    Rock,
}

#[allow(dead_code)]
enum Split {
    Vertical {
        left: Grid<Stuff>,
        right: Grid<Stuff>,
    },
    Horizontal {
        above: Grid<Stuff>,
        below: Grid<Stuff>,
    },
}

impl Split {
    fn score(&self) -> usize {
        match self {
            Split::Vertical { left, right: _ } => left.height(),
            Split::Horizontal { above, below: _ } => above.height() * 100,
        }
    }
}

// Not exactly the prettiest, but I attempted to deduplicate code to check for both horizontal and
// vertical mirrors
fn split_by_mirror<F>(grid: &Grid<Stuff>, check_fn: F) -> Split
where
    F: Fn(&Vec<Vec<Stuff>>, &Vec<Vec<Stuff>>) -> bool,
{
    (1..grid.height())
        .map(|i| (true, i))
        .chain((1..grid.width()).map(|i| (false, i)))
        .find_map(|(is_horizontal, i)| {
            let (before, after) = (0..if is_horizontal {
                grid.height()
            } else {
                grid.width()
            })
                .map(|i| {
                    if is_horizontal {
                        grid.row(i).copied().collect()
                    } else {
                        grid.column(i).copied().collect()
                    }
                })
                .partition_enumerated::<Vec<_>, _>(|index, _| index < i);

            let to_check = before.len().min(after.len());
            let skip_before = before.len() - to_check;

            if check_fn(
                &before[skip_before..skip_before + to_check].to_vec(),
                &after[0..to_check].iter().rev().cloned().collect(),
            ) {
                Some(if is_horizontal {
                    Split::Horizontal {
                        above: Grid::new(before).unwrap(),
                        below: Grid::new(after).unwrap(),
                    }
                } else {
                    Split::Vertical {
                        left: Grid::new(before).unwrap(),
                        right: Grid::new(after).unwrap(),
                    }
                })
            } else {
                None
            }
        })
        .expect("No mirror found")
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Vec<Grid<Stuff>> {
    input
        .fold(vec![vec![]], |mut acc, line| {
            let line = line.as_ref();

            if line.is_empty() {
                acc.push(vec![]);
            } else {
                let last = acc.len() - 1;
                acc[last].push(
                    line.chars()
                        .map(|stuff| match stuff {
                            '.' => Stuff::Ash,
                            '#' => Stuff::Rock,
                            _ => panic!("Unknown stuff: {stuff}"),
                        })
                        .collect::<Vec<Stuff>>(),
                );
            }

            acc
        })
        .into_iter()
        .map(|grid| Grid::new(grid).unwrap())
        .collect()
}

fn part_1(input: &[Grid<Stuff>]) -> usize {
    input
        .iter()
        .map(|grid| split_by_mirror(grid, |a, b| a.iter().zip(b).all(|(a, b)| a == b)))
        .map(|split| split.score())
        .sum::<usize>()
}

fn part_2(input: &[Grid<Stuff>]) -> usize {
    input
        .iter()
        .map(|grid| {
            split_by_mirror(grid, |a, b| {
                a.iter()
                    .zip(b)
                    .map(|(a, b)| a.iter().zip(b).filter(|(a, b)| a != b).count())
                    .sum::<usize>()
                    == 1
            })
        })
        .map(|split| split.score())
        .sum::<usize>()
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 405);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 400);
    }
}
