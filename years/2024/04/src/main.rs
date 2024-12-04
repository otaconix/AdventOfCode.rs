use std::io;

use aoc_timing::trace::log_run;
use grid::Grid;

type Input = Grid<char>;

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

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

fn part_1(input: &Input) -> usize {
    let mut count = 0;

    for row in 0..input.height() {
        let row = input.row(row).collect::<String>();
        for x in 0..=row.len() - 4 {
            let maybe_xmas = &row[x..x + 4];
            if maybe_xmas == XMAS || maybe_xmas == SAMX {
                count += 1;
            }
        }
    }

    for column in 0..input.width() {
        let column = input.column(column).collect::<String>();
        for x in 0..=column.len() - 4 {
            let maybe_xmas = &column[x..x + 4];
            if maybe_xmas == XMAS || maybe_xmas == SAMX {
                count += 1;
            }
        }
    }

    // Down & to the right
    for y in 0..input.height() - 3 {
        for x in 0..input.width() - 3 {
            let maybe_xmas: String = [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]
                .into_iter()
                .map(|(x, y)| input.get(x, y).unwrap())
                .collect();

            if maybe_xmas == XMAS || maybe_xmas == SAMX {
                count += 1;
            }
        }
    }

    // Down & to the left
    for y in 0..input.height() - 3 {
        for x in (3..input.width()).rev() {
            let maybe_xmas: String = [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]
                .into_iter()
                .map(|(x, y)| input.get(x, y).unwrap())
                .collect();

            if maybe_xmas == XMAS || maybe_xmas == SAMX {
                count += 1;
            }
        }
    }

    count
}

fn part_2(input: &Input) -> usize {
    input
        .coordinates()
        .filter(|(x, y)| {
            (1..input.width() - 1).contains(x)
                && (1..input.height() - 1).contains(y)
                && input.get(*x, *y).unwrap() == &'A'
        })
        .filter(|(x, y)| {
            ((input.get(*x - 1, *y - 1).unwrap() == &'M'
                && input.get(x + 1, y + 1).unwrap() == &'S')
                || (input.get(*x - 1, *y - 1).unwrap() == &'S'
                    && input.get(x + 1, y + 1).unwrap() == &'M'))
                && ((input.get(*x - 1, *y + 1).unwrap() == &'M'
                    && input.get(x + 1, y - 1).unwrap() == &'S')
                    || (input.get(*x - 1, *y + 1).unwrap() == &'S'
                        && input.get(x + 1, y - 1).unwrap() == &'M'))
        })
        .count()
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

        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 9);
    }
}
