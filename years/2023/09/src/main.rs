use std::io;

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<Vec<i32>> {
    input
        .map(|line| {
            line.to_string()
                .split_whitespace()
                .map(|number| number.parse().expect("Couldn't parse number"))
                .collect()
        })
        .collect()
}

fn deduce_around(series: &[i32]) -> (i32, i32) {
    if series.iter().all(|n| n == &0) {
        (0, 0)
    } else {
        let last = series[series.len() - 1];
        let first = series[0];

        let subseries = series.windows(2).map(|n| n[1] - n[0]).collect::<Vec<_>>();

        let (sub_prev, sub_next) = deduce_around(&subseries);

        (first - sub_prev, last + sub_next)
    }
}

fn main() {
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let (part_2, part_1) = input
        .iter()
        .map(|series| deduce_around(series))
        .fold((0, 0), |(sum_prev, sum_next), (prev, next)| {
            (sum_prev + prev, sum_next + next)
        });

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
