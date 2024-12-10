use std::io;

use aoc_timing::trace::log_run;

type Input = Vec<BlockSequence>;
type Output = usize;

#[derive(Debug, Clone, Copy)]
struct BlockSequence {
    is_file: bool,
    block_count: usize,
    original_block_count: usize,
    index: usize,
}

fn parse<S: AsRef<str>, I: Iterator<Item = S>>(input: I) -> Input {
    input
        .map(|line| {
            let line = line.as_ref();

            line.chars()
                .enumerate()
                .map(|(index, c)| {
                    let block_count = c.to_digit(10).unwrap() as usize;

                    BlockSequence {
                        is_file: index % 2 == 0,
                        block_count,
                        original_block_count: block_count,
                        index: index / 2,
                    }
                })
                .collect()
        })
        .next()
        .unwrap()
}

trait Helper {
    fn last_file_index(&self) -> usize;
}

impl Helper for Vec<BlockSequence> {
    // Make sure that, if the last element in the input is empty space, we don't use it as though
    // it were a file.
    fn last_file_index(&self) -> usize {
        (self.len() - 1) & !1
    }
}

fn part_1(input: &Input) -> Output {
    let mut end_index = input.last_file_index();
    let mut result = 0;
    let mut block_position = 0;
    let mut sequences = input.clone();

    for start_index in 0..input.len() {
        if start_index > end_index {
            break;
        }

        let current_sequence = sequences[start_index];

        if current_sequence.is_file {
            result += current_sequence.index
                * (block_position..)
                    .take(current_sequence.block_count)
                    .sum::<usize>();
            block_position += current_sequence.block_count;
        } else {
            let mut empty_block_count = current_sequence.block_count;

            while start_index < end_index && empty_block_count > 0 {
                let file = &mut sequences[end_index];
                let moved_block_count = if empty_block_count > file.block_count {
                    file.block_count
                } else {
                    empty_block_count
                };

                file.block_count -= moved_block_count;
                empty_block_count -= moved_block_count;

                result += file.index * (block_position..).take(moved_block_count).sum::<usize>();
                block_position += moved_block_count;

                end_index = sequences[start_index..=end_index]
                    .iter()
                    .rev()
                    .find_map(|next_file| {
                        if next_file.is_file && next_file.block_count > 0 {
                            Some(next_file.index * 2)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);
            }
        }
    }

    result
}

#[allow(dead_code)]
fn part_2_cleaner_but_slower(input: &Input) -> Output {
    let mut sequences: Vec<_> = input.clone();
    let end_index = input.last_file_index();

    for index in (1..=end_index).rev() {
        let file = sequences[index];

        if !file.is_file {
            continue;
        }

        for empty_index in 0..index {
            let empty = &mut sequences[empty_index];
            if !empty.is_file && empty.block_count >= file.block_count {
                empty.block_count -= file.block_count;

                sequences[index] = BlockSequence {
                    is_file: false,
                    ..file
                };
                sequences.insert(empty_index, file);
                break;
            }
        }
    }

    let mut result = 0;
    let mut block_position = 0;
    for sequence in sequences {
        if sequence.is_file {
            result += sequence.index * (block_position..).take(sequence.block_count).sum::<usize>();
        }

        block_position += sequence.block_count;
    }

    result
}

fn part_2(input: &Input) -> Output {
    let mut end_index = input.last_file_index();
    let mut result = 0;
    let mut block_position = 0;
    let mut sequences = input.clone();

    for start_index in 0..sequences.len() {
        // Let's find the first file from the end that still has to be moved
        // This is an optimalization: if the file has already been moved, its block_count is set to
        // 0, after which there's no point checking if it should be moved again.
        end_index = (start_index + 1..=end_index)
            .rev()
            .find(|index| {
                let next_file = sequences[*index];

                next_file.is_file && next_file.block_count > 0
            })
            .unwrap_or(0);

        let sequence = &mut sequences[start_index];

        if sequence.is_file {
            if sequence.block_count == 0 {
                // There is now a hole where a file used to be. Increase block_position by the
                // original block count.
                block_position += sequence.original_block_count;
            } else {
                // Unmoved original file
                result +=
                    sequence.index * (block_position..).take(sequence.block_count).sum::<usize>();
                block_position += sequence.block_count;
                sequence.block_count = 0;
            }
        } else {
            let mut empty_block_count = sequence.block_count;
            // We need a copy of the last index to check for files that can be moved, since we may
            // have to skip over a bunch of them, and still check them for later empty sequences.
            let mut loop_end_index = end_index;
            while start_index < loop_end_index && empty_block_count > 0 {
                if let Some(file_to_move) = sequences[start_index + 1..=loop_end_index]
                    .iter_mut()
                    .rev()
                    .find(|file_to_move| {
                        file_to_move.is_file
                            && (1..=empty_block_count).contains(&file_to_move.block_count)
                    })
                {
                    result += file_to_move.index
                        * (block_position..)
                            .take(file_to_move.block_count)
                            .sum::<usize>();
                    empty_block_count -= file_to_move.block_count;
                    block_position += file_to_move.block_count;
                    file_to_move.block_count = 0;

                    loop_end_index = (start_index + 1..loop_end_index)
                        .rev()
                        .find(|index| {
                            let next_file = sequences[*index];

                            next_file.is_file && next_file.block_count > 0
                        })
                        .unwrap_or(0);
                } else {
                    break;
                }
            }

            if empty_block_count != 0 {
                block_position += empty_block_count;
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
    fn test_part_2_cleaner_but_slower() {
        let input = parse(INPUT.lines());
        let result = part_2_cleaner_but_slower(&input);

        assert_eq!(result, 2858);
    }

    #[test]
    fn test_part_2() {
        let input = parse(INPUT.lines());
        let result = part_2(&input);

        assert_eq!(result, 2858);
    }
}
