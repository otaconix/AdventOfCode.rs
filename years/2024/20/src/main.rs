use std::collections::HashSet;
use std::io;
use std::iter::successors;

use aoc_timing::trace::log_run;

type Coord = (usize, usize);
type Input = Vec<Coord>;
type Output1 = usize;
type Output2 = Output1;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (road, start_position) = input.enumerate().fold(
        (HashSet::new(), None),
        |(mut road, mut start_position), (row, line)| {
            let line = line.as_ref();

            for (column, char) in line.chars().enumerate() {
                match char {
                    '#' => {}
                    '.' | 'E' => {
                        road.insert((column, row));
                    }
                    'S' => {
                        road.insert((column, row));
                        start_position = Some((column, row));
                    }
                    _ => panic!("Unexpected character in map: {char}"),
                }
            }

            (road, start_position)
        },
    );

    let start_position = start_position.unwrap();

    successors(
        Some((None as Option<Coord>, start_position)),
        |(prev, current @ (column, row))| {
            let mut nexts = vec![(column + 1, *row), (*column, row + 1)];
            if column > &0 {
                nexts.push((column - 1, *row));
            }
            if row > &0 {
                nexts.push((*column, row - 1));
            }

            nexts
                .into_iter()
                .find(|next| prev != &Some(*next) && road.contains(next))
                .map(|next| (Some(*current), next))
        },
    )
    .map(|(_, coord)| coord)
    .collect()
}

fn manhattan_distance(from: &Coord, to: &Coord) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

fn count_cheats(road: &[Coord], cheat_picoseconds: usize, minimal_savings: usize) -> usize {
    road.iter()
        .enumerate()
        .map(|(picoseconds, position)| {
            road[picoseconds + 1..]
                .iter()
                .enumerate()
                .filter(|(delta_picoseconds, next)| {
                    manhattan_distance(position, next) <= cheat_picoseconds
                        && delta_picoseconds >= &minimal_savings
                })
                .count()
        })
        .sum()
}

fn part_1(input: &Input, minimal_savings: usize) -> Output1 {
    count_cheats(input, 2, minimal_savings)
}

fn part_2(input: &Input, minimal_savings: usize) -> Output2 {
    count_cheats(input, 20, minimal_savings)
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input, 100));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&input, 100));
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
        let result = part_1(&input, 2);

        assert_eq!(result, 44);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input, 50);

        assert_eq!(result, 285);
    }

    #[test]
    fn manhattan_distance() {
        assert_eq!(super::manhattan_distance(&(0, 0), &(20, 20)), 40);
    }
}
