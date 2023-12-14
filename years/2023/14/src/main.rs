use std::io;

use grid::Grid;

#[derive(Debug, PartialEq)]
enum Rock {
    Round,
    Cube,
    None,
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Grid<Rock> {
    input
        .map(|line| {
            line.to_string()
                .chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::None,
                    _ => panic!("Unknown rock type {c}"),
                })
                .collect()
        })
        .collect()
}

fn part_1(input: &Grid<Rock>) -> usize {
    (0..input.width())
        .map(|column| {
            input
                .column(column)
                .enumerate()
                .map(|(row, rock)| {
                    if row == 0 && rock != &Rock::Cube {
                        // println!("First row in column {column} is empty");
                        input
                            .column(column)
                            .take_while(|rock| rock != &&Rock::Cube)
                            .filter(|rock| rock == &&Rock::Round)
                            .enumerate()
                            .map(|(row, _)| input.height() - row)
                            // .inspect(|num| println!("{num}"))
                            .sum()
                    } else if rock == &Rock::Cube {
                        input
                            .column(column)
                            .skip(row + 1)
                            .take_while(|rock| rock != &&Rock::Cube)
                            .filter(|rock| rock == &&Rock::Round)
                            .enumerate()
                            // .inspect(|(sub_row, _)| {
                            //     println!("Rock below {column},{row}: {column},{}", row + sub_row)
                            // })
                            .map(|(sub_row, _)| input.height() - (row + sub_row + 1))
                            // .inspect(|n| println!("{n}"))
                            .sum()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = part_1(&input);
    println!("Part 1: {part_1}");
}
