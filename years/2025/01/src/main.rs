use std::io;

use aoc_timing::trace::log_run;

type Input = Vec<Turn>;
type Output1 = usize;
type Output2 = Output1;

enum Turn {
    Left(u64),
    Right(u64),
}

impl Turn {
    fn to_number_to_add(&self) -> i32 {
        match self {
            Turn::Left(n) => -(*n as i32),
            Turn::Right(n) => *n as i32,
        }
    }

    fn to_clicks(&self) -> impl Iterator<Item = i32> {
        let (click, n) = match self {
            Turn::Left(n) => (-1, n),
            Turn::Right(n) => (1, n),
        };

        (0..(*n as i32).abs()).map(move |_| click)
    }
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();
            let (direction, count) = line.split_at(1);
            let direction = direction.chars().next().unwrap();
            let count = count.parse().expect("Invalid number of turns");

            match direction {
                'L' => Turn::Left(count),
                'R' => Turn::Right(count),
                _ => panic!("Unknown turn direction"),
            }
        })
        .collect()
}

fn part_1(input: &Input) -> Output1 {
    input
        .iter()
        .scan(50, |current_position, turn| {
            *current_position += turn.to_number_to_add();
            *current_position = current_position.rem_euclid(100);
            Some(*current_position)
        })
        .filter(|&position| position == 0)
        .count()
}

fn part_2(input: &Input) -> Output2 {
    input
        .iter()
        .flat_map(|turn| turn.to_clicks())
        .scan(50, |current_position, click| {
            *current_position += click;
            *current_position = current_position.rem_euclid(100);
            Some(*current_position)
        })
        .filter(|&position| position == 0)
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

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 6);
    }
}
