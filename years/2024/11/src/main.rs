use std::collections::HashMap;
use std::io;

use aoc_timing::trace::log_run;

struct Input {
    cache: HashMap<(usize, Stone), usize>,
    stones: Vec<Stone>,
}

type Output = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone {
    number: usize,
}

fn log_n(mut number: usize, n: usize) -> usize {
    let mut result = 0;

    while number > 0 {
        result += 1;
        number /= n;
    }

    if result == 0 { 1 } else { result }
}

impl Stone {
    fn new(number: usize) -> Self {
        Self { number }
    }

    fn split(self) -> Option<(Self, Self)> {
        let digit_count = log_n(self.number, 10);

        if digit_count.is_multiple_of(2) {
            let power_of_ten = (1..digit_count / 2).fold(10, |acc, _| acc * 10);
            let left = self.number / power_of_ten;
            Some((
                Stone::new(left),
                Stone::new(self.number - left * power_of_ten),
            ))
        } else {
            None
        }
    }

    fn step(self) -> Vec<Self> {
        if self.number == 0 {
            vec![Self { number: 1 }]
        } else if let Some((left, right)) = self.split() {
            vec![left, right]
        } else {
            vec![Stone::new(self.number * 2024)]
        }
    }

    fn count_after_steps(
        &self,
        steps_remaining: usize,
        cache: &mut HashMap<(usize, Stone), usize>,
    ) -> usize {
        if steps_remaining == 0 {
            1
        } else if let Some(count) = cache.get(&(steps_remaining, *self)) {
            *count
        } else {
            let count = self
                .step()
                .into_iter()
                .map(|stone| stone.count_after_steps(steps_remaining - 1, cache))
                .sum();
            cache.insert((steps_remaining, *self), count);
            count
        }
    }
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let stones = input
        .flat_map(|line| {
            let line = line.as_ref();

            line.split_whitespace()
                .map(|number| Stone {
                    number: number.parse().unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Input {
        cache: HashMap::new(),
        stones,
    }
}

fn do_part(input: &mut Input, steps: usize) -> Output {
    input
        .stones
        .iter()
        .map(|stone| stone.count_after_steps(steps, &mut input.cache))
        .sum()
}

fn part_1(input: &mut Input) -> Output {
    do_part(input, 25)
}

fn part_2(input: &mut Input) -> Output {
    do_part(input, 75)
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let mut input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&mut input));
        println!("Part 1: {part_1}");

        let part_2 = log_run("Part 2", || part_2(&mut input));
        println!("Part 2: {part_2}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input");

    #[test]
    fn test_digit_count() {
        assert_eq!(log_n(10, 10), 2);
        assert_eq!(log_n(1, 10), 1);
        assert_eq!(log_n(0, 10), 1);
    }

    #[test]
    fn test_part_1() {
        let mut input = parse(INPUT.lines());
        let result = part_1(&mut input);

        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_2() {
        let mut input = parse(INPUT.lines());
        let result = part_2(&mut input);

        assert_eq!(result, 65601038650482);
    }
}
