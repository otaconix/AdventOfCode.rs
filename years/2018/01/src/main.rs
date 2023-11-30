use std::collections::HashSet;
use std::io;
use std::ops::ControlFlow;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse::<i32>().expect("Couldn't parse number"))
        .collect::<Vec<_>>();

    let part_1 = input.iter().sum::<i32>();
    println!("Part 1: {part_1}");

    let part_2 = match input
        .iter()
        .cycle()
        .scan(0, |acc, change| {
            *acc += change;

            Some(*acc)
        })
        .try_fold(HashSet::new(), |mut already_seen, next| {
            if !already_seen.contains(&next) {
                already_seen.insert(next);
                ControlFlow::Continue(already_seen)
            } else {
                ControlFlow::Break(next)
            }
        }) {
        ControlFlow::Break(result) => result,
        _ => panic!("Couldn't find a result!"),
    };

    println!("Part 2: {part_2:#?}");
}
