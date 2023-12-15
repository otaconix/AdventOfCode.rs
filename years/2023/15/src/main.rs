use std::io;

use aoc_timing::info;

fn parse<S: ToString, I: Iterator<Item = S>>(input: I) -> Vec<String> {
    input
        .map(|line| line.to_string())
        .flat_map(|line| {
            line.split(',')
                .map(|step| step.to_owned())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn hash(step: &str) -> u8 {
    step.chars()
        .map(|c| c as u8)
        .fold(0, |hash, ascii| (hash.wrapping_add(ascii)).wrapping_mul(17))
}

fn part_1(input: &[String]) -> usize {
    input.iter().map(|step| hash(step) as usize).sum::<usize>()
}

fn part_2(input: &[String]) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = (0..=u8::MAX).map(|_| Vec::new()).collect();

    input.iter().for_each(|step| {
        let (label, focal_length) = step
            .split_once('-')
            .or_else(|| step.split_once('='))
            .unwrap();
        let label_hash = hash(label) as usize;
        let operation = step.chars().find(|c| c == &'-' || c == &'=').unwrap();

        if operation == '=' {
            let focal_length: usize = focal_length.parse().expect("Invalid focal length");

            if let Some((_, existing_focal_length)) =
                boxes[label_hash].iter_mut().find(|(l, _)| l == &label)
            {
                *existing_focal_length = focal_length;
            } else {
                boxes[label_hash].push((label, focal_length));
            }
        } else {
            boxes[label_hash].retain_mut(|(l, _)| l != &label);
        }
    });

    (1usize..)
        .zip(boxes)
        .flat_map(move |(box_number, r#box)| {
            // `box` is a keyword, apparently
            (1usize..)
                .zip(r#box)
                .map(move |(slot, (_, focal_length))| box_number * slot * focal_length)
        })
        .sum()
}

fn main() {
    env_logger::init();
    let input = parse(io::stdin().lines().map(|result| result.expect("I/O error")));

    let part_1 = info::log_run("Part 1", || part_1(&input));
    println!("Part 1: {part_1}");

    let part_2 = info::log_run("Part 2", || part_2(&input));
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_part_1() {
        let input = parse(INPUT.lines());
        let result = part_1(&input);

        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 145);
    }
}
