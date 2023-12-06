use std::io;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winners(&self) -> usize {
        (1..self.time)
            .filter(|speed| (self.time - speed) * speed > self.distance)
            .count()
    }
}

fn concatenate_integers(a: u64, b: u64) -> u64 {
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
    let input: Vec<Race> = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = part_1(&input);

    println!("Part 1: {part_1}");

    let part_2 = part_2(&input);

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
