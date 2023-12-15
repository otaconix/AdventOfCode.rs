use aoc_timing::trace::log_run;
use std::{
    collections::{HashMap, HashSet},
    io,
    ops::ControlFlow,
};

fn main() {
    env_logger::init();

    let input = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .collect::<Vec<_>>();

    let part_1 = log_run("Part 1", || {
        let letter_counts = input
            .iter()
            .map(|line| {
                line.chars()
                    .fold(HashMap::new(), |mut counts, char| {
                        if let Some(char_count) = counts.get_mut(&char) {
                            *char_count += 1;
                        } else {
                            counts.insert(char, 1);
                        }

                        counts
                    })
                    .into_values()
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        let twos = letter_counts
            .iter()
            .filter(|counts| counts.contains(&2))
            .count();
        let threes = letter_counts
            .iter()
            .filter(|counts| counts.contains(&3))
            .count();

        twos * threes
    });

    println!("Part 1: {part_1:#?}");

    let part_2 = log_run("Part 2", || {
        let correct_boxes = input.iter().enumerate().try_fold((), |_, (index, box_a)| {
            if let Some(box_b) = input.iter().skip(index + 1).find(|box_b| {
                box_a
                    .chars()
                    .zip(box_b.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1
            }) {
                ControlFlow::Break((box_a, box_b))
            } else {
                ControlFlow::Continue(())
            }
        });

        if let ControlFlow::Break((box_a, box_b)) = correct_boxes {
            box_a
                .chars()
                .zip(box_b.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect::<String>()
        } else {
            panic!("No correct boxes found!")
        }
    });

    println!("Part 2: {part_2}");
}
