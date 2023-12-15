use aoc_timing::trace::log_run;
use std::io;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn winners(&self) -> usize {
        let discriminant = self.time * self.time - 4 * self.distance;
        let root = (discriminant as f64).sqrt();
        let left = (self.time as f64) - root;
        let left = left / 2.0;
        let left = left.ceil();
        let right = (self.time as f64) + root;
        let right = right / 2.0;
        let right = right.floor() + 1.0;

        (right - left) as usize
    }
}

fn concatenate_integers(a: usize, b: usize) -> usize {
    let mut multiplier = 1;

    loop {
        multiplier *= 10;

        if multiplier > b {
            break;
        };
    }

    a * multiplier + b
}

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<Race> {
    let number_lines = input
        .map(|line| {
            line.to_string()
                .split_whitespace()
                .skip(1)
                .map(|number| number.parse().expect("Couldn't parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    number_lines[0]
        .iter()
        .zip(number_lines[1].iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

fn part_1(input: &[Race]) -> usize {
    input.iter().map(Race::winners).product()
}

fn part_2(input: &[Race]) -> usize {
    let (part_2_time, part_2_distance) = input
        .iter()
        .map(|race| (race.time, race.distance))
        .reduce(|(time, distance), (race_time, race_distance)| {
            (
                concatenate_integers(time, race_time),
                concatenate_integers(distance, race_distance),
            )
        })
        .unwrap();

    Race {
        time: part_2_time,
        distance: part_2_distance,
    }
    .winners()
}

fn main() {
    env_logger::init();

    let (part_1, part_2) = log_run("Full run", || {
        let input: Vec<Race> = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        let part_2 = log_run("Part 2", || part_2(&input));

        (part_1, part_2)
    });

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 71503);
    }
}
