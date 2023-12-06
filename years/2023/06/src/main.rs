use std::io;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn winners(&self) -> usize {
        (1..self.time)
            .filter(|speed| (self.time - speed) * speed > self.distance)
            .count()
    }
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

    println!("Part 1: {part_1:?}");
}
