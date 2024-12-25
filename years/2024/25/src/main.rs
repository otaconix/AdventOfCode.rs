use std::io;

use aoc_timing::trace::log_run;

type Shape = [u8; 5];
#[derive(Debug, Default)]
struct Input {
    keys: Vec<Shape>,
    locks: Vec<Shape>,
}
type Output1 = usize;

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    let (mut input, shape, is_lock, start_of_shape) = input.fold(
        (Input::default(), [0; 5], true, true),
        |(mut input, mut shape, mut is_lock, start_of_shape), line| {
            let line = line.as_ref();

            if line.is_empty() {
                if is_lock {
                    input.locks.push(shape);
                } else {
                    input.keys.push(shape.map(|n| n - 1));
                }

                (input, [0; 5], true, true)
            } else {
                if start_of_shape {
                    is_lock = line.chars().all(|c| c == '#');
                } else {
                    line.char_indices().for_each(|(index, c)| {
                        if c == '#' {
                            shape[index] += 1;
                        }
                    });
                }

                (input, shape, is_lock, false)
            }
        },
    );

    if !start_of_shape {
        if is_lock {
            input.locks.push(shape);
        } else {
            input.keys.push(shape.map(|n| n - 1));
        }
    }

    input
}

fn lock_and_key_fit(lock: &[u8; 5], key: &[u8; 5]) -> bool {
    lock.iter().zip(key).all(|(lock, key)| lock + key <= 5)
}

fn part_1(input: &Input) -> Output1 {
    input
        .keys
        .iter()
        .map(|key| {
            input
                .locks
                .iter()
                .filter(|lock| lock_and_key_fit(lock, key))
                .count()
        })
        .sum()
}

fn main() {
    env_logger::init();

    log_run("Full run", || {
        let input = log_run("Parsing", || {
            parse(io::stdin().lines().map(|result| result.expect("I/O error")))
        });

        let part_1 = log_run("Part 1", || part_1(&input));
        println!("Part 1: {part_1}");
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

        assert_eq!(result, 3);
    }
}
