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

fn main() {
    let number_lines = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|number| number.parse().expect("Couldn't parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let input: Vec<Race> = number_lines[0]
        .iter()
        .zip(number_lines[1].iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();

    let part_1: usize = input.iter().map(Race::winners).product();

    println!("Part 1: {part_1}");

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

    let part_2 = Race {
        time: part_2_time,
        distance: part_2_distance,
    }
    .winners();

    println!("Part 2: {part_2}");
}
