use std::io;

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

fn split_by_mirror(grid: &Grid<Stuff>) -> Split {
    for row in 1..grid.height() {
        let above = (0..row)
            .map(|i| grid.row(i).copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let below = (row..grid.height())
            .map(|i| grid.row(i).copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let to_check = above.len().min(below.len());
        let skip_above = above.len() - to_check;

        if above[skip_above..skip_above + to_check]
            .iter()
            .zip(below[0..to_check].iter().rev())
            .all(|(a, b)| a == b)
        {
            return Split::Horizontal {
                above: Grid::new(above).unwrap(),
                below: Grid::new(below).unwrap(),
            };
        }
    }

    for column in 1..grid.width() {
        let left = (0..column)
            .map(|i| grid.column(i).copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let right = (column..grid.width())
            .map(|i| grid.column(i).copied().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let to_check = left.len().min(right.len());
        let skip_left = left.len() - to_check;

        if left[skip_left..skip_left + to_check]
            .iter()
            .zip(right[0..to_check].iter().rev())
            .all(|(a, b)| a == b)
        {
            return Split::Vertical {
                left: Grid::new(left).unwrap(),
                right: Grid::new(right).unwrap(),
            };
        }
    }

    panic!("No mirror found")
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<Grid<Stuff>> {
    input
        .map(|line| line.to_string())
        .fold(vec![vec![]], |mut acc, line| {
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
                )
            }

            acc
        })
        .into_iter()
        .map(|grid| Grid::new(grid).unwrap())
        .collect()
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = input
        .iter()
        .map(split_by_mirror)
        .map(|split| match split {
            Split::Vertical { left, right: _ } => left.height(),
            Split::Horizontal { above, below: _ } => above.height() * 100,
        })
        // .inspect(|n| println!("{n}"))
        .sum::<usize>();

    println!("Part 1: {part_1}");
}
