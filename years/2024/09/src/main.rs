use std::io;

use aoc_timing::trace::log_run;

type Input = Vec<u32>;
type Output = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.chars()
                .map(|c| format!("{c}").parse().unwrap())
                .collect()
        })
        .next()
        .unwrap()
}

fn part_1(input: &Input) -> Output {
    let mut start_index = 0;
    let mut end_index = input.len() - 1;
    let mut result = 0;
    let mut input = input.clone();

    for pos in 0.. {
        if start_index > end_index || start_index >= input.len() {
            break;
        } else if start_index % 2 == 1 {
            // Take from the end
            // println!("end:   {pos} * {}", end_index / 2);
            result += pos * (end_index / 2);
            input[end_index] -= 1;
        } else {
            // Take from the start
            // println!("start: {pos} * {}", start_index / 2);
            result += pos * (start_index / 2);
        }

        input[start_index] -= 1;

        // Skip over zero-sized or exhausted blocks
        while start_index < input.len() && input[start_index] == 0 {
            start_index += 1;
        }

        // Skip over zero-sized or exhausted blocks
        while end_index > 0 && input[end_index] == 0 {
            end_index -= 2;
        }
    }

    result
}

fn part_2(input: &Input) -> Output {
    todo!()
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

        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 0);
    }
}
