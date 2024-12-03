use std::io;
use std::io::Read;

use aoc_timing::trace::log_run;
use regex::Regex;

fn part_1(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regex
        .captures_iter(input)
        .map(|c| {
            let (_, [left, right]) = c.extract();

            left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    // Not very pretty regex, with useless capture groups to make it easier to work with
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(()()\)|don't\(()()\)").unwrap();
    regex
        .captures_iter(input)
        .fold((true, 0), |(enabled, sum), c| {
            let (full, [left, right]) = c.extract();

            match full {
                "do()" => (true, sum),
                "don't()" => (false, sum),
                _ => {
                    if enabled {
                        (
                            enabled,
                            sum + left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap(),
                        )
                    } else {
                        (enabled, sum)
                    }
                }
            }
        })
        .1
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Couldn't read input");

            input
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
    const INPUT2: &str = include_str!("test-input2");

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT2);

        assert_eq!(result, 48);
    }
}
