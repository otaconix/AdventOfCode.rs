use std::io;

use aoc_timing::debug::log_run;

type Input = Vec<BlockSequence>;
type Output = usize;

#[derive(Debug, Clone, Copy)]
struct File {
    index: usize,
    block_count: usize,
}

#[derive(Debug, Clone, Copy)]
enum BlockSequence {
    Empty(usize),
    File(File),
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.chars()
                .enumerate()
                .map(|(index, c)| {
                    let block_count = format!("{c}").parse::<usize>().unwrap();

                    if index % 2 == 0 {
                        BlockSequence::File(File {
                            block_count,
                            index: index / 2,
                        })
                    } else {
                        BlockSequence::Empty(block_count)
                    }
                })
                .collect()
        })
        .next()
        .unwrap()
}

fn part_1(input: &Input) -> Output {
    // Make sure that, if the last element in the input is empty space, we don't use it as though
    // it were a file.
    let mut end_index = (input.len() - 1) & !1;
    let mut result = 0;
    let mut block_position = 0;
    let mut input = input.clone();

    for start_index in 0..input.len() {
        if start_index > end_index {
            break;
        }

        match input[start_index] {
            BlockSequence::Empty(mut empty_block_count) => {
                while start_index < end_index && empty_block_count > 0 {
                    if let BlockSequence::File(ref mut file) = input[end_index] {
                        let moved_block_count = if empty_block_count > file.block_count {
                            file.block_count
                        } else {
                            empty_block_count
                        };

                        file.block_count -= moved_block_count;
                        empty_block_count -= moved_block_count;

                        result +=
                            file.index * (block_position..).take(moved_block_count).sum::<usize>();
                        block_position += moved_block_count;

                        while end_index > 0
                            && match input[end_index] {
                                BlockSequence::File(file) if file.block_count == 0 => true,
                                BlockSequence::Empty(_) => true,
                                _ => false,
                            }
                        {
                            end_index -= 1;
                        }
                    }
                }
            }
            BlockSequence::File(ref mut file) => {
                result += file.index * (block_position..).take(file.block_count).sum::<usize>();
                block_position += file.block_count;
                file.block_count = 0;
            }
        }
    }

    result
}

fn part_2(input: &Input) -> Output {
    let mut sequences: Vec<_> = input.clone();
    let end_index = (sequences.len() - 1) & !1;

    for index in (1..=end_index).rev() {
        let file = if let BlockSequence::File(file) = sequences[index] {
            file
        } else {
            continue;
        };

        for empty_index in 0..index {
            match sequences[empty_index] {
                BlockSequence::Empty(ref mut empty_block_count)
                    if *empty_block_count >= file.block_count =>
                {
                    *empty_block_count -= file.block_count;

                    sequences[index] = BlockSequence::Empty(file.block_count);
                    sequences.insert(empty_index, BlockSequence::File(file));

                    break;
                }
                _ => {}
            }
        }
    }

    let mut result = 0;
    let mut block_position = 0;
    for sequence in sequences {
        match sequence {
            BlockSequence::Empty(block_count) => block_position += block_count,
            BlockSequence::File(file) => {
                result += file.index * (block_position..).take(file.block_count).sum::<usize>();

                block_position += file.block_count;
            }
        }
    }

    result
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

        assert_eq!(result, 2858);
    }
}
